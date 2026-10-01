"""🧩️ W2-W-media follow-up: adds the snapshot-editing kinds production dispatch offers — `set-snapshot`, `patch-snapshot` and
png `patch-pixels` — as wire rows to the real-document mutate and inverse outlines of the png, jpg document, tiff document
and bmp cases. Each row is the leaf's exact `payload_value()` wire. Refuses to run twice."""
import json

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"


def compact(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def witness(path):
    with open(f"{ROOT}/{path}", encoding="utf-8") as handle:
        return json.load(handle)["payload"]


def entry(tag, kind, value):
    return {"kind": kind, "tag": tag, "values": {"kind": kind, "value": value}}


PNG = "📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any"
JPG = "📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document"
TIFF = "🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document"
BMP = "🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any"

CASES = {
    f"{PNG}/🧪️tests/🔀️mutate-png-1-2/🥒️.feature": ("shared://🏛️rathaus-ahlen-grundriss/🖼️.png", [
        ("set-snapshot", witness(f"{PNG}/🧫️fixtures/🧬️mutations/📸️set-snapshot/🧾️wire-witness/🦠️mutation/🔣️.json")),
        ("patch-snapshot", {"patch": {"edits": [{"path": ["gama"], "edit": {"operation": "insert", "value": 45455}}]}}),
        ("patch-pixels", {"index": 0, "removeCount": 4, "pixels": [200, 40, 40, 255]}),
    ]),
    f"{JPG}/🧪️tests/📸️mutate-jpg-jfif-1-01/🥒️.feature": ("shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg", [
        ("set-snapshot", witness(f"{JPG}/🧫️fixtures/🧬️mutations/📸️set-snapshot/🧾️wire-witness/🦠️mutation/🔣️.json")),
        ("patch-snapshot", {"patch": {"edits": [{"path": ["jfifXDensity"], "edit": {"operation": "set", "value": 300}}]}}),
    ]),
    f"{TIFF}/🧪️tests/🖼️mutate-tiff-6-0/🥒️.feature": ("shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff", [
        ("set-snapshot", {"snapshot": {"byteOrder": "littleEndian", "ifds": [{"entries": [entry(256, "long", [2]), entry(257, "long", [2]), entry(305, "ascii", "semio")], "pixels": []}], "pixels": [255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255], "schema": "stdio.tiff"}}),
        ("patch-snapshot", {"patch": {"edits": [{"path": ["byteOrder"], "edit": {"operation": "set", "value": "bigEndian"}}]}}),
    ]),
    f"{BMP}/🧪️tests/🪟️mutate-bmp-v3/🥒️.feature": ("shared://🏛️rathaus-ahlen-grundriss/🖼️.bmp", [
        ("set-snapshot", witness(f"{BMP}/🧫️fixtures/🧬️mutations/📸️set-snapshot/🧾️wire-witness/🦠️mutation/🔣️.json")),
    ]),
}


def add_rows(path, uri, rows):
    lines = open(f"{ROOT}/{path}", encoding="utf-8").read().split("\n")
    givens = [index for index, line in enumerate(lines) if line.strip() == f"Given the real input document {uri}"]
    outlines = [index for index in givens if any(lines[back].strip() in ("@id-mutate", "@id-inverse") for back in range(max(0, index - 4), index))]
    assert len(outlines) == 2, (path, outlines)
    for start in reversed(outlines):
        at = next(index for index in range(start, len(lines)) if lines[index].strip() == "Examples:") + 2
        while lines[at].startswith("      |"):
            assert not any(lines[at].startswith(f"      | {kind} |") for kind, _ in rows), (path, lines[at][:60])
            at += 1
        lines[at:at] = [f"      | {kind} | {compact(params)} |" for kind, params in rows]
    open(f"{ROOT}/{path}", "w", encoding="utf-8").write("\n".join(lines))


for feature, (uri, rows) in CASES.items():
    add_rows(feature, uri, rows)
    print(f"[w2w-media] {feature.split('/')[0]}: {len(rows)} kind(s) added to the mutate and inverse outlines")
