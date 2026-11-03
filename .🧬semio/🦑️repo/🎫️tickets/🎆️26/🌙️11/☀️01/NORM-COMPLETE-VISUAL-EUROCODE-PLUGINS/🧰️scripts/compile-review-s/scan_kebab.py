"""🔎 Kebab-case identifiers in Rust code position (`r0.circle-radius`, `{ circle-radius: Some(..)`) left by generators: a minus between two lowercase words directly after `.` or `{ ` / `, ` before `:`."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *
code = sys.argv[1]
adir = ART + "/" + CODE[code]
for f in rs_files(adir):
    src = strip(open(f, encoding="utf-8").read())
    for m in re.finditer(r'(?<=[\w)\]])\.([a-z][a-z0-9_]*-[a-z][a-z0-9_-]*)\b|[{,]\s*([a-z][a-z0-9_]*-[a-z][a-z0-9_-]*)\s*:(?!:)', src):
        print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: kebab identifier `{m.group(0).strip()[:50]}`")
