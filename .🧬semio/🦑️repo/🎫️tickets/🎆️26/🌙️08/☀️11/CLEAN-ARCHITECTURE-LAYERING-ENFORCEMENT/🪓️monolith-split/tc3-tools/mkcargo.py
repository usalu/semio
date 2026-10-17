"""📦 Write a companion crate (Cargo.toml + 📋️project.json) next to <module>/🧪️tests/<emoji>️companion.
usage: mkcargo.py <spec.json>
spec: {parent_manifest, companion_dir, name, id, role, description, project, features:[..], carry:{"dev":[names from parent [dependencies]/[dev-dependencies]], "extra":[raw lines]}, parent_dep_path_rel?}
`workspace` is computed as the relative path from the companion package dir to the parent's `workspace` target; every carried path dependency is re-relativized; the crate under test is the single normal dependency."""
import json, os, re, sys
R = "/Users/ueli/Documents/semio/"
spec = json.load(open(sys.argv[1], encoding="utf-8"))
pm = R + spec["parent_manifest"]
pdir = os.path.dirname(pm)
ptext = open(pm, encoding="utf-8").read()
pkg_dir = R + spec["companion_dir"] + "/📦️packages/🦀️rust"
ws_rel = re.search(r'^workspace\s*=\s*"([^"]+)"', ptext, re.M).group(1)
ws_abs = os.path.normpath(os.path.join(pdir, ws_rel))
ws_new = os.path.relpath(ws_abs, pkg_dir)
parent_name = re.search(r'^name\s*=\s*"([^"]+)"', ptext, re.M).group(1)


def section(name):
    m = re.search(r"^\[" + re.escape(name) + r"\]\n(.*?)(?=^\[|\Z)", ptext, re.S | re.M)
    return m.group(1) if m else ""


def dep_lines(sec):
    out = {}
    for l in sec.split("\n"):
        m = re.match(r"^([A-Za-z0-9_\-]+)\s*=\s*(.*)$", l)
        if m:
            out[m.group(1)] = m.group(2)
    return out


deps = {**dep_lines(section("dependencies")), **dep_lines(section("dev-dependencies"))}


def rerel(spec_text):
    def sub(m):
        abs_ = os.path.normpath(os.path.join(pdir, m.group(1)))
        return 'path = "' + os.path.relpath(abs_, pkg_dir) + '"'
    return re.sub(r'path\s*=\s*"([^"]+)"', sub, spec_text)


me = os.path.relpath(pdir, pkg_dir)
lines = [
    "[package]", f'workspace = "{ws_new}"', f'name = "{spec["name"]}"', 'version = "0.1.0"', 'edition = "2021"', "rust-version.workspace = true",
    f'description = "{spec["description"]}"', "publish = false", "",
    "[package.metadata.semio]", f'role = "{spec["role"]}"', f'id = "{spec["id"]}"', "",
    "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
    "[lints]", "workspace = true", "",
    "[lib]", 'path = "../../🦀️.rs"', "",
    "[dependencies]",
    f'{parent_name} = {{ path = "{me}"' + (f', features = [{", ".join(chr(34) + f + chr(34) for f in spec.get("features", []))}]' if spec.get("features") else "") + " }",
    "", "[dev-dependencies]",
]
for n in spec["carry"]:
    if n not in deps:
        raise SystemExit(f"{n} is not declared by the parent manifest")
    lines.append(f"{n} = {rerel(deps[n])}")
lines += spec.get("extra", [])
os.makedirs(pkg_dir, exist_ok=True)
open(pkg_dir + "/Cargo.toml", "w", encoding="utf-8").write("\n".join(lines) + "\n")
open(pkg_dir + "/📋️project.json", "w", encoding="utf-8").write(json.dumps({"name": spec["project"], "tags": ["scope:framework", "language:rust"]}, indent=2, ensure_ascii=False) + "\n")
print(open(pkg_dir + "/Cargo.toml", encoding="utf-8").read())
