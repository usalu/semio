from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🔣️.json")
s = p.read_text(encoding="utf-8")
def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)
sub(''',
      "tariff": {"value": 0.25, "unit": "CHF/kWh", "note": {"en": "Electricity price.", "de": "Strompreis."}}
    },''', '''
    },
    "economics": {
      "tariff": {"value": 0.25, "unit": "CHF/kWh", "note": {"en": "Electricity price today.", "de": "Strompreis heute."}},
      "tariff_hi": {"value": 0.4, "unit": "CHF/kWh", "note": {"en": "Electricity price in a high-price scenario.", "de": "Strompreis in einem Szenario mit hohen Preisen."}},
      "service_life": {"value": 30, "unit": "a", "note": {"en": "Years a facade is expected to serve before it is replaced.", "de": "Jahre, die eine Fassade voraussichtlich genutzt wird."}}
    },''')
a = s.index('    "sun": {'); b = s.index('  "surface": {')
block = s[a:b]
s = s[:a].rstrip().rstrip(",") + "\n  },\n" + s[b:]
sun = block.rstrip().rstrip("}").rstrip().rstrip(",")
s = s.replace('  "baseline":', '  "sun": {' + sun.split("{", 1)[1].rstrip() + "\n  },\n  \"baseline\":", 1)
p.write_text(s, encoding="utf-8")
