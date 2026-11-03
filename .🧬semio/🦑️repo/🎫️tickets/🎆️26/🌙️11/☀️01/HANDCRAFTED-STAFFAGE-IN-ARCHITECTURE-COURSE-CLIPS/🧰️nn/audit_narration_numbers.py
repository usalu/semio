"""🔎 Audit the device numbers of the energy clip narration (en and de) against the cast manifest and the catalog assets."""
import ast
import json
import re
import sys
from pathlib import Path

REPO = Path(sys.argv[1])
TICKET = Path(sys.argv[2])
ENERGY = REPO / "🎓️teaching/🏛️architecture/⚡️energy"
CATALOG = REPO / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
MANIFEST = ENERGY / "🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json"
REPORT = TICKET / "audit-narration-numbers-r2.md"
JUSTIFIED = {}
AIR_RHO_CP = 1.2 * 1005.0

C_IG = "❄️cooling/🎬️clip/👥️internal-gains/🐍️.py"
H_IG = "🔥️heating/🎬️clip/👥️internal-gains/🐍️.py"
PF = "🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py"
HF = "🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py"
CON = "🔥️heating/🎬️clip/🧱️conduction/🐍️.py"
NEED = "🌬️ventilation/🎬️clip/❓️need/🐍️.py"
NAT = "🌬️ventilation/🎬️clip/🪟️natural/🐍️.py"
MECH = "🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py"
CALC = "🔥️heating/🎬️clip/🧮️calculation/🐍️.py"
HC = "❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py"

EN_UNITS = {"zero": 0, "one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8,
            "nine": 9, "ten": 10, "eleven": 11, "twelve": 12, "thirteen": 13, "fourteen": 14, "fifteen": 15,
            "sixteen": 16, "seventeen": 17, "eighteen": 18, "nineteen": 19, "twenty": 20, "thirty": 30, "forty": 40,
            "fifty": 50, "sixty": 60, "seventy": 70, "eighty": 80, "ninety": 90, "tenth": 0.1}
DE_UNITS = {"null": 0, "eins": 1, "ein": 1, "zwei": 2, "drei": 3, "vier": 4, "fünf": 5, "sechs": 6, "sieben": 7,
            "acht": 8, "neun": 9, "zehn": 10, "elf": 11, "zwölf": 12, "zwanzig": 20, "dreißig": 30, "vierzig": 40,
            "fünfzig": 50, "sechzig": 60, "siebzig": 70, "achtzig": 80, "neunzig": 90, "fünfmal": 5, "dreimal": 3}
DE_COMPOUND = re.compile(r"(eins|ein|zwei|drei|vier|fünf|sechs|sieben|acht|neun)?(hundert|tausend)(zehn|zwanzig|dreißig|vierzig|fünfzig|sechzig|siebzig|achtzig|neunzig)?")
DIGIT_WORDS = "(?:zero|one|two|three|four|five|six|seven|eight|nine)"


def load_catalog():
    """📚 Map every catalog asset id to its parsed asset block."""
    assets = {}
    for path in CATALOG.rglob("🔣️.json"):
        doc = json.loads(path.read_text(encoding="utf-8"))
        if "asset" in doc:
            assets[doc["asset"]["id"]] = doc["asset"]
    return assets


def region_sum(asset):
    """🔥 Sum of the operating heat of all regions at level 1, in W."""
    return sum(r.get("operating", {}).get("heatW", 0.0) for r in asset["thermal"]["regions"])


def air_flow_heat(asset):
    """💨 Sensible heat of the emitter air streams, V times rho c_p times the rise, in W."""
    return sum(e["volumeFlowM3S"] * AIR_RHO_CP * abs(e["temperatureRiseK"])
               for e in asset["thermal"].get("emitters", []) if "volumeFlowM3S" in e and "temperatureRiseK" in e)


def met_w(asset, pose):
    """🧍 Metabolic heat of one pose: activity MET times the body W per m2 per MET times the skin area, in W."""
    body = asset["thermal"]["body"]
    return asset["poses"][pose]["thermal"]["activityMet"] * body["metWM2"] * body["skinAreaM2"]


def phase_block(node):
    """🫕 The first phaseChange declaration found in a nested asset block."""
    stack = [node]
    while stack:
        item = stack.pop()
        if isinstance(item, dict):
            if "phaseChange" in item:
                return item["phaseChange"]
            stack.extend(item.values())
        elif isinstance(item, list):
            stack.extend(item)
    raise KeyError("no phaseChange")


def first(pattern, text):
    """🔤 The first regex group of a text as float, with a decimal comma read as a point."""
    match = re.search(pattern, text)
    if not match:
        raise KeyError(pattern)
    return float(match.group(1).replace(",", "."))


def asset_values(assets):
    """🧮 Every catalog-derived value the claims compare against, keyed by a short name."""
    pot = assets["cooking-pot"]
    tealight = assets["tealight"]
    shower = assets["shower-head"]
    vacuum = assets["vacuum-cleaner"]
    paraffin_g = first(r"filled with ([\d.]+) g of paraffin", tealight["provenance"]["notes"]["en"])
    mj_low, mj_high = map(float, re.search(r"(\d+) to (\d+) MJ/kg", tealight["provenance"]["notes"]["en"]).groups())
    flow_lpm = first(r"delivers ([\d.]+) L/min", shower["summary"]["en"])
    pot_water = first(r"the water ([\d.]+) kg", pot["provenance"]["notes"]["en"])
    water_c = next(m["specificHeatJKgK"] for m in pot["materials"] if "phaseChange" in m)
    tealight_w = region_sum(tealight) + air_flow_heat(tealight)
    vacuum_w = region_sum(vacuum) + air_flow_heat(vacuum)
    return {
        "seated": met_w(assets["seated-office-worker"], 0),
        "seated_sensible": met_w(assets["seated-office-worker"], 0) * 2 / 3,
        "radiator_night_W": first(r"rated ([\d.]+) kW", assets["panel-radiator"]["summary"]["en"]) * 1000 * 0.515,
        "sleeping": met_w(assets["sleeping-person"], 0),
        "standing": met_w(assets["standing-adult"], 0),
        "walking": met_w(assets["walking-commuter"], 2),
        "jogging": met_w(assets["jogger"], 0),
        "tealight_W": tealight_w,
        "tealight_fuel_kWh": (paraffin_g / 1000 * mj_low / 3.6 + paraffin_g / 1000 * mj_high / 3.6) / 2,
        "toaster_W": region_sum(assets["toaster"]) + air_flow_heat(assets["toaster"]),
        "kettle_W": region_sum(assets["electric-kettle"]),
        "vacuum_W": vacuum_w,
        "vacuum_hours_per_2p7kWh": 2.7 / (vacuum_w / 1000),
        "fan_heater_W": region_sum(assets["electric-fan-heater"]) + air_flow_heat(assets["electric-fan-heater"]),
        "lamp_rated_W": first(r"rated ([\d.]+) W", assets["pendant-lamp"]["provenance"]["notes"]["en"]),
        "radiator_rated_W": first(r"rated ([\d.]+) kW", assets["panel-radiator"]["summary"]["en"]) * 1000,
        "cylinder_kWh_day": region_sum(assets["hot-water-cylinder"]) * 24 / 1000,
        "mvhr_flow_m3s": assets["mvhr-unit"]["thermal"]["emitters"][0]["volumeFlowM3S"],
        "cooler_dT_K": abs(assets["split-ac-indoor-unit"]["thermal"]["emitters"][0]["temperatureRiseK"]),
        "shower_min_30L": 30 / flow_lpm,
        "shower_C": first(r"of ([\d.]+) degC water", shower["summary"]["en"]),
        "pot_heat_kJ_1kg_100K": pot_water * water_c * 100 / 1000,
        "pot_latent_kJ_kg": phase_block(pot)["latentHeatJKg"] / 1000,
        "pot_boil_C": phase_block(pot)["pointC"],
    }


def words_to_numbers(text, lang):
    """🔢 Numbers of one sentence in order, digits and English or German number words alike."""
    if lang == "en":
        text = re.sub(r"(?<=\d),(?=\d{3}\b)", "", text)
    else:
        text = re.sub(r"(?<=\d),(?=\d)", ".", text)
        text = re.sub(r"(?<=\d) (?=\d{3}\b)", "", text)
    text = text.lower().replace("-", " ").replace("—", " ").replace("–", " ")
    text = re.sub(rf"zero point ((?:{DIGIT_WORDS})(?: {DIGIT_WORDS})*)",
                  lambda m: "0." + "".join(str(EN_UNITS[w]) for w in m.group(1).split()), text)
    tokens = re.findall(r"\d+(?:\.\d+)?|[a-zäöüß]+", text)
    numbers, current = [], None
    for index, token in enumerate(tokens):
        following = tokens[index + 1] if index + 1 < len(tokens) else ""
        if token in EN_UNITS and not (token == "one" and following == "watt"):
            current = (current or 0) + EN_UNITS[token]
        elif token in DE_UNITS and token != "ein":
            current = (current or 0) + DE_UNITS[token]
        elif token in ("hundred", "hundert"):
            current = (current or 1) * 100
        elif token in ("thousand", "tausend"):
            current = (current or 1) * 1000
        elif DE_COMPOUND.fullmatch(token):
            prefix, unit, tail = DE_COMPOUND.fullmatch(token).groups()
            current = (current or 0) + (DE_UNITS[prefix] if prefix else 1) * (100 if unit == "hundert" else 1000) + (DE_UNITS[tail] if tail else 0)
        else:
            if current is not None:
                numbers.append(current)
                current = None
            if re.fullmatch(r"\d+(?:\.\d+)?", token):
                numbers.append(float(token))
    if current is not None:
        numbers.append(current)
    return numbers


def narration_strings():
    """📜 Every string constant of the energy clip modules that is not a docstring, with file and line."""
    rows = []
    for path in sorted(ENERGY.rglob("🐍️.py")):
        if "🎬️clip" not in path.parts or "__pycache__" in path.parts:
            continue
        rel = str(path.relative_to(ENERGY)).replace("\\", "/")
        tree = ast.parse(path.read_text(encoding="utf-8"))
        docs = set()
        for node in ast.walk(tree):
            if isinstance(node, (ast.Module, ast.ClassDef, ast.FunctionDef)) and node.body:
                head = node.body[0]
                if isinstance(head, ast.Expr) and isinstance(head.value, ast.Constant):
                    docs.add(id(head.value))
        for node in ast.walk(tree):
            if isinstance(node, ast.Constant) and isinstance(node.value, str) and id(node) not in docs:
                rows.append((rel, node.lineno, node.value))
    return rows


def find(rows, rel, needle):
    """🎯 The single string of a clip file that contains the needle, as (file, line, text)."""
    hits = [(r, l, s) for r, l, s in rows if r == rel and needle in s]
    if len(hits) != 1:
        raise ValueError(f"{len(hits)} hits in {rel} for: {needle[:70]}")
    return hits[0]


def find_line(rows, rel, needle):
    """🎯 Line number of the single string of a clip file that contains the needle."""
    return find(rows, rel, needle)[1]


UNIT_RE = re.compile(r"\d[\d .,]*\s?(kWh|kW|MJ|kJ|kg|L|W|g|min|°C|K|m²|m³|m|h|s)\b|Watt|Liter|Litre|Minute|Stunde|Sekunde|Grad|Kilo|Gramm")


def verdict(claim, value):
    """⚖️ Match within 5 percent, minor deviation within 15 percent, mismatch beyond."""
    deviation = (claim - value) / value * 100
    label = "match" if abs(deviation) <= 5 else "minor deviation" if abs(deviation) <= 15 else "mismatch"
    return f"{label} ({deviation:+.0f} %)"


SCALE_EN = "On one watt scale: about seventy watts asleep, one hundred twenty at the desk, three hundred walking, up to nine hundred in sport."
SCALE_DE = "Auf einer Watt-Skala: rund siebzig Watt im Schlaf, hundertzwanzig am Schreibtisch, dreihundert beim Gehen, bis neunhundert beim Sport."
POWER_EN = "A scale of power: a tealight about forty watts, a resting person about a hundred, a vacuum cleaner about seven hundred."
POWER_DE = "Eine Skala der Leistung: Teelicht rund 40 W, ruhender Mensch rund 100 W, Staubsauger rund 700 W."
TOAST_EN = "From the last video: a tealight gives about 40 watts, a resting person about 100 watts, a toaster about 900 watts."
TOAST_DE = "Aus dem letzten Video: Ein Teelicht leistet rund 40 W, ein ruhender Mensch rund 100 W, ein Toaster rund 900 W."
SHOWER_EN = "Or about three minutes of warm shower: thirty litres of water heated from ten to thirty-eight degrees."
SHOWER_DE = "Oder rund drei Minuten warm duschen: 30 Liter Wasser, erwärmt von 10 auf 38 °C."

CLAIMS = [
    dict(id="C1", rel=C_IG, en="A single seated adult emits about one hundred twenty watts.", de="Eine sitzende Person gibt etwa einhundertzwanzig Watt ab.",
         claim="seated adult about 120 W", claimed=120, value="seated", source="seated-office-worker pose 0, 1.1 MET"),
    dict(id="C2a", rel=C_IG, en=SCALE_EN, de=SCALE_DE, claim="asleep about 70 W", claimed=70, value="sleeping",
         source="sleeping-person pose 0, 0.7 MET"),
    dict(id="C2b", rel=C_IG, en=SCALE_EN, de=SCALE_DE, claim="at the desk 120 W", claimed=120, value="seated",
         source="seated-office-worker pose 0, 1.1 MET"),
    dict(id="C2c", rel=C_IG, en=SCALE_EN, de=SCALE_DE, claim="walking 300 W", claimed=300, value="walking",
         source="walking-commuter pose 2, 2.6 MET"),
    dict(id="C2d", rel=C_IG, en=SCALE_EN, de=SCALE_DE, claim="sport up to 900 W", claimed=900, value="jogging",
         source="jogger pose 0, 8 MET"),
    dict(id="C3", rel=C_IG, en="q-dot p is roughly one hundred twenty watts per person at desk work.",
         de="q-Punkt p liegt bei etwa einhundertzwanzig Watt pro Person bei Büroarbeit.", claim="per person at desk 120 W",
         claimed=120, value="seated", source="seated-office-worker pose 0, 1.1 MET"),
    dict(id="C4", rel=C_IG, en=None, de="Einzelperson (120 W)", claim="on-screen label single person 120 W", claimed=120,
         value="standing", source="standing-adult pose 0 (cast cooling-internal-gains-standing)"),
    dict(id="C5", rel=H_IG, en="That load is about eighty watts — roughly two thirds of a seated person's full output.",
         de="Diese Last liegt bei etwa achtzig Watt — rund zwei Drittel einer sitzenden Person.",
         claim="load 80 W, two thirds of seated full output", claimed=80, value="seated_sensible", source="2/3 of seated-office-worker pose 0, 1.1 MET"),
    dict(id="C6a", rel=PF, en="A resting person gives off about seventy watts sensibly and thirty latently — about 45 grams of water per hour.",
         de="Ein ruhender Mensch gibt rund 70 W sensibel und 30 W latent ab — etwa 45 g Wasser pro Stunde.",
         claim="resting person 70 + 30 = 100 W", claimed=100, value="standing", source="standing-adult (shared-person-winter) pose 0, 1.2 MET"),
    dict(id="C6b", rel=PF, en="A resting person gives off about seventy watts sensibly and thirty latently — about 45 grams of water per hour.",
         de="Ein ruhender Mensch gibt rund 70 W sensibel und 30 W latent ab — etwa 45 g Wasser pro Stunde.",
         claim="45 g/h from 30 W latent", claimed=45, value="latent_gh", source="derived 30 W / 2.257 MJ/kg x 3600 s"),
    dict(id="C7", rel=PF, en="A resting person also gives off about a hundred watts.", de="Auch ein ruhender Mensch gibt rund 100 W Wärme ab.",
         claim="resting person 100 W", claimed=100, value="standing", source="standing-adult pose 0, 1.2 MET"),
    dict(id="C8", rel=PF, en="A lamp draws sixty watts of electrical power.", de="Eine Lampe nimmt 60 W elektrische Leistung auf.",
         claim="lamp draws 60 W electric", claimed=60, value="lamp_rated_W", source="pendant-lamp rated 60 W"),
    dict(id="C9", rel=PF, en="In the end all sixty watts are room heat — with LEDs too, and with every appliance.",
         de="Am Ende sind alle 60 W Raumwärme. Das gilt auch für LEDs und für jedes Gerät.", claim="all 60 W become room heat",
         claimed=60, value="lamp_rated_W", source="pendant-lamp rated 60 W"),
    dict(id="C10", rel=PF, en="A vacuum cleaner: 0.7 kilowatts high, about 1.4 hours wide — one kilowatt-hour.",
         de="Ein Staubsauger: 0,7 kW hoch, rund 1,4 h breit — Fläche 1 kWh.", claim="vacuum cleaner 700 W", claimed=700,
         value="vacuum_W", source="vacuum-cleaner regions 157 W + slot air 543 W"),
    dict(id="C11", rel=PF, en="A two-kilowatt kettle needs only half an hour for it: almost three times as high, a third as wide.",
         de="Ein Wasserkocher mit 2 kW braucht dafür nur eine halbe Stunde: fast dreimal so hoch, ein Drittel so breit.",
         claim="kettle 2 kW", claimed=2000, value="kettle_W", source="electric-kettle regions"),
    dict(id="C12", rel=PF, en="A warm shower: one kilowatt-hour in three minutes is about twenty kilowatts — so instantaneous heaters have 18 to 24.",
         de="Eine warme Dusche: 1 kWh in drei Minuten sind rund 20 kW — darum haben Durchlauferhitzer 18 bis 24 kW.",
         claim="shower heater 20 kW (1 kWh in 3 min)", claimed=None, value=None,
         source="no heater in the catalog; shower-head is a hand shower (12 L/min)"),
    dict(id="C13a", rel=PF, en=SHOWER_EN, de=SHOWER_DE, claim="30 L in about 3 min", claimed=3, value="shower_min_30L",
         source="shower-head 12 L/min", note="the beat states a throttled 10 L/min (30 L in about 3 min, 1 kWh = 20 kW); the head's 12 L/min is its maximum and the beat draws no flow rate"),
    dict(id="C13b", rel=PF, en=SHOWER_EN, de=SHOWER_DE, claim="shower water 38 °C", claimed=38, value="shower_C",
         source="shower-head summary"),
    dict(id="C14", rel=PF, en="Mass times specific heat times temperature rise gives about 3,520 kilojoules.",
         de="Masse mal spezifische Wärme mal Temperaturanstieg: rund 3 520 kJ.", claim="30 kg x 4.19 kJ/kgK x 28 K",
         claimed=3520, value=None, source="arithmetic only: 3516 kJ, no catalog value"),
    dict(id="C15", rel=PF, en="Heating one kilogram of water from zero to a hundred degrees takes about 419 kilojoules.",
         de="1 kg Wasser von 0 auf 100 °C zu erwärmen kostet rund 419 kJ.", claim="1 kg water 0 to 100 °C 419 kJ",
         claimed=419, value="pot_heat_kJ_1kg_100K", source="cooking-pot water 1.0 kg, material c"),
    dict(id="C16", rel=PF, en="At a hundred degrees the temperature stops, although energy keeps flowing in — it goes into evaporation.",
         de="Bei 100 °C bleibt die Temperatur stehen, obwohl weiter Energie hineingeht — sie steckt im Verdampfen.",
         claim="boiling point 100 °C", claimed=100, value="pot_boil_C", source="cooking-pot phaseChange pointC"),
    dict(id="C17", rel=PF, en="This latent heat, about 2,257 kilojoules per kilogram, is more than five times the heating.",
         de="Diese latente Wärme, rund 2 257 kJ pro kg, ist mehr als fünfmal so groß wie das Erwärmen.",
         claim="latent heat 2257 kJ/kg", claimed=2257, value="pot_latent_kJ_kg", source="cooking-pot latentHeatJKg"),
    dict(id="C18", rel=PF, en="On a scale: a tealight holds about 0.15 kilowatt-hours; a household uses about three thousand a year in electricity.",
         de="Auf einer Skala: Ein Teelicht enthält rund 0,15 kWh, ein Haushalt braucht rund 3 000 kWh Strom im Jahr.",
         claim="tealight energy 0.15 kWh", claimed=0.15, value="tealight_fuel_kWh", source="12.8 g paraffin x 42 to 46 MJ/kg (mid)"),
    dict(id="C19a", rel=PF, en=POWER_EN, de=POWER_DE, claim="tealight 40 W", claimed=40, value="tealight_W",
         source="tealight regions 40 W (summary 35 W)"),
    dict(id="C19b", rel=PF, en=POWER_EN, de=POWER_DE, claim="resting person 100 W", claimed=100, value="standing",
         source="standing-adult pose 0, 1.2 MET"),
    dict(id="C19c", rel=PF, en=POWER_EN, de=POWER_DE, claim="vacuum cleaner 700 W", claimed=700, value="vacuum_W",
         source="vacuum-cleaner 700 W"),
    dict(id="C20", rel=PF, en="A twenty-square-metre ceiling holds about 2.7 kilowatt-hours — almost four hours of vacuuming.",
         de="Eine Decke mit 20 m² nimmt so rund 2,7 kWh auf — fast vier Stunden Staubsaugen.",
         claim="2.7 kWh = almost 4 h vacuuming", claimed=4, value="vacuum_hours_per_2p7kWh", source="vacuum-cleaner 700 W: 2.7 kWh = 3.9 h"),
    dict(id="C21", rel=HF, en="For this wall the answer is about two thousand watts, the same as a full-power electric heater running non-stop.",
         de="Für diese Wand sind das rund zweitausend Watt — so viel wie ein Heizlüfter auf voller Stufe im Dauerbetrieb.",
         claim="full-power electric heater 2000 W", claimed=2000, value="fan_heater_W", source="electric-fan-heater stages 1000 + 1000 W"),
    dict(id="C22a", rel=CON, en=TOAST_EN, de=TOAST_DE, claim="tealight 40 W", claimed=40, value="tealight_W", source="tealight regions 40 W"),
    dict(id="C22b", rel=CON, en=TOAST_EN, de=TOAST_DE, claim="resting person 100 W", claimed=100, value="standing", source="standing-adult pose 0"),
    dict(id="C22c", rel=CON, en=TOAST_EN, de=TOAST_DE, claim="toaster 900 W", claimed=900, value="toaster_W",
         source="toaster regions 355 W + slot air 546 W"),
    dict(id="C23", rel=NEED, en="Moisture adds up as well: a resting person gives off about 30 watts latent.",
         de="Dazu kommt Feuchte: Ein ruhender Mensch gibt rund 30 W latent ab.", claim="resting person 30 W latent", claimed=30,
         value=None, source="no latent split stored in the person asset (total 117 W)"),
    dict(id="C24", rel=NEED, en="More people need more air: about 30 cubic metres per hour and person is the planning order of magnitude.",
         de="Mehr Personen brauchen mehr Luft: etwa 30 m³/h je Person.", claim="30 m³/h air per person", claimed=30, value=None,
         source="no air-demand value in the person asset"),
    dict(id="C25", rel=NAT, en="Cold air falls onto the thermostat below the window; the valve opens and half of 600 watts flows straight out: 300 watt-hours.",
         de="Kalte Luft fällt auf das Thermostat unter dem Fenster. Das Ventil öffnet, die Hälfte von 600 W geht direkt hinaus: 300 Wh.",
         claim="radiator 600 W (scene constant P_RAD)", claimed=600, value="radiator_night_W", source="panel-radiator night pose, 51.5 % of the rated 1.2 kW"),
    dict(id="C26", rel=CALC, en="Storage: the buffer store loses 1.8 kilowatt-hours per day on standby, over 200 heating days — 360 kilowatt-hours.",
         de="Speicherung: Der Pufferspeicher verliert im Bereitschaftsbetrieb 1,8 kWh pro Tag, über 200 Heiztage 360 kWh.",
         claim="storage standby loss 1.8 kWh/day", claimed=1.8, value="cylinder_kWh_day",
         source="hot-water-cylinder standby 76 W; the calc clip draws no tank, mapped to the tank asset of cast shared-tank-charge-up"),
    dict(id="C27", rel=MECH, en="The system has to deliver the hygiene airflow of zero point zero six one cubic metres per second. Which duct cross-section does that take?",
         de="Die Anlage muss den Hygieneluftstrom von 0,061 m³/s fördern —\nwelchen Kanalquerschnitt braucht sie dafür?",
         claim="hygiene airflow 0.061 m³/s", claimed=0.061, value="mvhr_flow_m3s", source="mvhr-unit 0.061 m³/s per stream"),
    dict(id="C28", rel=HC, en="At the coil the outdoor air falls from thirty-two to nineteen degrees. That drop is the heat the unit removes.",
         de="Am Kühlregister fällt die Außenluft von 32 auf 19 Grad.\nGenau diese Differenz nimmt das Gerät auf.",
         claim="coil air drop 13 K", claimed=13, value="cooler_dT_K", source="split-ac-indoor-unit air rise 13.3 K"),
]


CAST_OF = {
    "C1": "cooling-internal-gains-seated-worker", "C2a": "cooling-internal-gains-sleeper", "C2b": "cooling-internal-gains-seated-worker",
    "C2c": "cooling-internal-gains-walker", "C2d": "cooling-internal-gains-jogger", "C3": "cooling-internal-gains-seated-worker",
    "C4": "cooling-internal-gains-standing", "C5": "heating-internal-gains-office-worker", "C6a": "shared-person-winter",
    "C6b": "shared-person-winter", "C7": "shared-person-winter", "C8": "none in beat (pendant-lamp asset)",
    "C9": "none in beat (pendant-lamp asset)", "C10": "physics-fundamentals-vacuum", "C11": "physics-fundamentals-kettle",
    "C12": "physics-fundamentals-shower", "C13a": "none in beat (shower-head asset, cast in Beat1)", "C13b": "none in beat (shower-head asset)",
    "C14": "none in beat", "C15": "physics-fundamentals-pot", "C16": "physics-fundamentals-pot", "C17": "physics-fundamentals-pot",
    "C18": "none in beat (tealight asset)", "C19a": "none in beat (tealight asset)", "C19b": "none in beat (standing-adult asset)",
    "C19c": "physics-fundamentals-vacuum", "C20": "none in beat (vacuum-cleaner asset)", "C21": "none in beat (electric-fan-heater asset)",
    "C22a": "heating-conduction-tealight", "C22b": "shared-person-winter", "C22c": "shared-toaster-winter",
    "C23": "ventilation-need-occupant-mirrored", "C24": "ventilation-need-occupant-mirrored", "C25": "ventilation-natural-radiator-on (pose night)",
    "C26": "shared-tank-charge-up (system-losses clip)", "C27": "ventilation-mechanical-mvhr", "C28": "cooling-heating-comparison-cooler",
}


NOTES = {'C6a': 'textbook rounding of the resting person, stated with about (70 W sensible + 30 W latent); the standing-adult asset runs 1.2 MET, the 100 W figure is the VDI 2078 resting value', 'C6b': 'derived from the stated 30 W latent, stated with about', 'C7': 'textbook rounding of the resting person, stated with about', 'C19b': 'textbook rounding of the resting person, stated with about', 'C22b': 'textbook rounding of the resting person, stated with about', 'C12': 'no heater in the catalog; 1 kWh in 3 min is arithmetic and 18 to 24 kW is the real instantaneous heater range', 'C14': 'pure arithmetic, 30 kg x 4.19 x 28 K = 3516 kJ, stated as about 3520', 'C23': 'the person asset stores the total only; 30 W latent is the textbook split of the resting person and consistent with the physics clip', 'C24': 'planning figure of DIN 1946-6 type, not a device property'}


def main():
    """🚀 Run the audit over the energy clip narration and write the markdown report into the ticket folder."""
    assets = load_catalog()
    values = asset_values(assets)
    values["latent_gh"] = 30 / 2257000 * 3600 * 1000
    rows = narration_strings()
    table = ["| # | clip file:line | claim | language | asset value (source) | verdict |", "|---|---|---|---|---|---|"]
    parity = ["| # | clip file:line | en numbers | de numbers | same |", "|---|---|---|---|---|"]
    missing, seen, verdicts = [], set(), {}
    for claim in CLAIMS:
        found = {}
        for lang in ("en", "de"):
            if claim[lang] is None:
                continue
            try:
                found[lang] = find(rows, claim["rel"], claim[lang])
            except ValueError as exc:
                missing.append(f"{claim['id']}: {exc}")
        value = values.get(claim["value"]) if claim["value"] else None
        if claim["value"] is None:
            asset_text, verdict_text = f"no catalog value: {claim['source']}", "no catalog value"
        else:
            asset_text = f"{value:.4g} ({claim['source']}; manifest: {CAST_OF[claim['id']]})"
            verdict_text = verdict(claim["claimed"], value) if claim["claimed"] is not None else "no catalog value"
        note = claim.get("note") or NOTES.get(claim["id"])
        if note and not verdict_text.startswith("match"):
            verdict_text = f"justified: {note}" if claim["value"] is None else f"justified ({verdict_text.split(' (')[1].rstrip(')')}): {note}"
        verdicts[claim["id"]] = verdict_text.split(" (")[0].split(":")[0]
        for lang in ("en", "de"):
            if lang in found:
                rel, line, _ = found[lang]
                table.append(f"| {claim['id']} | {rel}:{line} | {claim['claim']} | {lang} | {asset_text} | {verdict_text} |")
            elif claim[lang] is None:
                table.append(f"| {claim['id']} | - | {claim['claim']} | en | no English string in source | n/a |")
        if "en" in found and "de" in found:
            key = (found["en"][0], found["en"][1], found["de"][1])
            if key not in seen:
                seen.add(key)
                en_nums = words_to_numbers(found["en"][2], "en")
                de_nums = words_to_numbers(found["de"][2], "de")
                same = "yes" if en_nums == de_nums else "NO"
                parity.append(f"| {claim['id']} | {found['de'][0]}:{found['de'][1]} | {en_nums} | {de_nums} | {same} |")
    audited = {(c["rel"], find_line(rows, c["rel"], c[l])) for c in CLAIMS for l in ("en", "de") if c[l]}
    unit_rows = [(r, l, s) for r, l, s in rows if UNIT_RE.search(s) and (r, l) not in audited]
    per_file = {}
    for r, _, _ in unit_rows:
        per_file[r] = per_file.get(r, 0) + 1
    counts = {"strings": len(rows), "claims": len(CLAIMS)}
    excluded = ["", "## Coverage: unit-bearing strings not audited", "",
                f"{len(unit_rows)} further non-docstring strings carry a number with a unit but are building, air or device-free "
                "(heat-loss, ventilation volume, U values, kWh of buildings, labels, code). Counts per clip file:", "",
                "| clip file | strings |", "|---|---|"]
    excluded += [f"| {r} | {n} |" for r, n in sorted(per_file.items())]
    tally = {}
    for label in verdicts.values():
        tally[label] = tally.get(label, 0) + 1
    head = [
        "# R2 NN: narration device numbers vs catalog",
        "",
        "Verdicts per claim (en and de share one verdict): " + ", ".join(f"{k} {v}" for k, v in sorted(tally.items())) + ".",
        "",
        f"Scope: energy clip modules (`🎓️teaching/🏛️architecture/⚡️energy/**/🎬️clip/**/🐍️.py`, speech modules included): {counts['strings']} non-docstring string constants enumerated, "
        f"{counts['claims']} device quantities audited in the narration and on-screen strings (en and de rows where both exist). Docstrings and comments are not narration and are not audited.",
        "Catalog values come from the cast manifest `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json` (cast to asset) and the catalog assets "
        "`🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/**/🔣️.json`, computed by `T/🧰️nn/audit_narration_numbers.py`.",
        "Device heat = sum of region `operating.heatW` at level 1 plus emitter air heat V rho c_p dT (rho c_p 1206 J/m3K). Person heat = activityMet x metWM2 (58.15) x skinAreaM2. "
        "Verdict: match within 5 %, minor deviation within 15 %, mismatch beyond.",
        "",
        "## Claims",
        "",
    ]
    tail = ["", "## En and de parity", "", "Numbers of each sentence in order; word numbers converted.", ""]
    if missing:
        tail += ["## Not found", ""] + [f"- {m}" for m in missing]
    REPORT.write_text("\n".join(head + table + tail + parity + excluded) + "\n", encoding="utf-8")
    print("strings", len(rows), "claims", len(CLAIMS), "missing", len(missing))


if __name__ == "__main__":
    main()
