"""➕ OHF dependency adder: appends `<name> = { path = "<relative>" }` to the [dependencies] table of a crate manifest (sorted insert is not attempted). usage: adddep.py <pkg-to-edit> <dep-pkg>"""
import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio/"
GEN = ROOT + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
idx = json.load(open(GEN + "/ohf-manifest-index.json", encoding="utf-8"))
args = [x for x in sys.argv[1:] if not x.startswith("--")]
a, b = args[:2]
dev = "--dev" in sys.argv
key = args[2] if len(args) > 2 else b
pa, pb = ROOT + idx[a], ROOT + idx[b]
t = open(pa, encoding="utf-8").read()
rel = os.path.relpath(os.path.dirname(pb), os.path.dirname(pa))
line = (f'{key} = {{ path = "{rel}" }}\n' if key == b else f'{key} = {{ path = "{rel}", package = "{b}" }}\n')
assert f"\n{key} = " not in t, "already declared"
tbl = "dev-dependencies" if dev else "dependencies"
m = re.search(r"(?m)^\[%s\]\n" % tbl, t)
if not m:
    t = t.rstrip("\n") + "\n\n[%s]\n" % tbl
    m = re.search(r"(?m)^\[%s\]\n" % tbl, t)
end = re.search(r"(?m)^\[", t[m.end():])
pos = m.end() + (end.start() if end else len(t) - m.end())
body = t[m.end():pos]
if not body.endswith("\n"):
    body += "\n"
t = t[:m.end()] + body.rstrip("\n") + "\n" + line + body[len(body.rstrip("\n")) + 1:] + t[pos:]
open(pa, "w", encoding="utf-8").write(t)
print("added", line.strip(), "to", idx[a])
