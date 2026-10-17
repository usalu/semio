"""➕️ Adds the new Semio content binding crate as a workspace dependency of the rewriting artifact: workspace table, every member manifest and the lock file. usage: add_dependency.py [--apply]"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rename_crates import resort_lock
R = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting"
NEW = "semio-s-artifact-trinity-jack-semio-content"
apply = "--apply" in sys.argv
members = []
for base, dirs, files in os.walk(R):
    dirs[:] = [d for d in dirs if d not in ("node_modules", "target")]
    if "Cargo.toml" in files and base != R:
        members.append(os.path.join(base, "Cargo.toml"))
edited = []
for m in sorted(members):
    t = open(m, encoding="utf8").read()
    line = "semio-s-artifact-trinity-jack = { workspace = true }\n"
    if t.count(line) != 1 or NEW in t:
        continue
    edited.append((m, t.replace(line, line + NEW + " = { workspace = true }\n")))
root = os.path.join(R, "Cargo.toml")
rt = open(root, encoding="utf8").read()
blk = '[workspace.dependencies.semio-s-artifact-trinity-jack]\npath = "../🔌️jack/📦️packages/🦀️rust"\n'
assert rt.count(blk) == 1 and NEW not in rt
rt = rt.replace(blk, blk + f'\n[workspace.dependencies.{NEW}]\npath = "../🔌️jack/🪆️content/🧿️semio/📦️packages/🦀️rust"\n')
lock = os.path.join(R, "Cargo.lock")
lt = open(lock, encoding="utf8").read()
assert resort_lock(lt) == lt and f'name = "{NEW}"' not in lt
names = [os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(m)))) for m, _ in edited]
crate_names = []
for m, _ in edited:
    crate_names.append(re.search(r'^name = "([^"]+)"', open(m, encoding="utf8").read(), re.M).group(1))
for cn in crate_names:
    rx = re.compile(r'(\[\[package\]\]\nname = "%s"\nversion = "[^"]+"\ndependencies = \[\n)((?: "[^"]+",\n)+)(\])' % re.escape(cn))
    m = rx.search(lt)
    assert m, cn
    lt = lt[:m.start(2)] + m.group(2) + f' "{NEW}",\n' + lt[m.end(2):]
deps = ["semio-framework-artifact-reference", "semio-framework-graph-manifest", "semio-framework-os-kernel", "semio-framework-pack", "semio-framework-plugin", "semio-framework-schema-composition", "semio-framework-value", "semio-s-artifact-stdio-semio-base", "semio-s-artifact-stdio-semio-graph", "semio-s-artifact-stdio-semio-value-model", "semio-s-artifact-trinity-jack", "semio-s-artifact-trinity-jack-graph-query"]
for d in deps:
    assert re.search(r'^name = "%s"$' % re.escape(d), lt, re.M), d
lt = lt.rstrip("\n") + f'\n\n[[package]]\nname = "{NEW}"\nversion = "0.1.0"\ndependencies = [\n' + "".join(f' "{d}",\n' for d in deps) + "]\n"
lt = resort_lock(lt)
print("crates:", crate_names)
print("manifests:", len(edited))
if apply:
    for m, t in edited:
        open(m, "w", encoding="utf8").write(t)
    open(root, "w", encoding="utf8").write(rt)
    open(lock, "w", encoding="utf8").write(lt)
    print("APPLIED")
