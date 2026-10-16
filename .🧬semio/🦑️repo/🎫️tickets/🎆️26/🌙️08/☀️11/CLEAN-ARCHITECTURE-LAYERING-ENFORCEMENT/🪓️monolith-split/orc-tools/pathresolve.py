"""Static name resolution of every `<root>::record::<name>` and `<root>::record::scalar_witness::<name>` token in the repo against the exports of the framework pack facade (orc).
Reads the facade `pub mod record { pub use ...::{...}; }` block and the pub items of the scalar-witness module; reports any token that does not resolve."""
import re, sys
from orclib import *
facade = read(FW + "/📦️packages/🦀️rust/🦀️.rs")
m = re.search(r"pub mod record \{(.*?)\n\}", facade, re.S)
block = m.group(1)
exports = set()
for um in re.finditer(r"pub use [\w:]+::\{([^}]*)\};|pub use [\w:]+::(\w+);", block):
    names = um.group(1) if um.group(1) is not None else um.group(2)
    for n in names.split(","):
        n = n.strip()
        if n: exports.add(re.split(r"\s+as\s+", n)[-1].strip())
sw = read(FW + "/🌱️value/🔎️scalar-witness/🦀️.rs")
sw_items = set(re.findall(r"^\s*pub (?:struct|enum|fn|trait|type|const)\s+(\w+)", sw, re.M))
files = []
for n in ("tracked", "untracked"):
    files += [x for x in open(G + "/orc-rs-%s.z" % n, "rb").read().decode().split("\0") if x]
HEAD = r"(?<![\w])(?:(?:[A-Za-z_]\w*::)*(?:semio_framework_os_pack|os_pack)|pack)::record::"
single = re.compile(HEAD + r"(\w+)(?:::(\w+))?")
braced = re.compile(HEAD + r"\{([^}]*)\}")
mutation = None
if "--mutate" in sys.argv:
    mutation = sys.argv[sys.argv.index("--mutate") + 1].split("@@")
bad = []; checked = 0; per = {}
for f in files:
    if f.startswith(".🧬semio/") or "/🗑️generated/" in f: continue
    try: t = open(ROOT + "/" + f, errors="replace").read()
    except Exception: continue
    if mutation and f.endswith(mutation[0]) and mutation[1] in t:
        t = t.replace(mutation[1], mutation[2], 1)
        print("   (in-memory mutation applied to", f[-60:] + ")")
    if "record::" not in t: continue
    for mm in single.finditer(t):
        a, b = mm.group(1), mm.group(2)
        line = t.count("\n", 0, mm.start()) + 1
        checked += 1
        if a == "scalar_witness":
            if b and b not in sw_items: bad.append((f, line, "scalar_witness::" + b))
        elif a not in exports: bad.append((f, line, a))
    for mm in braced.finditer(t):
        for n in mm.group(1).split(","):
            n = n.strip()
            if not n: continue
            n0 = re.split(r"\s+as\s+", n)[0].strip()
            checked += 1
            if n0.startswith("scalar_witness"):
                continue
            if n0.split("::")[0] not in exports: bad.append((f, t.count("\n", 0, mm.start()) + 1, n0))
print("facade record exports:", len(exports), "scalar-witness items:", sorted(sw_items))
print("tokens checked:", checked, "unresolved:", len(bad))
for b in bad[:40]: print("  UNRESOLVED", b[0][-90:], b[1], b[2])
