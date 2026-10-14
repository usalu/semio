#!/usr/bin/env python3
"""🧾️ Writes the manifest of each configured crate from its sources (P5 split lane).

Normal dependencies = the crates its lib closure names; dev-dependencies = the extra crates its test closure names
(`#[cfg(test)]` mounts and `🧪️tests` paths). Identifiers resolve through the configured alias table and a package
registry (the root `Cargo.toml` workspace dependencies plus every package manifest under the configured roots).
Every dependency is `{ workspace = true }`; missing rows are added to the crate's workspace manifest.

usage: 🧾️crate-manifests.py <config.json> plan|apply
config: {"workspace": <workspace Cargo.toml>, "crates": [{"name", "dir", "lib", "role", "description",
         "features": {pkg: [..]}, "dev_features": {pkg: [..]}, "extra": [pkg], "extra_dev": [pkg], "tests": [[name, path]],
         "metadata": [raw toml lines]}], "aliases": {ident: pkg}, "registry_roots": [..]}
"""
import json, os, re, sys, glob
REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
src = open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0]
NS = {}
exec(compile(src, "module-graph", "exec"), NS)
Crate = NS["Crate"]
EXTERNAL = {"serde_json", "serde", "csv", "blake3"}
IDENT = re.compile(r"(?<![\w])(semio_[a-z0-9_]+|serde_json|serde|csv|blake3|protocol|store|dsl|vcs|replication|pack|machine)\s*::|owner\s*=\s*(semio_[a-z0-9_]+)")
EXTERN = re.compile(r"extern\s+crate\s+(\w+)(?:\s+as\s+(\w+))?\s*;")

def registry(roots):
    cache = os.path.join(HERE, "🗑️generated", "registry.json")
    if os.path.exists(cache) and os.environ.get("FRESH_REGISTRY") != "1":
        reg = json.load(open(cache, encoding="utf-8"))
        for r in roots:
            for path in glob.glob(os.path.join(REPO, r, "**", "📦️packages", "🦀️rust", "Cargo.toml"), recursive=True) if not r.startswith("🧰️") else []:
                m = re.search(r'(?m)^name\s*=\s*"([^"]+)"', open(path, encoding="utf-8").read())
                if m: reg[m.group(1)] = os.path.dirname(path)
        return reg
    reg = {}
    root = open(os.path.join(REPO, "Cargo.toml"), encoding="utf-8").read()
    for m in re.finditer(r'(?m)^([\w-]+)\s*=\s*\{\s*path\s*=\s*"([^"]+)"', root):
        reg[m.group(1)] = os.path.join(REPO, m.group(2))
    for r in roots:
        for path in glob.glob(os.path.join(REPO, r, "**", "Cargo.toml"), recursive=True):
            if "/target/" in path or "node_modules" in path: continue
            m = re.search(r'(?m)^name\s*=\s*"([^"]+)"', open(path, encoding="utf-8").read())
            if m and "[package]" in open(path, encoding="utf-8").read():
                reg.setdefault(m.group(1), os.path.dirname(path))
    json.dump(reg, open(cache, "w", encoding="utf-8"), ensure_ascii=False)
    return reg

RENAMES = {"pack": "semio-framework-pack", "replication": "semio-framework-replication", "machine": "semio-framework-machine"}

def closure_names(crate, files):
    names = set()
    for f in files:
        text = crate.text[f]
        names |= {a or b for a, b in IDENT.findall(text)}
        for m in EXTERN.finditer(text):
            names.add(m.group(1))
    return names

def main():
    cfg = json.load(open(sys.argv[1], encoding="utf-8")); mode = sys.argv[2]
    aliases = {"protocol": "semio-framework-os-kernel", "store": "semio-framework-os-kernel", "dsl": "semio-framework-os-kernel", "vcs": "semio-framework-os-kernel"}
    aliases.update(cfg.get("aliases", {}))
    reg = registry(cfg.get("registry_roots", []))
    for c in cfg["crates"]: reg[c["name"]] = os.path.join(REPO, c["dir"])
    ws_path = os.path.join(REPO, cfg["workspace"]); ws_dir = os.path.dirname(ws_path)
    rows = {}
    for c in cfg["crates"]:
        d = os.path.join(REPO, c["dir"]); lib = os.path.normpath(os.path.join(d, c.get("lib", "../../🦀️.rs")))
        crate = Crate(lib)
        test_files = {f for f in crate.files if f in crate.test_files or "🧪️tests" in os.path.relpath(f, os.path.dirname(lib))}
        lib_files = [f for f in crate.files if f not in test_files]
        def pkgs(names):
            out = set()
            for n in names:
                if n in RENAMES: out.add(n); continue
                p = aliases.get(n, n.replace("_", "-") if n not in EXTERNAL else n)
                if p == c["name"]: continue
                if p in EXTERNAL or p in reg: out.add(p)
            return out
        deps = pkgs(closure_names(crate, lib_files)) | set(c.get("extra", []))
        dev = (pkgs(closure_names(crate, test_files)) | set(c.get("extra_dev", []))) - deps
        for t in c.get("tests", []):
            tc = Crate(os.path.normpath(os.path.join(d, t[1])))
            dev |= pkgs(closure_names(tc, list(tc.files))) - deps - {c["name"]}
            dev.add(c["name"]) if False else None
        manifest_dir = d
        lines = ["[package]", f'workspace = "{os.path.relpath(ws_dir, manifest_dir)}"', f'name = "{c["name"]}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", f'description = "{c["description"]}"', "",
                 "[package.metadata.semio]", f'role = "{c.get("role", "library")}"'] + c.get("metadata", []) + ["", "[lints]", "workspace = true", "", "[lib]", f'path = "{os.path.relpath(lib, manifest_dir)}"']
        for t in c.get("tests", []):
            lines += ["", "[[test]]", f'name = "{t[0]}"', f'path = "{t[1]}"']
        raw = c.get("raw", {})
        def row(p, feats):
            if p in raw: return raw[p]
            if p in EXTERNAL and p not in ("serde_json", "serde"):
                return f'{p} = {{ workspace = true }}'
            f = feats.get(p)
            return f'{p} = {{ workspace = true{", features = " + json.dumps(f) if f else ""} }}'
        lines += ["", "[dependencies]"] + [row(p, c.get("features", {})) for p in sorted(deps)]
        if dev:
            lines += ["", "[dev-dependencies]"] + [row(p, c.get("dev_features", {})) for p in sorted(dev)]
        text = "\n".join(lines) + "\n"
        for p in deps | dev:
            if p in raw: continue
            if p in EXTERNAL: rows.setdefault(p, None)
            elif p in RENAMES: rows.setdefault(p, (os.path.relpath(reg[RENAMES[p]], ws_dir), RENAMES[p]))
            else: rows.setdefault(p, os.path.relpath(reg[p], ws_dir))
        print(f"== {c['name']}: deps={sorted(deps)}\n   dev={sorted(dev)}")
        if mode == "apply":
            os.makedirs(manifest_dir, exist_ok=True)
            open(os.path.join(manifest_dir, "Cargo.toml"), "w", encoding="utf-8").write(text)
    ws = open(ws_path, encoding="utf-8").read()
    add = []
    versions = {"serde_json": 'version = "1.0.149"\nfeatures = [ "raw_value" ]', "serde": 'version = "1.0.228"\nfeatures = [ "derive" ]', "csv": 'version = "1"', "blake3": 'version = "1.8"'}
    for p, path in sorted(rows.items()):
        if re.search(r"(?m)^\[workspace\.dependencies\." + re.escape(p) + r"\]", ws) or re.search(r"(?m)^" + re.escape(p) + r"\s*=", ws.split("[workspace.dependencies]")[-1] if "[workspace.dependencies]" in ws else ""):
            continue
        body = versions[p] if path is None else (f'path = "{path[0]}"\npackage = "{path[1]}"' if isinstance(path, tuple) else f'path = "{path}"')
        add.append(f"[workspace.dependencies.{p}]\n{body}\n")
    print("workspace rows to add:", [a.split("]")[0][24:] for a in add])
    if mode == "apply" and add:
        ws = open(ws_path, encoding="utf-8").read()
        anchor = re.search(r"(?m)^\[workspace\.lints\.rust\]", ws).start()
        ws = ws[:anchor] + "\n".join(add) + "\n" + ws[anchor:]
        open(ws_path, "w", encoding="utf-8").write(ws)

main()
