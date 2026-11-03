"""🔢 Compare en and de number multisets (value plus adjacent unit) in bilingual staffage text."""
import json
import os
import re
import sys
from collections import Counter
from decimal import Decimal
from pathlib import Path

CATALOG, MANIFEST, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
EN_NUM = re.compile(r'(?<![A-Za-z_°²³])(\d{1,3}(?:,\d{3})+(?:\.\d+)?|\d+(?:\.\d+)?)(?!\d)')
DE_NUM = re.compile(r'(?<![A-Za-zÄÖÜäöüß°²³_])(\d{1,3}(?:\.\d{3})+(?:,\d+)?|\d+(?:[,.]\d+)?)(?!\d)')
UNIT = re.compile(r'[\s-]?(degrees?\s?C(?:elsius)?\b|Grad\s?C(?:elsius)?\b|[A-Za-z°%µ²³℃][A-Za-z°%µ²³/0-9]*)')
ALIAS = {'Watt': 'W', 'Kilowatt': 'kW', 'Kelvin': 'K', 'Prozent': '%', 'percent': '%', 'Meter': 'm', 'metres': 'm',
         'meters': 'm', 'Kilogramm': 'kg', 'kilograms': 'kg', 'Sekunden': 's', 'seconds': 's', 'Stunden': 'h',
         'hours': 'h', 'Minuten': 'min', 'minutes': 'min', 'Quadratmeter': 'm2', 'Kubikmeter': 'm3', 'Liter': 'l', 'degrees': 'deg', 'Grad': 'deg', 'degC': 'deg', '°C': 'deg', '°': 'deg', 'L': 'l'}
KNOWN = {'W', 'kW', 'MW', 'Wh', 'kWh', 'J', 'kJ', 'MJ', 'V', 'A', 'Hz', 'Pa', 'kPa', 'bar', 'm', 'm2', 'm3', 'mm', 'cm',
         'km', 'kg', 'g', 't', 's', 'h', 'min', 'K', '%', 'dB', 'l', 'L', 'lx', 'W/m2', 'W/m2K', 'W/mK', 'W/m3',
         'kg/m3', 'm3/s', 'm3/h', 'kWh/m2', 'kWh/m2a', 'm/s', 'm/s2', 'deg', 'l'}
YEAR = re.compile(r'^(19|20)\d\d$')


def canon(tok):
    """🧭 Normalise a unit token to its canonical spelling, or empty when it is not a known unit."""
    t = tok.replace('²', '2').replace('³', '3').replace('µ', 'u').replace('℃', '°C')
    t = re.sub(r'^(?:degrees?|Grad)\s?C(?:elsius)?$', 'deg', t)
    t = ALIAS.get(t, t)
    return t if t in KNOWN else ''


def extract(text, lang):
    """🔎 Return (value, unit) pairs for every number in a text, with German decimal commas and dot thousands normalised."""
    out = []
    for m in (EN_NUM if lang == 'en' else DE_NUM).finditer(text):
        raw = m.group(1)
        s = raw.replace(',', '') if lang == 'en' else (raw.replace('.', '').replace(',', '.') if re.fullmatch(r'[1-9]\d{0,2}(?:\.\d{3})+(?:,\d+)?', raw) else raw.replace(',', '.'))
        u = UNIT.match(text, m.end())
        out.append((f'{Decimal(s).normalize():f}', canon(u.group(1)) if u else ''))
    return out


def warnings(text, lang):
    """⚠️ Flag decimal separators that belong to the other language."""
    bad = r'\d,\d{1,2}(?!\d)' if lang == 'en' else r'\d\.\d{1,2}(?!\d)'
    return [f'{lang}: suspicious decimal separator' for _ in [0] if re.search(bad, text)]


def compare(en_text, de_text):
    """⚖️ Return the number pairs present only in en and only in de, ignoring year-like numbers present in both; units are reported by the caller, not compared."""
    ce, cd = Counter(v for v, _ in extract(en_text, 'en')), Counter(v for v, _ in extract(de_text, 'de'))
    for k in set(ce) & set(cd):
        if YEAR.match(k):
            del ce[k]
            del cd[k]
    return sorted((ce - cd).elements()), sorted((cd - ce).elements())


def walk(node, pointer, file, found):
    """🌳 Collect every bilingual {en, de} string pair with its file and JSON pointer."""
    if isinstance(node, dict):
        if isinstance(node.get('en'), str) and isinstance(node.get('de'), str):
            found.append((file, pointer, node['en'], node['de']))
            return
        for k, v in node.items():
            walk(v, f'{pointer}/{k}', file, found)
    elif isinstance(node, list):
        for i, v in enumerate(node):
            walk(v, f'{pointer}/{i}', file, found)


def main():
    """🚀 Check every catalog and manifest file, print mismatches and write them as JSON."""
    files = [MANIFEST]
    for root, _, names in os.walk(CATALOG):
        files += [Path(root) / n for n in names if n.endswith('.json') and '🔣' in n]
    found = []
    for f in sorted(set(files)):
        walk(json.loads(f.read_text(encoding='utf-8')), '', str(f), found)
    report, warn = [], []
    for file, pointer, en, de in found:
        only_en, only_de = compare(en, de)
        warn += [(file, pointer, w) for w in warnings(en, 'en') + warnings(de, 'de')]
        if only_en or only_de:
            report.append({'file': file, 'pointer': pointer, 'en': en, 'de': de,
                           'only_en': only_en, 'only_de': only_de})
    print(f'files={len(set(files))} pairs={len(found)} mismatches={len(report)} warnings={len(warn)}')
    for r in report:
        print(f"\n## {r['file']}{r['pointer']}\n  only_en={r['only_en']} only_de={r['only_de']}\n  en: {r['en']}\n  de: {r['de']}")
    for w in warn:
        print(f'WARN {w[0]}{w[1]} {w[2]}')
    OUT.write_text(json.dumps(report, ensure_ascii=False, indent=1), encoding='utf-8')


if __name__ == '__main__':
    main()
