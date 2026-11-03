"""Checks every gauge closure `|x| x.field` of the en1996 gauge tables against the field type of the snapshot struct."""
import re, glob
A = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any"
g = open(A + "/🎬️scene/🕹️handles/🎚️gauges/🦀️.rs", encoding="utf-8").read()
m = open(A + "/🧬️schema/📸️snapshot/🦀️.rs", encoding="utf-8").read()
structs = {}
for sm in re.finditer(r"pub struct (\w+)\s*\{(.*?)\n\}", m, re.S):
    structs[sm.group(1)] = dict(re.findall(r"pub (\w+):\s*([^,\n]+),", sm.group(2)))
for tm in re.finditer(r"pub const (\w+): &\[Gauge<(\w+)>\] = &\[(.*?)\];\n", g, re.S):
    name, ty, body = tm.groups()
    if ty not in structs: print("NOSTRUCT", ty)
    for f in re.findall(r"\|\w\| (?:f64::from\()?\w\.(\w+)", body):
        t = structs.get(ty, {}).get(f)
        if t != "f64": print(name, ty, f, t)
