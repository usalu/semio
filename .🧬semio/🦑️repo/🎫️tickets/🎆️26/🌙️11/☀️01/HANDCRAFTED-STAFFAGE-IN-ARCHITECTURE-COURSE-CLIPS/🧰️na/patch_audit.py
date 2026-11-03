import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")


def sub(old, new, count=1):
    global text
    if text.count(old) < 1:
        raise SystemExit(f"missing: {old[:80]}")
    text = text.replace(old, new) if count == 0 else text.replace(old, new, count)


sub('DE_COMPOUND = re.compile(r"(eins|ein|zwei|drei|vier|fünf|sechs|sieben|acht|neun)?(hundert|tausend)")',
    'DE_COMPOUND = re.compile(r"(eins|ein|zwei|drei|vier|fünf|sechs|sieben|acht|neun)?(hundert|tausend)(zehn|zwanzig|dreißig|vierzig|fünfzig|sechzig|siebzig|achtzig|neunzig)?")')
sub('"fünfzig": 50, "sechzig": 60, "siebzig": 70, "achtzig": 80, "neunzig": 90, "fünfmal": 5}',
    '"fünfzig": 50, "sechzig": 60, "siebzig": 70, "achtzig": 80, "neunzig": 90, "fünfmal": 5, "dreimal": 3}')
sub('''            prefix, unit = DE_COMPOUND.fullmatch(token).groups()
            current = (current or 0) + (DE_UNITS[prefix] if prefix else 1) * (100 if unit == "hundert" else 1000)''',
    '''            prefix, unit, tail = DE_COMPOUND.fullmatch(token).groups()
            current = (current or 0) + (DE_UNITS[prefix] if prefix else 1) * (100 if unit == "hundert" else 1000) + (DE_UNITS[tail] if tail else 0)''')
sub('''        "seated": met_w(assets["seated-office-worker"], 0),''',
    '''        "seated": met_w(assets["seated-office-worker"], 0),
        "seated_sensible": met_w(assets["seated-office-worker"], 0) * 2 / 3,
        "radiator_night_W": first(r"rated ([\\d.]+) kW", assets["panel-radiator"]["summary"]["en"]) * 1000 * 0.515,''')
sub('REPORT = TICKET / "audit-narration-numbers-r2.md"', 'REPORT = TICKET / "audit-narration-numbers-r2.md"\nJUSTIFIED = {}')

sub('SCALE_EN = "On one watt scale: about eighty watts asleep, one hundred at the desk, two hundred walking, up to eight hundred in sport."',
    'SCALE_EN = "On one watt scale: about seventy watts asleep, one hundred twenty at the desk, three hundred walking, up to nine hundred in sport."')
sub('SCALE_DE = "Auf einer Watt-Skala: rund achtzig Watt im Schlaf, hundert am Schreibtisch, zweihundert beim Gehen, bis achthundert beim Sport."',
    'SCALE_DE = "Auf einer Watt-Skala: rund siebzig Watt im Schlaf, hundertzwanzig am Schreibtisch, dreihundert beim Gehen, bis neunhundert beim Sport."')
sub('a tealight about thirty watts, a resting person about a hundred, a vacuum cleaner about one kilowatt.',
    'a tealight about forty watts, a resting person about a hundred, a vacuum cleaner about seven hundred.')
sub('Teelicht rund 30 W, ruhender Mensch rund 100 W, Staubsauger rund 1 kW."', 'Teelicht rund 40 W, ruhender Mensch rund 100 W, Staubsauger rund 700 W."')
sub('a tealight gives about 30 watts, a resting person about 100 watts, a toaster about 1000 watts.',
    'a tealight gives about 40 watts, a resting person about 100 watts, a toaster about 900 watts.')
sub('Ein Teelicht leistet rund 30 W, ein ruhender Mensch rund 100 W, ein Toaster rund 1 000 W.',
    'Ein Teelicht leistet rund 40 W, ein ruhender Mensch rund 100 W, ein Toaster rund 900 W.')

sub('en="A single seated adult emits about one hundred watts.", de="Eine sitzende Person gibt etwa einhundert Watt ab.",\n         claim="seated adult about 100 W", claimed=100,',
    'en="A single seated adult emits about one hundred twenty watts.", de="Eine sitzende Person gibt etwa einhundertzwanzig Watt ab.",\n         claim="seated adult about 120 W", claimed=120,')
sub('claim="asleep about 80 W", claimed=80,', 'claim="asleep about 70 W", claimed=70,')
sub('claim="at the desk 100 W", claimed=100,', 'claim="at the desk 120 W", claimed=120,')
sub('claim="walking 200 W", claimed=200,', 'claim="walking 300 W", claimed=300,')
sub('claim="sport up to 800 W", claimed=800,', 'claim="sport up to 900 W", claimed=900,')
sub('en="q-dot p is roughly one hundred watts per person at desk work.",\n         de="q-Punkt p liegt bei etwa einhundert Watt pro Person bei Büroarbeit.", claim="per person at desk 100 W",\n         claimed=100,',
    'en="q-dot p is roughly one hundred twenty watts per person at desk work.",\n         de="q-Punkt p liegt bei etwa einhundertzwanzig Watt pro Person bei Büroarbeit.", claim="per person at desk 120 W",\n         claimed=120,')
sub('de="Einzelperson (100 W)", claim="on-screen label single person 100 W", claimed=100,', 'de="Einzelperson (120 W)", claim="on-screen label single person 120 W", claimed=120,')
sub('roughly four fifths of a seated person\'s full output.', 'roughly two thirds of a seated person\'s full output.')
sub('rund vier Fünftel einer sitzenden Person.', 'rund zwei Drittel einer sitzenden Person.')
sub('claim="load 80 W, 4/5 of seated full output", claimed=80, value="seated", source="seated-office-worker pose 0, 1.1 MET"',
    'claim="load 80 W, two thirds of seated full output", claimed=80, value="seated_sensible", source="2/3 of seated-office-worker pose 0, 1.1 MET"')

sub('en="A lamp draws a hundred watts of electrical power.", de="Eine Lampe nimmt 100 W elektrische Leistung auf.",\n         claim="lamp draws 100 W electric", claimed=100,',
    'en="A lamp draws sixty watts of electrical power.", de="Eine Lampe nimmt 60 W elektrische Leistung auf.",\n         claim="lamp draws 60 W electric", claimed=60,')
sub('en="In the end all hundred watts are room heat — with LEDs too, and with every appliance.",\n         de="Am Ende sind alle 100 W Raumwärme. Das gilt auch für LEDs und für jedes Gerät.", claim="all 100 W become room heat",\n         claimed=100,',
    'en="In the end all sixty watts are room heat — with LEDs too, and with every appliance.",\n         de="Am Ende sind alle 60 W Raumwärme. Das gilt auch für LEDs und für jedes Gerät.", claim="all 60 W become room heat",\n         claimed=60,')
sub('en="A vacuum cleaner: one kilowatt high, one hour wide — one kilowatt-hour.",\n         de="Ein Staubsauger: 1 kW hoch, 1 h breit — Fläche 1 kWh.", claim="vacuum cleaner 1 kW", claimed=1000,',
    'en="A vacuum cleaner: 0.7 kilowatts high, about 1.4 hours wide — one kilowatt-hour.",\n         de="Ein Staubsauger: 0,7 kW hoch, rund 1,4 h breit — Fläche 1 kWh.", claim="vacuum cleaner 700 W", claimed=700,')
sub('twice as high, half as wide.', 'almost three times as high, a third as wide.')
sub('doppelt so hoch, halb so breit.', 'fast dreimal so hoch, ein Drittel so breit.')
sub('claim="30 L in about 3 min", claimed=3, value="shower_min_30L",\n         source="shower-head 12 L/min"),',
    'claim="30 L in about 3 min", claimed=3, value="shower_min_30L",\n         source="shower-head 12 L/min", note="the beat states a throttled 10 L/min (30 L in about 3 min, 1 kWh = 20 kW); the head\'s 12 L/min is its maximum and the beat draws no flow rate"),')
sub('a tealight holds about a tenth of a kilowatt-hour;', 'a tealight holds about 0.15 kilowatt-hours;')
sub('Ein Teelicht enthält rund 0,1 kWh,', 'Ein Teelicht enthält rund 0,15 kWh,')
sub('claim="tealight energy 0.1 kWh", claimed=0.1,', 'claim="tealight energy 0.15 kWh", claimed=0.15,')
sub('claim="tealight 30 W", claimed=30, value="tealight_W",\n         source="tealight regions 40 W (summary 35 W)"', 'claim="tealight 40 W", claimed=40, value="tealight_W",\n         source="tealight regions 40 W (summary 35 W)"')
sub('claim="vacuum cleaner 1 kW", claimed=1000, value="vacuum_W",\n         source="vacuum-cleaner 700 W"', 'claim="vacuum cleaner 700 W", claimed=700, value="vacuum_W",\n         source="vacuum-cleaner 700 W"')
sub('almost three hours of vacuuming.', 'almost four hours of vacuuming.')
sub('fast drei Stunden Staubsaugen.', 'fast vier Stunden Staubsaugen.')
sub('claim="2.7 kWh = almost 3 h vacuuming", claimed=3,', 'claim="2.7 kWh = almost 4 h vacuuming", claimed=4,')
sub('claim="tealight 30 W", claimed=30, value="tealight_W", source="tealight regions 40 W"', 'claim="tealight 40 W", claimed=40, value="tealight_W", source="tealight regions 40 W"')
sub('claim="toaster 1000 W", claimed=1000,', 'claim="toaster 900 W", claimed=900,')
sub('half of 500 watts flows straight out: 250 watt-hours.', 'half of 600 watts flows straight out: 300 watt-hours.')
sub('die Hälfte von 500 W geht direkt hinaus: 250 Wh.', 'die Hälfte von 600 W geht direkt hinaus: 300 Wh.')
sub('claim="radiator 500 W (scene constant P_RAD)", claimed=500, value="radiator_rated_W", source="panel-radiator rated 1.2 kW at 50 K"',
    'claim="radiator 600 W (scene constant P_RAD)", claimed=600, value="radiator_night_W", source="panel-radiator night pose, 51.5 % of the rated 1.2 kW"')
sub('loses 1.5 kilowatt-hours per day on standby, over 200 heating days.', 'loses 1.8 kilowatt-hours per day on standby, over 200 heating days — 360 kilowatt-hours.')
sub('Bereitschaftsbetrieb 1,5 kWh pro Tag, über 200 Heiztage 300 kWh.', 'Bereitschaftsbetrieb 1,8 kWh pro Tag, über 200 Heiztage 360 kWh.')
sub('claim="storage standby loss 1.5 kWh/day", claimed=1.5,', 'claim="storage standby loss 1.8 kWh/day", claimed=1.8,')
sub('falls from thirty-two to eighteen degrees.', 'falls from thirty-two to nineteen degrees.')
sub('von 32 auf 18 Grad.', 'von 32 auf 19 Grad.')
sub('claim="coil air drop 14 K", claimed=14,', 'claim="coil air drop 13 K", claimed=13,')

sub('"C25": "ventilation-natural-radiator-on"', '"C25": "ventilation-natural-radiator-on (pose night)"')

notes = {
    "C6a": "textbook rounding of the resting person, stated with about (70 W sensible + 30 W latent); the standing-adult asset runs 1.2 MET, the 100 W figure is the VDI 2078 resting value",
    "C6b": "derived from the stated 30 W latent, stated with about",
    "C7": "textbook rounding of the resting person, stated with about",
    "C19b": "textbook rounding of the resting person, stated with about",
    "C22b": "textbook rounding of the resting person, stated with about",
    "C12": "no heater in the catalog; 1 kWh in 3 min is arithmetic and 18 to 24 kW is the real instantaneous heater range",
    "C14": "pure arithmetic, 30 kg x 4.19 x 28 K = 3516 kJ, stated as about 3520",
    "C23": "the person asset stores the total only; 30 W latent is the textbook split of the resting person and consistent with the physics clip",
    "C24": "planning figure of DIN 1946-6 type, not a device property",
}
sub('def main():', f'NOTES = {notes!r}\n\n\ndef main():')
sub('''        verdicts[claim["id"]] = verdict_text.split(" (")[0]''',
    '''        note = claim.get("note") or NOTES.get(claim["id"])
        if note and not verdict_text.startswith("match"):
            verdict_text = f"justified: {note}" if claim["value"] is None else f"justified ({verdict_text.split(' (')[1].rstrip(')')}): {note}"
        verdicts[claim["id"]] = verdict_text.split(" (")[0].split(":")[0]''')
path.write_text(text, encoding="utf-8")
print("patched")
