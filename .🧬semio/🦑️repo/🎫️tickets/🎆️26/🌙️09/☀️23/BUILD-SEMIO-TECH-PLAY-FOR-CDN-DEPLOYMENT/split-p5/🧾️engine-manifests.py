#!/usr/bin/env python3
"""🧾️ Writes the manifest of every fem engine crate (members of the `✏️s` workspace) from its sources: normal
dependencies are the crates the lib closure names, dev-dependencies the extra crates its test modules name. Adds the
missing `[workspace.dependencies]` rows to `✏️s/Cargo.toml` (re-read right before the write).

usage: 🧾️engine-manifests.py plan|apply
"""
import os, re, sys
REPO = "/Users/ueli/Documents/semio"
ENGINE = REPO + "/✏️s/🔨️modules/🏗️fem/⚙️engine"
WS = REPO + "/✏️s"
CRATES = [
    ("semio-s-fem-algebra", "➕️algebra", "Dense `MatD`/`VecD` linear algebra of the fem engine"),
    ("semio-s-fem-formulation", "➗️formulation", "Shape functions, quadrature and plane constitutive matrices of the fem engine"),
    ("semio-s-fem-model", "🏗️model", "FEM element contract: DOFs, nodes, loads, the Element trait and result records"),
    ("semio-s-fem-elements2d", "📏️elements2d", "2D finite elements: bars, Euler-Bernoulli beams, plane continua and DKT plates"),
    ("semio-s-fem-elements3d", "🧱️elements3d", "3D finite elements: bars, frames, tetrahedra, hexahedra and facet shells"),
    ("semio-s-fem-static", "⚖️static", "Linear static analysis over the closed fem element set"),
    ("semio-s-fem-sparse", "🔢️sparse", "Sparse storage, RCM ordering and the shared numerical job protocol"),
    ("semio-s-fem-sparse-ldlt", "🔢️sparse/🔺️ldlt", "Envelope sparse LDLT direct solver and its checkpointed job"),
    ("semio-s-fem-sparse-pcg", "🔢️sparse/🔁️pcg", "Jacobi-preconditioned conjugate-gradient solver and its checkpointed job"),
    ("semio-s-fem-sparse-subspace", "🔢️sparse/🎵️subspace", "Subspace-iteration generalized eigensolver and its checkpointed job"),
    ("semio-s-fem-mesh", "🕸️mesh", "Independent binary64 planar meshing and region volume surfaces"),
    ("semio-s-fem-mesh-derived", "🕸️mesh/🧬️derived", "Structured, quadratic and extruded meshes derived from planar triangulations"),
    ("semio-s-fem-analyses-model", "🧮️analyses/🗂️model", "Analysis model, load cases, combinations and the equation numbering of fem analyses"),
    ("semio-s-fem-analyses-assembly", "🧮️analyses/🧩️assembly", "Deterministic checkpointed sparse stiffness assembly of fem analyses"),
    ("semio-s-fem-analyses", "🧮️analyses", "Multi-case static, modal and buckling fem analyses and the fem job graph"),
    ("semio-s-fem-test-support", "🧪️tests/🧰️support", "Retained-payload test vectors shared by the fem engine contract tests (dev-dependency only)"),
]
FRAMEWORK = {
    "semio-framework-job": "../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
    "semio-framework-value": "../🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
    "semio-framework-value-derive": "../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
    "replication": "../🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
    "semio-framework-dispatch-macros": "../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust",
    "semio-framework-async": "../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust",
    "semio-framework-os-pack": "../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/📦️packages/🦀️rust",
}
EXTERNAL = {"serde_json": 'serde_json = { workspace = true }', "serde": 'serde = { workspace = true }'}
IDENT = re.compile(r"\b(semio_[a-z0-9_]+|serde_json|serde|replication)::")

def mounted(root, test):
    out = []
    src = open(root, encoding="utf-8").read()
    for m in re.finditer(r'((?:#\[cfg\(test\)\]\s*)?)#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\s+\w+\s*;', src):
        if bool(m.group(1)) == test:
            out.append(os.path.normpath(os.path.join(os.path.dirname(root), m.group(2))))
    return out

def names(files):
    found = set()
    for f in files:
        found |= set(IDENT.findall(open(f, encoding="utf-8").read()))
    return found

def pkg(ident):
    return ident.replace("_", "-")

def main():
    mode = sys.argv[1]
    known = {name for name, _, _ in CRATES}
    rows = {}
    for name, rel, desc in CRATES:
        d = os.path.join(ENGINE, rel)
        root = os.path.join(d, "🦀️.rs")
        lib = [root] + mounted(root, False)
        tests = mounted(root, True)
        own = name.replace("-", "_")
        deps = sorted(pkg(i) for i in names(lib) if pkg(i) != name and (pkg(i) in known or pkg(i) in FRAMEWORK))
        dev = sorted(pkg(i) for i in names(tests) if pkg(i) != name and pkg(i) not in deps and (pkg(i) in known or pkg(i) in FRAMEWORK))
        ext_dev = sorted(i for i in names(tests) if i in EXTERNAL)
        manifest_dir = os.path.join(d, "📦️packages", "🦀️rust")
        ws_rel = os.path.relpath(WS, manifest_dir)
        lines = ["[package]", f'workspace = "{ws_rel}"', f'name = "{name}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", f'description = "{desc}"', "",
                 "[package.metadata.semio]", 'role = "s-module"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', "", "[dependencies]"]
        lines += [f"{dep} = {{ workspace = true }}" for dep in deps]
        if dev or ext_dev:
            lines += ["", "[dev-dependencies]"] + [f"{dep} = {{ workspace = true }}" for dep in dev] + [EXTERNAL[e] for e in ext_dev]
        text = "\n".join(lines) + "\n"
        rows[name] = os.path.relpath(manifest_dir, WS)
        print(f"== {name}: deps={deps} dev={dev + ext_dev}")
        if mode == "apply":
            os.makedirs(manifest_dir, exist_ok=True)
            open(os.path.join(manifest_dir, "Cargo.toml"), "w", encoding="utf-8").write(text)
    for k, v in FRAMEWORK.items():
        if k != "replication": rows.setdefault(k, v)
    ws_path = os.path.join(WS, "Cargo.toml")
    ws = open(ws_path, encoding="utf-8").read()
    add = []
    for k, v in sorted(rows.items()):
        if f"[workspace.dependencies.{k}]" in ws: continue
        add.append(f'[workspace.dependencies.{k}]\npath = "{v}"\n')
    print("workspace rows to add:", len(add))
    if mode == "apply" and add:
        ws = open(ws_path, encoding="utf-8").read()
        anchor = ws.index("[workspace.dependencies.serde]")
        ws = ws[:anchor] + "\n".join(add) + "\n" + ws[anchor:]
        open(ws_path, "w", encoding="utf-8").write(ws)

main()
