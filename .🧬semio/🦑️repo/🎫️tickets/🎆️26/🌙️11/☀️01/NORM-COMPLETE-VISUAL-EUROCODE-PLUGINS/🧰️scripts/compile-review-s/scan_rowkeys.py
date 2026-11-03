"""🔎 Every row type behind a KeyOf / row_key! entry must have `id: String` when the key extractor reads `row.id.clone()`."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *
code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, _, files = index_types(adir)
for f, src in files.items():
    if "🔺️diff" not in f or "/🧪️tests/" in f: continue
    rows = set(re.findall(r'(\w+)\s+for\s+(\w+)\s*;', src))
    for key, row in rows:
        defs = [d for _, d in structs.get(row, [])]
        if not defs: print(f"{rel(f)}: row_key {key} for {row}: row struct not found")
        elif not any(d.get("id") == "String" for d in defs): print(f"{rel(f)}: row_key {key} for {row}: no `id: String` field")
    for m in re.finditer(r'KeyOf<(?:\w+::)*(\w+)>\s+for\s+(\w+)\s*\{(.*?)\n\}', src, re.S):
        row, key, body = m.groups()
        fm = re.search(r'row\.(\w+)\b', body)
        defs = [d for _, d in structs.get(row, [])]
        if fm and defs and not any(fm.group(1) in d for d in defs): print(f"{rel(f)}: KeyOf {key} for {row} reads row.{fm.group(1)} which does not exist")
