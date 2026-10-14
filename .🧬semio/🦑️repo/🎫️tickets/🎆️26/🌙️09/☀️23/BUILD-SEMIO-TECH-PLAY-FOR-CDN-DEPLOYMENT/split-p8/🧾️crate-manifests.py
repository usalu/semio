#!/usr/bin/env python3
"""🧾️ Writes the Cargo manifests of every layer crate a `✂️crate-cut.py apply` produced, from the monolith manifest.

Each new crate gets the monolith's package fields, its `[dependencies]`/`[dev-dependencies]` rows (path rows
re-relativized) pruned to the crate idents its own mounted sources name, plus one workspace row per layer crate it
references (`🗑️generated/<crate>-layer-edges.json`: lib edges to `[dependencies]`, test-only edges to
`[dev-dependencies]`). The monolith (aggregator) manifest gains its layer edges and is pruned the same way. The
artifact workspace root gains the members (sorted) and one `[workspace.dependencies.<crate>]` row per new crate.

usage: 🧾️crate-manifests.py <config.json> plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CENSUS = os.path.join(HERE, "🕸️module-graph.py")
census = {}
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), census)
Crate = census["Crate"]


def section(text, name):
    m = re.search(r"(?ms)^\[" + re.escape(name) + r"\]\s*\n(.*?)(?=^\[|\Z)", text)
    return m.group(1) if m else ""


def rows(body):
    out, current = [], None
    for line in body.splitlines():
        if re.match(r"^[A-Za-z0-9_-]+\s*=", line):
            current = [line]
            out.append(current)
        elif current is not None and line.strip() and not line.startswith("#"):
            current.append(line)
    return [("\n".join(r), re.match(r"^([A-Za-z0-9_-]+)", r[0]).group(1)) for r in out]


def relocate(row, old_dir, new_dir):
    def fix(m):
        return f'path = "{os.path.relpath(os.path.normpath(os.path.join(old_dir, m.group(1))), new_dir)}"'
    return re.sub(r'path\s*=\s*"([^"]+)"', fix, row)


def idents_used(files):
    text = "\n".join(open(f, encoding="utf-8").read() for f in files)
    return set(re.findall(r"\b[a-z][a-z0-9_]*\b", text)), text


def main():
    config_path, mode = sys.argv[1], sys.argv[2]
    config = json.load(open(config_path, encoding="utf-8"))
    artifact = os.path.join(REPO, config["artifact"])
    monolith_manifest = os.path.join(REPO, config["manifest"]) if "manifest" in config else os.path.join(artifact, "📦️packages", "🦀️rust", "Cargo.toml")
    mono = open(monolith_manifest, encoding="utf-8").read()
    mono_dir = os.path.dirname(monolith_manifest)
    ws_pointer = re.search(r'(?m)^workspace\s*=\s*"([^"]+)"', mono).group(1)
    ws_root = os.path.normpath(os.path.join(mono_dir, ws_pointer, "Cargo.toml"))
    edges = json.load(open(os.path.join(HERE, "🗑️generated", config["crate"] + "-layer-edges.json")))
    packages = {layer: spec["package"] for layer, spec in config["crates"].items()}
    packages[config["aggregator"]] = config["crate"].replace("_", "-")
    deps, devs = rows(section(mono, "dependencies")), rows(section(mono, "dev-dependencies"))
    header_tail = re.search(r"(?ms)^\[package\.metadata\.semio\].*?(?=^\[lib\])", mono)
    new_members = []
    for layer, spec in config["crates"].items():
        crate_dir = os.path.join(artifact, spec["dir"])
        root = os.path.join(crate_dir, "🦀️.rs")
        files = list(Crate(root).files)
        used, _ = idents_used(files)
        lib_edges = [packages[d] for d in edges.get(layer, {}).get("lib", [])]
        dev_edges = [packages[d] for d in edges.get(layer, {}).get("dev", []) if packages[d] not in lib_edges]
        keep = lambda name: name.replace("-", "_") in used
        dep_rows = [relocate(r, mono_dir, crate_dir) for r, n in deps if keep(n) and n not in lib_edges]
        dev_rows = [relocate(r, mono_dir, crate_dir) for r, n in devs if keep(n) and n not in lib_edges + dev_edges]
        text = "[package]\n"
        text += f'workspace = "{os.path.relpath(os.path.dirname(ws_root), crate_dir)}"\n'
        text += f'name = "{spec["package"]}"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n'
        text += f'description = "{spec.get("description", spec.get("doc", spec["package"]).lstrip("/! ").replace(chr(34), chr(39)).replace("`", ""))}"\n\n'
        text += (header_tail.group(0).rstrip() + "\n\n") if header_tail else ""
        text += '[lib]\npath = "🦀️.rs"\n\n[dependencies]\n' + "".join(r + "\n" for r in dep_rows) + "".join(f"{p} = {{ workspace = true }}\n" for p in lib_edges)
        if dev_rows or dev_edges:
            text += "\n[dev-dependencies]\n" + "".join(r + "\n" for r in dev_rows) + "".join(f"{p} = {{ workspace = true }}\n" for p in dev_edges)
        print(f"== {os.path.relpath(crate_dir, REPO)}/Cargo.toml lib-edges={lib_edges} dev-edges={dev_edges} deps={len(dep_rows)} dev={len(dev_rows)}")
        existing = os.path.join(crate_dir, "Cargo.toml")
        if os.path.exists(existing):
            tests = re.findall(r"(?ms)^\[\[test\]\]\n.*?(?=^\[|\Z)", open(existing, encoding="utf-8").read())
            text = text.rstrip("\n") + "\n" + "".join("\n" + t.rstrip("\n") + "\n" for t in tests)
        if mode == "apply":
            open(existing, "w", encoding="utf-8").write(text)
        new_members.append((spec["package"], crate_dir))
    agg = config["aggregator"]
    root = os.path.normpath(os.path.join(mono_dir, re.search(r'(?ms)^\[lib\].*?^path\s*=\s*"([^"]+)"', mono).group(1)))
    used, _ = idents_used(Crate(root).files)
    lib_edges = [packages[d] for d in edges.get(agg, {}).get("lib", [])]
    dev_edges = [packages[d] for d in edges.get(agg, {}).get("dev", []) if packages[d] not in lib_edges]
    text = mono
    for name, rowset, sect in (("dependencies", deps, "dependencies"), ("dev-dependencies", devs, "dev-dependencies")):
        body = section(text, sect)
        new_body = body
        for row, n in rowset:
            if n.replace("-", "_") not in used:
                new_body = new_body.replace(row + "\n", "", 1)
        add = lib_edges if sect == "dependencies" else dev_edges
        new_body = new_body.rstrip("\n") + "\n" + "".join(f"{p} = {{ workspace = true }}\n" for p in add if not re.search(r"(?m)^" + re.escape(p) + r"\s*=", body))
        text = text.replace(body, new_body + ("\n" if body.endswith("\n\n") else ""), 1) if body else text
    print(f"== aggregator {os.path.relpath(monolith_manifest, REPO)} lib-edges={lib_edges} dev-edges={dev_edges}")
    if mode == "apply":
        open(monolith_manifest, "w", encoding="utf-8").write(text)
    ws = open(ws_root, encoding="utf-8").read()
    members = re.search(r"(?ms)^members\s*=\s*\[(.*?)\]", ws)
    listed = re.findall(r'"([^"]+)"', members.group(1))
    for name, crate_dir in new_members:
        rel = os.path.relpath(crate_dir, os.path.dirname(ws_root))
        if rel not in listed:
            listed.append(rel)
        if not re.search(r"(?m)^\[workspace\.dependencies\." + re.escape(name) + r"\]", ws) and not re.search(r"(?m)^" + re.escape(name) + r"\s*=", ws):
            ws = ws.rstrip("\n") + f'\n\n[workspace.dependencies.{name}]\npath = "{rel}"\n'
    ws = ws[:members.start(1)] + "\n" + "".join(f'    "{m}",\n' for m in sorted(listed)) + ws[members.end(1):]
    print(f"== workspace {os.path.relpath(ws_root, REPO)} members={len(listed)}")
    if mode == "apply":
        open(ws_root, "w", encoding="utf-8").write(ws)


main()
