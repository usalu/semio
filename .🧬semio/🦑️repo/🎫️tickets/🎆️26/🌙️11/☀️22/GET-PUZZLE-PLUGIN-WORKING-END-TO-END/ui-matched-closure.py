import json
from pathlib import Path

ticket = Path(__file__).resolve().parent
base = ticket / "🗑️generated/current-native/b/debug/build"
libraries = {
    fingerprint.read_text().strip(): fingerprint.with_suffix(".json")
    for fingerprint in base.glob("*/*/fingerprint/lib-*")
    if fingerprint.suffix != ".json"
}


def fingerprint_key(row):
    return row[3].to_bytes(8, "little").hex()


def closure(path, nodes):
    data = json.loads(path.read_text())
    for row in data["deps"]:
        key = fingerprint_key(row)
        if key in nodes or key not in libraries:
            continue
        child = libraries[key]
        nodes[key] = (child, closure(child, nodes))
    return data


def owner(pair):
    path, data = pair
    return path.parents[2].name, data["features"]


old = base / "semio-s-artifact-puzzle-2d-v1/31f97683412a2e44/fingerprint/lib-semio_s_artifact_puzzle_2d_v1.json"
new = max(
    (base / "semio-s-artifact-puzzle-3d-editor").glob("*/fingerprint/lib-semio_s_artifact_puzzle_3d_editor.json"),
    key=lambda path: path.stat().st_mtime,
)
left, right = {}, {}
closure(old, left)
closure(new, right)
replacement = {owner(pair): key for key, pair in right.items()}
impact = {}


def change(key):
    if key not in left:
        return False
    if key not in impact:
        _, data = left[key]
        match = replacement.get(owner(left[key]))
        if match:
            impact[key] = "Fresh" if match != key else "Same"
        else:
            impact[key] = "Relink" if any(change(fingerprint_key(row)) for row in data["deps"]) else "Same"
    return impact[key] != "Same"


for key in left:
    change(key)
print(len(left), len(right), {kind: list(impact.values()).count(kind) for kind in set(impact.values())})
for name in sorted({owner(left[key])[0] for key, state in impact.items() if state == "Relink"}):
    print(name)
