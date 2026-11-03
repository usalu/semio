"""Registers the EN 1996 masonry checks crate split out of the schema crate (source budget). Idempotent."""
import os, re, json
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))
N = "semio-s-artifact-norm-en1996-masonry"
REL = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/📦️packages/🦀️rust"
p = f"{ROOT}/Cargo.toml"; s = rd(p)
m = f'    "{REL}",\n'
if m not in s:
    s = s.replace('    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust",\n', m + '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust",\n')
b = f'[workspace.dependencies.{N}]\npath = "{REL}"\n\n'
if b not in s:
    a = '[workspace.dependencies.semio-s-artifact-norm-en1996-model]'
    assert a in s
    s = s.replace(a, b + a)
wr(p, s)
def nm(x): return re.search(r'name = "([^"]+)"', x).group(1)
for lk in (f"{ROOT}/Cargo.lock", "/home/deck/semio/🌎️hub/Cargo.lock", "/home/deck/semio/✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock", "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.lock"):
    t = rd(lk)
    parts = re.split(r"(?=\[\[package\]\]\n)", t); head, pk = parts[0], parts[1:]
    if any(nm(x) == N for x in pk):
        continue
    for i, x in enumerate(pk):
        if nm(x) == "semio-s-artifact-norm-en1996-schema":
            mm = re.search(r"dependencies = \[\n(.*?)\]\n", x, re.S)
            deps = sorted(set(re.findall(r' "([^"]+)",', mm.group(1))) | {N})
            pk[i] = x[: mm.start(1)] + "".join(f' "{d}",\n' for d in deps) + x[mm.end(1):]
    blk = f'[[package]]\nname = "{N}"\nversion = "0.1.0"\ndependencies = [\n "semio-s-artifact-norm-en1996-model",\n "semio-s-plugin-norm-compliance",\n]\n\n'
    k = next((j for j, x in enumerate(pk) if nm(x) > N), len(pk))
    pk.insert(k, blk)
    wr(lk, head + "".join(pk))
p = "/home/deck/semio/📋️build-owners.json"; d = jload(p)
if not any(o["name"] == N for o in d["owners"]):
    o = {"name": N, "root": f"{ROOT.split('/home/deck/semio/')[1]}/{REL}", "manifest": f"{ROOT.split('/home/deck/semio/')[1]}/{REL}/Cargo.toml", "packages": []}
    i = next(k for k, x in enumerate(d["owners"]) if x["name"] == "semio-s-artifact-norm-en1996-model")
    d["owners"].insert(i, o)
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")
p = "/home/deck/semio/🔒️dependencies.json"; s = rd(p)
base = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/"
line = f'        "{base}🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml",\n'
new = f'        "{base}{REL}/Cargo.toml",\n'
if new not in s:
    assert line in s
    s = s.replace(line, new + line, 1)
    wr(p, s)
print("ok")
