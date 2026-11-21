"""🔎️ Compares the staged PDF roster against captured Cargo component source inputs."""

import ast
import json
import os
from pathlib import Path
import re
import sys
import unicodedata

root = Path.cwd()
ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
run = Path(sys.argv[1])
units = json.loads((run / "l/unit-graph-prototype-00.json").read_text())["units"]
base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"
source = (ticket / "📜️prepare-pdf-overlay.py").read_text()
tree = ast.parse(source)
paths = next(ast.literal_eval(node.value) for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "paths" for target in node.targets))
boundary = json.loads((generated / "boundary-draft-narrow/overlay.json").read_text())
image_report = (ticket / "image-authoring-stage.md").read_text()
image = re.findall(r"^\| `([^`]+)\` \|", image_report, re.MULTILINE)
requested = {unicodedata.normalize("NFC", os.path.normpath(path)) for path in [*boundary, *(f"{base}/{path}" for path in [*paths, *image])]}
requested.add("🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/📦️packages/🦀️rust/Cargo.toml")

def name(unit):
    return unit["pkg_id"].split("#")[-1].split("@")[0]

def closure(package):
    pending = [i for i, unit in enumerate(units) if name(unit) == package]
    assert len(pending) == 1, package
    seen = set()
    while pending:
        i = pending.pop()
        if i in seen:
            continue
        seen.add(i)
        pending.extend(dep["index"] for dep in units[i]["dependencies"])
    return {name(units[i]) for i in seen}, {os.path.normpath(units[i]["target"]["src_path"]): name(units[i]) for i in seen}

stdio, stdio_entries = closure("semio-hub-stdio-component")
pdf, pdf_entries = closure("semio-hub-stdio-pdf-component")
entries = {**stdio_entries, **pdf_entries}
found = {}
for path in Path(".cache/b/r/m/u/a").glob("*/*.json"):
    try:
        memo = json.loads(path.read_text())
    except (FileNotFoundError, ValueError):
        continue
    if "target_env=p2" not in memo.get("cfg", []):
        continue
    inputs = {os.path.normpath(row["path"]): row for row in memo["inputs"]}
    owners = {entries[key] for key in inputs.keys() & entries.keys()}
    if not owners:
        continue
    for raw in inputs:
        try:
            relative = unicodedata.normalize("NFC", str(Path(raw).relative_to(root)))
        except ValueError:
            continue
        if relative not in requested:
            continue
        for owner in owners:
            old = found.get((relative, owner))
            if old is None or old < memo["createdMs"]:
                found[relative, owner] = memo["createdMs"]

for unit in units:
    manifest = unit["pkg_id"].split("path+file://")[-1].split("#")[0]
    if not manifest.startswith("/"):
        continue
    from urllib.parse import unquote
    try:
        path = str((Path(unquote(manifest)) / "Cargo.toml").relative_to(root))
    except ValueError:
        continue
    if path in requested and name(unit) in stdio | pdf:
        found[path, name(unit)] = None

rows = []
for path in sorted(requested):
    owners = sorted(owner for candidate, owner in found if candidate == path)
    inferred_owner = None
    if not owners and "/📄️document/🖼️image-authoring/" in path:
        inferred_owner = "semio-s-artifact-stdio-pdf-editor-document"
    if not owners and "/✏️editor/📥️local-image/" in path:
        inferred_owner = "semio-s-artifact-stdio-pdf-schema-5"
    if inferred_owner:
        owners = [inferred_owner]
    row = {"path": path, "ownerInferred": inferred_owner is not None, "inStdio": [owner for owner in owners if owner in stdio],
           "inPdf": [owner for owner in owners if owner in pdf]}
    rows.append(row)
    print(f"{path}: Stdio={row['inStdio']} PDF={row['inPdf']}")
(generated / "pdf-component-overlay-impact.json").write_text(json.dumps(rows, ensure_ascii=False, indent=2) + "\n")

reverse_edges = [set() for _ in units]
for index, unit in enumerate(units):
    for edge in unit["dependencies"]:
        reverse_edges[edge["index"]].add(index)

def reach(pending, adjacent):
    pending = list(pending)
    seen = set()
    while pending:
        index = pending.pop()
        if index in seen:
            continue
        seen.add(index)
        pending.extend(adjacent(index))
    return seen

for component, owners in [("stdio", "inStdio"), ("stdio-pdf", "inPdf")]:
    package_name = f"semio-hub-{component}-component"
    relevant = reach((i for i, unit in enumerate(units) if name(unit) == package_name),
                     lambda index: (edge["index"] for edge in units[index]["dependencies"]))
    for product, prefix in [("PDF implementation", "✏️s/"), ("Framework contracts", "🧰️framework/")]:
        seeds = {owner for row in rows if row["path"].startswith(prefix) for owner in row[owners]}
        changed = reach((i for i, unit in enumerate(units) if name(unit) in seeds), reverse_edges.__getitem__)
        print(f"{component} {product}: {len(relevant & changed)}/{len(relevant)} captured units reachable")
