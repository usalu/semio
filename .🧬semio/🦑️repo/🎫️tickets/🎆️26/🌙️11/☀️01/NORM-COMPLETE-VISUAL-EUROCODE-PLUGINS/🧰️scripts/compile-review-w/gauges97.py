"""Checks every gauge closure `|r| r.field` of the en1997 gauge tables against the field type (f64) of the snapshot struct."""
import re, sys
A = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any"
g = open(A + "/🎬️scene/🕹️handles/🎚️gauges/🦀️.rs", encoding="utf-8").read()
m = open(A + "/🧬️schema/📸️snapshot/🦀️.rs", encoding="utf-8").read()
structs = {}
for sm in re.finditer(r"pub struct (\w+)\s*\{(.*?)\n\}", m, re.S):
    structs[sm.group(1)] = dict(re.findall(r"pub (\w+):\s*([^,\n]+),", sm.group(2)))
for tm in re.finditer(r"pub const (\w+): &\[Gauge<(\w+)>\] = &\[(.*?)\n\];|pub const (\w+): &\[Gauge<(\w+)>\] = &\[(gauge!.*)\];", g, re.S):
    name, ty, body = (tm.group(1), tm.group(2), tm.group(3)) if tm.group(1) else (tm.group(4), tm.group(5), tm.group(6))
    for f in re.findall(r"\|r\| r\.(\w+)", body):
        t = structs.get(ty, {}).get(f)
        if t != "f64": print(name, ty, f, t)
print("done", sum(1 for _ in re.finditer(r"gauge!\(", g)))
