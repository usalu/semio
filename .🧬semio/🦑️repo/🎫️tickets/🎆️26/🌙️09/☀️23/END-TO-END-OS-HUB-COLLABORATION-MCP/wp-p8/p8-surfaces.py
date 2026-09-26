"""🧭️ P8 surface discovery: every `impl ArtifactEditor|ArtifactViewer for X` in the plugin artifact crates with its
module path, its `create_*` definition fn in the same file, its `Members` type and the owning crate — the input of
the ticket-local declared-verb probe harness (`wp-p8/probe-harness`)."""
import os, re, json, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
PLUGINS = os.path.join(ROOT, "✏️s/🔌️plugins")
out = subprocess.run(["/usr/bin/grep", "-rlE", "--include=🦀️.rs", r"impl (semio_framework_plugin::)?Artifact(Editor|Viewer) for ", PLUGINS], capture_output=True, text=True).stdout.split("\n")
rows = []
for path in sorted(p for p in out if p and "/target/" not in p and "🧪️tests" not in p):
    src = open(path, encoding="utf-8").read()
    m = re.search(r"impl (?:semio_framework_plugin::)?Artifact(Editor|Viewer) for (\w+)", src)
    if not m:
        continue
    role, ty = m.group(1), m.group(2)
    members = re.search(r"type Members\s*=\s*([\w:]+)\s*;", src)
    create = re.findall(r"pub fn (create_\w+)\(\)\s*->\s*(?:semio_framework_plugin::)?AppDefinition", src)
    crate_dir = path
    while crate_dir != PLUGINS and not os.path.isdir(os.path.join(crate_dir, "📦️packages")):
        crate_dir = os.path.dirname(crate_dir)
    cargo = os.path.join(crate_dir, "📦️packages/🦀️rust/Cargo.toml")
    name = re.search(r'^name = "([^"]+)"', open(cargo, encoding="utf-8").read(), re.M).group(1) if os.path.exists(cargo) else None
    rows.append({"role": role, "type": ty, "members": members.group(1) if members else None, "create": create, "crate": name, "file": os.path.relpath(path, ROOT)})
json.dump(rows, open(sys.argv[1], "w"), indent=1, ensure_ascii=False)
print(len(rows), "surfaces;", sum(1 for r in rows if not r["create"]), "without create fn in file;", sum(1 for r in rows if not r["crate"]), "without crate")
for r in rows:
    if not r["create"] or not r["crate"]:
        print(" ", r["role"], r["type"], r["crate"], r["file"][-90:])
