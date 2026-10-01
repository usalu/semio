"""🔲️ W2-W-media follow-up: re-derives the bmp `replace-pixel-data` vectors from the decided BITMAPINFOHEADER storage rule.
`🎯️direct-behavior` (a colour the 2-entry table lacks) now promotes the document to 24-bit `BI_RGB` without a colour table;
`🎨️keeps-indexed-storage` (only table colours) stays indexed. Each vector is the quintet the leaf's seven-law test reads."""
import copy
import json
import os

LEAF = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data"


def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n")


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def write_vector(name, before, pixels, after, diff):
    base = f"{LEAF}/{name}"
    dump(f"{base}/📸️snapshot/⬅️before/🔣️.json", before)
    dump(f"{base}/📸️snapshot/➡️after/🔣️.json", after)
    dump(f"{base}/🦠️mutation/🔣️.json", {"mutation": "replace-pixel-data", "payload": {"pixels": pixels}})
    dump(f"{base}/🔺️diff/🔣️.json", diff)
    dump(f"{base}/🎯️outcome/🔣️.json", {"status": "applied"})


def stride(width, bits):
    return (width * bits + 31) // 32 * 4


before = load(f"{LEAF}/🎯️direct-behavior/📸️snapshot/⬅️before/🔣️.json")
assert before["bitsPerPixel"] == 8 and len(before["palette"]) == 2

grey = [9, 9, 9, 255] * 4
promoted = copy.deepcopy(before)
promoted.update(bitsPerPixel=24, palette=[], colorsUsed=0, colorsImportant=0, imageSize=stride(before["width"], 24) * before["height"], pixels=grey)
promoted_diff = {"bitsPerPixel": 24, "colorsImportant": 0, "colorsUsed": 0, "palette": {"removed": list(range(len(before["palette"])))}, "pixels": grey}
if promoted["imageSize"] != before["imageSize"]:
    promoted_diff["imageSize"] = promoted["imageSize"]
write_vector("🎯️direct-behavior", before, grey, promoted, promoted_diff)

swapped = [0, 255, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 0, 255, 0, 255]
table = {(entry["r"], entry["g"], entry["b"]) for entry in before["palette"]}
assert all((swapped[i], swapped[i + 1], swapped[i + 2]) in table for i in range(0, len(swapped), 4)) and swapped != before["pixels"]
kept = copy.deepcopy(before)
kept["pixels"] = swapped
write_vector("🎨️keeps-indexed-storage", before, swapped, kept, {"pixels": swapped})
print("[w2w-media] wrote 🎯️direct-behavior and 🎨️keeps-indexed-storage")
