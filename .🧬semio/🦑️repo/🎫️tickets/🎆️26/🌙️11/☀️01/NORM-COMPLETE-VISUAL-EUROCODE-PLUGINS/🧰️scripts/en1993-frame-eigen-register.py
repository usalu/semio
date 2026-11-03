"""Idempotent: registers the sibling crate semio-s-artifact-norm-en1993-frame-eigen (checks/🕸️frame-eigen) in the artifact workspace, the checks-special manifest, Cargo.lock, script.ts and build-owners.json."""
import json, re
R = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
CH = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
REL = f"{CH}/🕸️frame-eigen/📦️packages/🦀️rust"
NAME = "semio-s-artifact-norm-en1993-frame-eigen"
rd = lambda p: open(p, encoding="utf8").read()
def wr(p, t): open(p, "w", encoding="utf8").write(t)
t = rd(R + "/Cargo.toml")
m = f'    "{REL}",\n'
anchor = f'    "{CH}/🧮️clauses/📦️packages/🦀️rust",\n'
if m not in t: t = t.replace(anchor, anchor + m, 1)
h = f"[workspace.dependencies.{NAME}]"
if h not in t:
    a = f'[workspace.dependencies.semio-s-artifact-norm-en1993-clauses]\npath = "{CH}/🧮️clauses/📦️packages/🦀️rust"\n'
    assert a in t
    t = t.replace(a, a + f'\n{h}\npath = "{REL}"\n', 1)
wr(R + "/Cargo.toml", t)
p = f"{R}/{CH}/🔗️special/📦️packages/🦀️rust/Cargo.toml"
t = rd(p); line = f"{NAME} = {{ workspace = true }}\n"
if line not in t:
    t = t.replace("semio-s-artifact-norm-en1993-model = { workspace = true }\n", line + "semio-s-artifact-norm-en1993-model = { workspace = true }\n", 1)
wr(p, t)
p = R + "/📦️packages/🦀️rust/📜️script.ts"; t = rd(p)
if f'"{NAME}"' not in t:
    t = t.replace('"semio-s-artifact-norm-en1993-clauses",', f'"semio-s-artifact-norm-en1993-clauses","{NAME}",', 1)
wr(p, t)
p = R + "/Cargo.lock"; t = rd(p)
if f'name = "{NAME}"' not in t:
    pkg = f'[[package]]\nname = "{NAME}"\nversion = "0.1.0"\ndependencies = [\n "semio-s-artifact-norm-en1993-clauses",\n "semio-s-artifact-norm-en1993-model",\n "semio-s-plugin-norm-mechanics",\n]\n\n'
    names = re.findall(r'\[\[package\]\]\nname = "(semio-s-artifact-norm-en1993-[^"]*)"', t)
    nxt = next(n for n in sorted(names) if n > NAME)
    j = t.index(f'[[package]]\nname = "{nxt}"')
    t = t[:j] + pkg + t[j:]
mm = re.search(r'(name = "semio-s-artifact-norm-en1993-checks-special"\nversion = "0.1.0"\ndependencies = \[\n)(.*?)(\]\n)', t, re.S)
deps = [l.strip().strip(",").strip('"') for l in mm.group(2).splitlines()]
if NAME not in deps:
    deps = sorted(set(deps + [NAME]))
    t = t[:mm.start(2)] + "".join(f' "{d}",\n' for d in deps) + t[mm.end(2):]
wr(p, t)
p = "/home/deck/semio/📋️build-owners.json"; t = rd(p)
if NAME not in t:
    entry = f'    {{\n      "name": "{NAME}",\n      "root": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/{REL}",\n      "manifest": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/{REL}/Cargo.toml",\n      "packages": []\n    }},\n'
    k = t.index('    {\n      "name": "semio-s-artifact-norm-en1993-model"') if '"name": "semio-s-artifact-norm-en1993-model"' in t else None
    names = re.findall(r'"name": "(semio-s-artifact-norm-en1993-[^"]*)"', t)
    nxt = next(n for n in sorted(names) if n > NAME)
    k = t.index(f'    {{\n      "name": "{nxt}"')
    t = t[:k] + entry + t[k:]
    json.loads(t)
    wr(p, t)
print("registered", NAME)
