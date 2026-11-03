"""🔎️ Heuristic checks over the bilingual rows extracted by the r3-doc3 extraction: transliteration, English and German leakage, decimal separators, unit parity and non-SI units."""
import collections
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')
TSV = sys.argv[1]
rows = []
with open(TSV, encoding='utf-8') as handle:
    for line in handle:
        rel, path, en, de = line.rstrip('\n').split('\t')
        rows.append((rel, path, en, de))

EN_STOP = re.compile(r'(?<![\w-])(the|and|with|from|for|are|its|that|which|while|into|each|of|is|by|this|these|being|have|has|between|through)(?![\w-])')
DE_STOP = re.compile(r'(?<![\wäöüÄÖÜß-])(und|mit|der|die|das|ein|eine|einem|einer|von|für|oder|sind|ist|auf|im|zum|zur|dem|den|des|bei|aus|wird|werden)(?![\wäöüÄÖÜß-])')
TRANSLIT = re.compile(r'[A-Za-zäöüÄÖÜß]*(ae|oe|ue|ss|Ae|Oe|Ue)[A-Za-zäöüÄÖÜß]*')
NONSI = re.compile(r'\b(ft|feet|foot|inch|inches|in|lb|lbs|psi|mph|gal|gallon|hp|BTU|Btu|°F|degrees F|deg F|F|oz|yd|acre|therm|ton|tons|cfm|fpm|Fahrenheit|Pfund|Zoll|Fuß|Meile|PS)\b')
UNIT = re.compile(r'(?<![\w.,])(\d+(?:[.,]\d+)*)\s?(m²|m³|m2|m3|mm|cm|km|m2K|W/m²K|W/m2K|W/m²|W/m2|W/mK|W/m|W|kW|MW|kWh|Wh|J|kJ|MJ|kg|g|t|N|kN|Pa|kPa|K|°C|Grad C|degrees C|C|dB|s|min|h|Hz|V|A|%|kg/m3|kg/m³|m/s|m3/s|m³/s|l/s|L|l|kg/kg|°|degrees|Grad|lx|cd|kJ/kgK|J/kgK|J/(kg·K)|kgK|kWh/m²a|kWh/m2a|kg/s|kg/h|m3/h|m³/h|ppm|mg|µm|μm|W/m2K|W/m²·K)(?![\wäöüÄÖÜß])')


def norm_num(value):
    return value.replace(',', '.')


def unit_tokens(text):
    out = []
    for match in UNIT.finditer(text):
        unit = match.group(2).replace('²', '2').replace('³', '3').replace('Grad C', 'C').replace('degrees C', 'C').replace('°C', 'C').replace('degrees', '°').replace('Grad', '°')
        out.append((norm_num(match.group(1)), unit))
    return out


def show(title, items, limit=80):
    print('[DEBUG] === %s: %d' % (title, len(items)))
    for item in items[:limit]:
        print('[DEBUG] %s' % (item,))


same = [(r[0], r[1], r[2]) for r in rows if r[2].strip() == r[3].strip()]
show('identical en/de', same, 200)

en_leak = [(r[0], r[1], r[3][:160]) for r in rows if EN_STOP.search(r[3])]
show('English words in de', en_leak, 400)

de_leak = [(r[0], r[1], r[2][:160]) for r in rows if DE_STOP.search(r[2])]
show('German words in en', de_leak, 200)

translit = collections.Counter()
translit_ex = {}
for r in rows:
    for m in TRANSLIT.finditer(r[3]):
        w = m.group(0)
        translit[w] += 1
        translit_ex.setdefault(w, (r[0], r[1]))
print('[DEBUG] === de transliteration candidate words: %d distinct' % len(translit))
print('[DEBUG] tl ' + ' | '.join('%s:%d' % (w, c) for w, c in sorted(translit.items(), key=lambda kv: kv[0].lower())))

dec_de = [(r[0], r[1], r[3][:120]) for r in rows if re.search(r'(?<![\w.])\d+\.\d', r[3]) and not re.search(r'\d\.\d+\.\d', r[3])]
show('decimal point in de (check)', dec_de, 150)
dec_en = [(r[0], r[1], r[2][:120]) for r in rows if re.search(r'(?<![\w.])\d+,\d{1,2}(?![\d])', r[2])]
show('decimal comma in en (check)', dec_en, 150)

mismatch = []
for r in rows:
    if r[2] == r[3]:
        continue
    a = unit_tokens(r[2])
    b = unit_tokens(r[3])
    if sorted(a) != sorted(b):
        mismatch.append((r[0], r[1], sorted(set(a) - set(b)), sorted(set(b) - set(a))))
show('unit token mismatch en vs de', mismatch, 400)

nonsi = collections.Counter()
nonsi_ex = {}
for r in rows:
    for lang, text in (('en', r[2]), ('de', r[3])):
        for m in NONSI.finditer(text):
            key = lang + ':' + m.group(0)
            nonsi[key] += 1
            nonsi_ex.setdefault(key, (r[0], r[1], text[:140]))
show('non-SI candidates', [(k, c, nonsi_ex[k]) for k, c in sorted(nonsi.items())], 200)
