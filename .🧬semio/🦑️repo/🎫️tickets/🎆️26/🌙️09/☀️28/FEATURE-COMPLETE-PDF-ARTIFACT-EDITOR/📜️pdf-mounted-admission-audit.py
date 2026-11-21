"""🔏️ Checks the mounted component graph and selected inputs against repository admission memos."""

import hashlib
import json
import os
from pathlib import Path
import sys
from urllib.parse import unquote

root = Path.cwd()
ticket = Path(__file__).resolve().parent
run = Path(sys.argv[1])
graph = json.loads((run / "l/unit-graph-prototype-00.json").read_text())["units"]
base = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"
brep = root / "🧰️framework/🔨️modules/🧊️3d/📐️brep"
wanted = {
    "BREP representation contract": brep / "📸️representation/🔖️contract/🦀️.rs",
    "BREP solid transaction": brep / "⚙️engine/🧠️native-registry/🏗️solid-transaction/🦀️.rs",
    "BREP mesh preview": brep / "⚙️engine/📦️mesh-io/🖼️primitive-preview/🦀️.rs",
    "BREP spline nearest": brep / "📸️representation/➰️curve/🧵️bounded-spline/🎯️nearest/🦀️.rs",
    "PDF controlled document": base / "🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs",
    "PDF controlled image": base / "🔨️modules/🖼️images/🎞️controlled/🦀️.rs",
    "PDF JPEG raw stream": base / "🔨️modules/🔗️xref/🦀️.rs",
    "PDF JPEG typed stream": base / "🚪️io/📥️import/🦀️.rs",
    "PDF font and encoding": base / "🔨️modules/🔤️fonts/🦀️.rs",
    "PDF CFF": base / "🔨️modules/🔤️fonts/✒️cff/🦀️.rs",
    "PDF image authoring": base / "✏️editor/📄️document/🖼️image-authoring/🦀️.rs",
    "PDF PNG decode": base / "✏️editor/📄️document/🖼️image-authoring/🧵️decode.rs",
    "PDF PNG controller": base / "✏️editor/📥️local-image/🦀️.rs",
    "PDF page inspector": base / "✏️editor/🖼️page/🔎️inspector/🦀️.rs",
}
label_paths = [line[6:] for line in (ticket / "📜️pdf-history-labels.patch").read_text().splitlines() if line.startswith("+++ b/")]
for i, path in enumerate(label_paths, 1):
    wanted[f"PDF history schema {i}"] = root / path

by_path = {os.path.normpath(path): name for name, path in wanted.items()}
digest = {name: hashlib.sha256(path.read_bytes()).hexdigest() for name, path in wanted.items()}
rows = {name: {"path": str(path.relative_to(root)), "live": digest[name]} for name, path in wanted.items()}
for path in Path(".cache/b/r/m/u/a").glob("*/*.json"):
    try:
        raw = path.read_text()
    except FileNotFoundError:
        continue
    if not any(name in raw for name in by_path):
        continue
    item = json.loads(raw)
    if "target_env=p2" not in item["cfg"] or "test" in item["cfg"]:
        continue
    for input in item["inputs"]:
        name = by_path.get(os.path.normpath(input["path"]))
        if name and (item["createdMs"] > rows[name].get("capturedAtMilliseconds", 0)):
            rows[name].update(capturedAtMilliseconds=item["createdMs"], admitted=input["digest"], bytes=input["bytes"])

def pkg(i):
    return graph[i]["pkg_id"].split("#")[-1].split("@")[0]

edges = {}
for owner in ["semio-s-artifact-stdio-pdf-editor-document", "semio-s-artifact-stdio-pdf-schema-5"]:
    found = next(i for i, item in enumerate(graph) if pkg(i) == owner and item["platform"] == "wasm32-wasip2")
    edges[owner] = sorted(pkg(edge["index"]) for edge in graph[found]["dependencies"])
assert {"semio-framework-pixels", "semio-s-artifact-stdio-contract"} <= set(edges["semio-s-artifact-stdio-pdf-editor-document"])
assert "semio-s-artifact-stdio-pdf-editor-document" in edges["semio-s-artifact-stdio-pdf-schema-5"]
for name, row in rows.items():
    state = "exact" if row["live"] == row.get("admitted") else "different" if "admitted" in row else "no direct WASI memo"
    print("[DEBUG]", name, state, row.get("bytes", ""), row["live"][:14], row.get("capturedAtMilliseconds", ""))
for unit, dependencies in edges.items():
    print("[DEBUG]", unit, "WASI edges", ", ".join(dependencies))
result = {"generation": run.name, "edges": edges, "inputs": rows}
(ticket / "🗑️generated/pdf-mounted-admission-audit.json").write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
