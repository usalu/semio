"""💧 kp: close the energy balance of the evaporative humidifier (adiabatic: latent heat from the air, pad at the wet-bulb temperature) - run once"""
import json
import re
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
PATH = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/evaporative-humidifier/🔣️.json"
data = json.loads(PATH.read_text(encoding="utf8"))
a = data["asset"]

notes = a["provenance"]["notes"]
notes["en"] = re.sub(
    r"Adiabatic saturation cools.*?rating\)\.",
    "Adiabatic saturation takes the latent heat of the evaporating water (about 2.45 MJ/kg) from the air: at 90 percent efficiency the 22 degC room air (50 percent) leaves at about 16 degC and about 95 percent humidity, 6 K cooler and 2.4 g/kg moister (0.28 m3/s: -2.0 kW sensible, +2.0 kW latent, 0.8 g/s or 2.9 L/h of water). The wetted pad and the sump water sit at the wet-bulb temperature of the room air (about 15 degC), not below it: they carry no heat source, the evaporation is their only sink.",
    notes["en"],
)
notes["de"] = re.sub(
    r"Die adiabate Sättigung kühlt.*?Nennleistung\)\.",
    "Die adiabate Sättigung entzieht der Luft die Verdunstungswärme des Wassers (etwa 2,45 MJ/kg): bei 90 Prozent Wirkungsgrad verlässt die 22 degC warme Raumluft (50 Prozent) das Gerät mit etwa 16 degC und etwa 95 Prozent Feuchte, 6 K kühler und 2,4 g/kg feuchter (0,28 m3/s: -2,0 kW fühlbar, +2,0 kW latent, 0,8 g/s oder 2,9 l/h Wasser). Die benetzte Matte und das Wannenwasser liegen auf der Kühlgrenztemperatur der Raumluft (etwa 15 degC), nicht darunter: sie haben keine Wärmequelle, die Verdunstung ist ihre einzige Senke.",
    notes["de"],
)

for r in a["thermal"]["regions"]:
    if r["id"] in ("pad", "sump"):
        r["model"] = "passive"
        del r["operating"]
    if r["id"] == "sump":
        r["heatCapacityJM2K"] = 125000
        r["layers"][0]["heatCapacityJM2K"] = 125000
    if r["id"] == "pad":
        r["title"] = {"en": "Wet cellulose pad", "de": "Nasse Cellulosematte"}

wet = {"dry": 0, "wetting": 0.5, "running": 1}
for p in a["poses"]:
    if p["id"] in wet:
        p["thermal"]["regions"]["pad"]["wettedness"] = wet[p["id"]]

em = a["thermal"]["emitters"][0]
em["temperatureRiseK"] = -6
em["vapourKgPerKg"] = 0.0106

a["thermal"]["assumptions"] = [{
    "en": "Adiabatic evaporation: the pad and the sump water are passive wet surfaces that settle at the wet-bulb temperature of the room air without a heat source; the pad is dry in the dry pose and half wet while wetting. The air emitter carries the exchange with the airflow: 0.28 m3/s cooled by 6 K (-2.0 kW sensible) and moistened to 10.6 g/kg (+2.0 kW latent, 2.45 MJ per kg of evaporated water), so the sensible and the latent power balance.",
    "de": "Adiabate Verdunstung: die Matte und das Wannenwasser sind passive nasse Oberflächen, die sich ohne Wärmequelle auf die Kühlgrenztemperatur der Raumluft einstellen; die Matte ist in der Pose Trocken trocken und beim Befeuchten halb nass. Der Luftemitter trägt den Austausch mit dem Luftstrom: 0,28 m3/s um 6 K gekühlt (-2,0 kW fühlbar) und auf 10,6 g/kg befeuchtet (+2,0 kW latent, 2,45 MJ je kg verdunstetes Wasser), fühlbare und latente Leistung sind also ausgeglichen.",
}]
PATH.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf8", newline="\n")
PATH.write_text(PATH.read_text(encoding="utf8").replace("4e-05", "0.00004"), encoding="utf8", newline="\n")
print("ok")
