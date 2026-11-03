"""en1998 handles `Scalar { .. }` literals: numeric literals must be floats and every `value:` accessor must be an f64 field of some snapshot struct."""
import re
A = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"
h = open(A + "/🎬️scene/🕹️handles/🦀️.rs", encoding="utf-8").read()
m = open(A + "/🧬️schema/📸️snapshot/🦀️.rs", encoding="utf-8").read()
types = {}
for sm in re.finditer(r"pub struct (\w+)\s*\{(.*?)\n\}", m, re.S):
    for f, t in re.findall(r"pub (\w+):\s*([^,\n]+),", sm.group(2)): types.setdefault(f, set()).add(t.strip())
n = 0
for lit in re.finditer(r"Scalar \{ ([^}]*) \}", h):
    n += 1
    body = lit.group(1)
    for key in ("floor", "lo", "hi", "step"):
        v = re.search(rf"\b{key}: ([^,]+),", body).group(1).strip()
        if not re.fullmatch(r"-?[\d_]+(\.[\d_]+)?(e-?\d+)?", v) or not re.search(r"[.e]", v): print("NONFLOAT", key, v, body[:60])
    acc = re.search(r"value: \w+\.(\w+),", body)
    if acc and types.get(acc.group(1)) != {"f64"}: print("TYPE", acc.group(1), types.get(acc.group(1)))
print("scalars", n)
