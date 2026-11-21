"""🔏️ Checks the reviewed PDF and Hub release against governed WASI inputs and Cargo edges."""

import hashlib
import json
from pathlib import Path
import sys

root = Path.cwd()
generated = Path(__file__).resolve().parent / "🗑️generated"
run = Path(sys.argv[1])
units = json.loads((run / "l/unit-graph-prototype-00.json").read_text())["units"]
product = json.loads((generated / "pdf-reviewed-postimages.json").read_text())
hub = json.loads((generated / "hub-assembly/reviewed-hub-assembly.json").read_text())
expected = product | {path: info["postimage"] for path, info in hub.items()}
live = {path: hashlib.sha256((root / path).read_bytes()).hexdigest() for path in expected}
lock_path = "🌎️hub/Cargo.lock"
drifted = {path for path in expected if live[path] != expected[path]}
assert len(product) == 21 and len(hub) == 4 and drifted <= {lock_path}, drifted
custody = {}
for path, digest in expected.items():
    copy = run / "cs" / digest
    if copy.is_file():
        assert hashlib.sha256(copy.read_bytes()).hexdigest() == digest
        custody[path] = digest
production = {path for path in expected if path.endswith(".rs") and "/🧪️tests/" not in path}
assert set(custody) == production, production.symmetric_difference(custody)

rust = {path: {} for path in expected if path.endswith(".rs")}
absolute = {str(root / path): path for path in rust}
for memo in (root / ".cache/b/r/m/u/a").glob("*/*.json"):
    try:
        raw = memo.read_text()
    except FileNotFoundError:
        continue
    if not any(path in raw for path in absolute):
        continue
    row = json.loads(raw)
    if "target_env=p2" not in row["cfg"] or "test" in row["cfg"]:
        continue
    for item in row["inputs"]:
        if (path := absolute.get(item["path"])) and row["createdMs"] > rust[path].get("createdMs", 0):
            rust[path] = {"digest": item["digest"], "createdMs": row["createdMs"], "bytes": item["bytes"]}

def package(index):
    return units[index]["pkg_id"].split("#")[-1].split("@")[0]

wanted = ("semio-hub-stdio", "semio-hub-stdio-pdf", "semio-hub-stdio-component", "semio-hub-stdio-pdf-component")
edges = {package(i): sorted(package(edge["index"]) for edge in row["dependencies"])
         for i, row in enumerate(units) if row["platform"] == "wasm32-wasip2" and package(i) in wanted}
for owner in wanted[:2]:
    assert "semio-framework-value" in edges[owner] and "semio-framework-job" in edges[owner], (owner, edges[owner])
for owner, component in zip(wanted, wanted[2:]):
    assert owner in edges[component], (owner, component)
lock = (root / "🌎️hub/Cargo.lock").read_text()
package_lock = next(block for block in lock.split("[[package]]") if '\nname = "semio-hub-stdio-pdf"\n' in block)
assert '\n "semio-framework-value",\n' in package_lock
states = {path: item.get("digest") == expected[path] for path, item in rust.items()}
for path in states:
    print("[DEBUG]", "exact" if states[path] else "unavailable or changed", rust[path].get("createdMs", ""), path)
print("[DEBUG]", f"{sum(states.values())}/{len(states)} reviewed Rust sources match latest WASI admission memos")
print("[DEBUG]", f"{len(custody)}/{len(production)} reviewed production Rust sources are present in the governed generation's source store.")
print("[DEBUG]", f"{len(expected) - len(drifted)}/{len(expected)} live release postimages match. Hub manifest and lock edge appear in governed graph.")
if drifted:
    print("[DEBUG]", "Whole Hub lock changed after graph capture. The exact reviewed lock dependency remains live.")
print("[DEBUG]", "Nested schema inputs require exact compiler observer receipts after emission.")
proof = {"generation": run.name, "lockedEdges": edges, "live": live, "latestWasiRustMemos": rust, "governedSourceStore": custody, "liveChanged": sorted(drifted)}
(generated / ("pdf-reviewed-wasi-" + run.name + ".json")).write_text(json.dumps(proof, indent=2, ensure_ascii=False) + "\n")
