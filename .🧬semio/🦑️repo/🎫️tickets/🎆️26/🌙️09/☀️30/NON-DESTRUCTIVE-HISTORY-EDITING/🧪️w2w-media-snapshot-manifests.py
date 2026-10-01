"""🧾️ W2-W-media follow-up: declares the snapshot-editing kinds production dispatch offers in the png, jpg document, tiff
document and bmp owner contributions — catalog kind, manifest row (outcomes and variant read from the leaf descriptor, the
oracle requirement of the kind whose facet the edit reaches) and a handcrafted before/after snapshot fixture pair per kind,
built from the subset's own committed `set-snapshot` wire witness."""
import copy
import hashlib
import json
import os

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
ATTRIBUTION = "Handcrafted before/after vector authored directly against this subset's own documented schema — Law 2's handcrafted-vector category."


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def pin(path):
    data = open(path, "rb").read()
    return {"sha256": "sha256:" + hashlib.sha256(data).hexdigest(), "bytes": len(data)}


def declare(subset, artifact, standard, subset_id, family, kinds):
    owner = f"{ROOT}/{subset}"
    manifest_path = f"{owner}/🔮️oracles/🔣️.json"
    raw = open(manifest_path, encoding="utf-8").read()
    document = json.loads(raw)
    assert json.dumps(document, indent=2, ensure_ascii=False) + "\n" == raw
    catalog = document["mutationCatalogs"][0]
    rows = document["mutationManifests"][0]["mutations"]
    snapshot = load(f"{owner}/🧫️fixtures/🧬️mutations/📸️set-snapshot/🧾️wire-witness/🦠️mutation/🔣️.json")["payload"]["snapshot"]
    for kind, leaf_dir, requirement_of, change, notes in kinds:
        assert kind not in catalog["kinds"] and all(row["id"] != kind for row in rows), kind
        catalog["kinds"].append(kind)
        descriptor = load(f"{owner}/🧬️schema/🧬️mutations/{leaf_dir}/🔣️.json")
        template = next(row for row in rows if row["id"] == requirement_of)
        row = copy.deepcopy(template)
        row.update(id=kind, outcomes=descriptor["outcomeClasses"])
        row["productionDispatch"] = {**template["productionDispatch"], "operation": kind, "variant": descriptor["aggregateVariant"]}
        rows.append(row)
        directory = f"{leaf_dir}-applied"
        before, after = copy.deepcopy(snapshot), change(copy.deepcopy(snapshot))
        assert before != after, kind
        for name, value in (("⬅️before.json", before), ("➡️after.json", after)):
            dump(f"{owner}/🧫️fixtures/{directory}/{name}", value)
        files = []
        for role, name in (("expected-before", "⬅️before.json"), ("expected-after", "➡️after.json")):
            relative = f"../🧫️fixtures/{directory}/{name}"
            files.append({"role": role, "path": relative, "mediaType": "application/json", **pin(f"{owner}/🔮️oracles/{relative}")})
        document["fixtureManifests"].append({
            "schema": "semio.repository-test.fixture/v2", "id": f"{kind}-applied", "class": "handcrafted",
            "target": {"artifact": artifact, "standard": standard, "subset": subset_id}, "mutation": kind, "outcome": "applied",
            "units": {"length": "unitless", "angle": "degree"}, "files": files,
            "provenance": {"source": "authored", "license": "public-domain (handcrafted by this repository)", "attribution": ATTRIBUTION, "security": "scanned-clean", "privacy": "no-personal-data"},
            "comparisonProfile": "ordered-json-v1", "reproducible": True, "family": family, "notes": notes,
        })
    rows.sort(key=lambda row: row["id"])
    dump(manifest_path, document)
    print(f"[w2w-media] {artifact}@{standard}/{subset_id}: declared {', '.join(kind for kind, *_ in kinds)}")


def with_pixels(pixels):
    def change(snapshot):
        snapshot["pixels"] = pixels
        return snapshot
    return change


def with_member(member, value):
    def change(snapshot):
        snapshot[member] = value
        return snapshot
    return change


def splice(index, replacement):
    def change(snapshot):
        snapshot["pixels"][index:index + len(replacement)] = replacement
        return snapshot
    return change


declare("📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any", "s.stdio.png", "1.2", "any", "png-carrier", [
    ("set-snapshot", "📸️set-snapshot", "replace-pixels", with_pixels([0, 0, 255, 255, 255, 0, 0, 255]), "SetSnapshot replaces the whole document: the 2x1 RGBA swatch's two pixels trade places while its gAMA and tEXt chunks stay as the replacement states them."),
    ("patch-snapshot", "🩹️patch-snapshot", "change-gamma", with_member("gama", 45455), "PatchSnapshot{edits:[set /gama = 45455]} is the editor's compact path-addressed patch: the gAMA chunk changes in place and every other member stays untouched."),
    ("patch-pixels", "🩹️patch-pixels", "replace-pixels", splice(4, [0, 255, 0, 255]), "PatchPixels{index:4,removeCount:4,pixels:[0,255,0,255]} replaces the second RGBA pixel in place; the raster keeps its width*height*4 byte length, which PNG's image data requires."),
])
declare("📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document", "s.stdio.jpg", "jfif-1.01", "document", "jpg-carrier", [
    ("set-snapshot", "📸️set-snapshot", "replace-pixels", with_pixels([200, 40, 40, 255]), "SetSnapshot replaces the whole document: the 1x1 swatch's one pixel is repainted while its JFIF header, COM segment and re-encode quality stay as the replacement states them."),
    ("patch-snapshot", "🩹️patch-snapshot", "change-jfif-header", with_member("jfifXDensity", 300), "PatchSnapshot{edits:[set /jfifXDensity = 300]} is the editor's compact path-addressed patch: the JFIF APP0 horizontal density changes in place and every other member stays untouched."),
])
declare("🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document", "s.stdio.tiff", "6.0", "document", "tiff-carrier", [
    ("set-snapshot", "📸️set-snapshot", "replace-pixels", with_pixels([255, 255, 255, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 255]), "SetSnapshot replaces the whole document: the 2x2 RGBA raster of IFD 0 is reversed while its tags stay as the replacement states them."),
    ("patch-snapshot", "🩹️patch-snapshot", "change-byte-order", with_member("byteOrder", "bigEndian"), "PatchSnapshot{edits:[set /byteOrder = bigEndian]} is the editor's compact path-addressed patch: the file header's byte order flips and every IFD keeps its typed values."),
])
declare("🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any", "s.stdio.bmp", "v3", "any", "bmp-carrier", [
    ("set-snapshot", "📸️set-snapshot", "replace-pixel-data", with_pixels([0, 255, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 0, 255, 0, 255]), "SetSnapshot replaces the whole document: the 2x2 indexed swatch's red and green pixels trade places, every one still an entry of its two-colour table."),
])
