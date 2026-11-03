#!/usr/bin/env python3
"""EN 1995 `ImposedCategory::SnowHigh`: the EN 1990 Table A1.1 snow row for sites above 1000 m a.s.l. (own psi0/psi1/psi2, registry row `ndp.en1990.psi.snow_high`) that v2 dropped together with the string kind `snow_high`.
It is a psi category of a snow action (`kind = snow`, `category = snowHigh`), so every consumer of `ActionKind::Snow` keeps working. Every replacement asserts its match count; a second run changes nothing.
Usage: python3 -I en1995_snow_high.py [--dry]"""
import sys
R = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/"
DRY = "--dry" in sys.argv
OPT = ',"snowHigh":{"en":"S — snow above 1000 m a.s.l.","de":"S — Schnee über 1000 m ü. NN"}'
EDITS = {
    "🧬️schema/📸️snapshot/🦀️.rs": [
        ("/// 🏢 Category of an imposed load for the combination factors psi (EN 1990 Table A1.1).", "/// 🏢 Category of an action for the combination factors psi (EN 1990 Table A1.1): imposed load categories A to H and the snow row for sites above 1000 m a.s.l.", 1),
        ("    G,\n    H,\n}\n\n/// ⏳ Load-duration", "    G,\n    H,\n    SnowHigh,\n}\n\n/// ⏳ Load-duration", 1),
    ],
    "🧬️schema/📸️snapshot/🟦️.ts": [
        ('"notImposed" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h";', '"notImposed" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "snowHigh";', 1),
        ('normWireLiteral("notImposed", "a", "b", "c", "d", "e", "f", "g", "h")', 'normWireLiteral("notImposed", "a", "b", "c", "d", "e", "f", "g", "h", "snowHigh")', 1),
    ],
    "🧬️schema/📸️snapshot/🔣️.json": [
        ('"enum":["notImposed","a","b","c","d","e","f","g","h"]', '"enum":["notImposed","a","b","c","d","e","f","g","h","snowHigh"]', 1),
        ('"h":{"en":"H — roofs not accessible","de":"H — Nicht begehbare Dächer"}}', '"h":{"en":"H — roofs not accessible","de":"H — Nicht begehbare Dächer"}' + OPT + "}", 1),
    ],
    "🧬️schema/🔣️.json": [
        ('"enum":["notImposed","a","b","c","d","e","f","g","h"]', '"enum":["notImposed","a","b","c","d","e","f","g","h","snowHigh"]', 1),
        ('"h":{"en":"H — roofs not accessible","de":"H — Nicht begehbare Dächer"}}', '"h":{"en":"H — roofs not accessible","de":"H — Nicht begehbare Dächer"}' + OPT + "}", 1),
    ],
    "🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs": [
        ('ImposedCategory::H=>"h",}}', 'ImposedCategory::H=>"h",ImposedCategory::SnowHigh=>"snowHigh",}}', 1),
        ('"h"=>Ok(ImposedCategory::H),_=>', '"h"=>Ok(ImposedCategory::H),"snowHigh"=>Ok(ImposedCategory::SnowHigh),_=>', 1),
    ],
    "🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts": [
        ('IMPOSED_CATEGORYS:readonly ImposedCategory[]=["notImposed","a","b","c","d","e","f","g","h"]', 'IMPOSED_CATEGORYS:readonly ImposedCategory[]=["notImposed","a","b","c","d","e","f","g","h","snowHigh"]', 1),
    ],
    "✏️editor/🏷️field-meta/🦀️.rs": [
        ('    choice("h", "H — roofs not accessible", "H — Nicht begehbare Dächer"),\n];', '    choice("h", "H — roofs not accessible", "H — Nicht begehbare Dächer"),\n    choice("snow-high", "S — snow above 1000 m a.s.l.", "S — Schnee über 1000 m ü. NN"),\n];', 1),
    ],
    "🧬️schema/⚖️timber/🌲️material-and-annex-factors/🦀️.rs": [
        ('        K::Snow => "snow",\n        K::Wind => "wind",\n        K::Imposed => match category {\n            C::NotImposed => "office",', '        K::Snow => match category {\n            C::SnowHigh => "snow_high",\n            _ => "snow",\n        },\n        K::Wind => "wind",\n        K::Imposed => match category {\n            C::NotImposed | C::SnowHigh => "office",', 1),
        ("snow and wind by kind, imposed actions by category letter A–H,", "snow by kind and category (above 1000 m a.s.l. the row `snow_high`), wind by kind, imposed actions by category letter A–H,", 1),
    ],
    "🔮️oracles/⚖️compliance/🐍️.py": [
        ('    key = k if k in ("snow", "snow_high", "wind") else str(category or "").strip().upper() if k == "imposed" else "office"', '    c = str(category or "").strip().lower()\n    key = ("snow_high" if c == "snowhigh" else "snow") if k == "snow" else "wind" if k == "wind" else c.upper() if k == "imposed" and c != "snowhigh" else "office"', 1),
        ("snow and wind by kind, imposed actions", "snow by kind and category (`snowHigh`: the row `snow_high` above 1000 m a.s.l.), wind by kind, imposed actions", 1),
    ],
}
for rel, edits in EDITS.items():
    p = R + rel
    t = open(p, encoding="utf-8", newline="").read()
    for old, new, n in edits:
        if new in t and old not in t:
            continue
        assert t.count(old) == n, (rel, old[:70], t.count(old))
        t = t.replace(old, new)
    if not DRY:
        open(p, "w", encoding="utf-8", newline="").write(t)
print("ok", "dry" if DRY else "written")
