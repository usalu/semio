"""🧾️ W2-W-media owner-manifest completion for wav and mp4: `patch-snapshot` manifest rows, handcrafted before/after
fixture pairs for the kinds that had none, and wav fixture hashes re-pinned to the committed bytes."""
import copy
import hashlib
import json
import os
import sys

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
WAV = f"{ROOT}/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any"
MP4 = f"{ROOT}/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any"
ATTRIBUTION = "Handcrafted before/after vector authored directly against this subset's own documented schema — Law 2's handcrafted-vector category."


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def dump(path, value):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def pin(owner, relative):
    data = open(os.path.join(owner, "🔮️oracles", relative), "rb").read()
    return {"sha256": "sha256:" + hashlib.sha256(data).hexdigest(), "bytes": len(data)}


def manifest_row(source, kind, variant, outcomes):
    row = copy.deepcopy(source)
    row["id"] = kind
    row["outcomes"] = outcomes
    row["productionDispatch"] = {**row["productionDispatch"], "operation": kind, "variant": variant}
    return row


def fixture_entry(template, owner, kind, directory, notes):
    entry = copy.deepcopy(template)
    entry["id"] = f"{kind}-applied"
    entry["mutation"] = kind
    entry["outcome"] = "applied"
    entry["provenance"]["attribution"] = ATTRIBUTION
    entry["notes"] = notes
    for file in entry["files"]:
        name = "⬅️before.json" if file["role"] == "expected-before" else "➡️after.json"
        file["path"] = f"../🧫️fixtures/{directory}/{name}"
        file.update(pin(owner, file["path"]))
    return entry


def write_pair(owner, directory, before, after):
    target = os.path.join(owner, "🧫️fixtures", directory)
    os.makedirs(target, exist_ok=True)
    dump(os.path.join(target, "⬅️before.json"), before)
    dump(os.path.join(target, "➡️after.json"), after)


def insert_sorted(rows, row):
    assert all(existing["id"] != row["id"] for existing in rows), row["id"]
    rows.append(row)
    rows.sort(key=lambda existing: existing["id"])


def wav():
    path = f"{WAV}/🔮️oracles/🔣️.json"
    document = load(path)
    for fixture in document["fixtureManifests"]:
        for file in fixture["files"]:
            file.update(pin(WAV, file["path"]))
    rows = document["mutationManifests"][0]["mutations"]
    snapshot = next(row for row in rows if row["id"] == "set-snapshot")
    insert_sorted(rows, manifest_row(snapshot, "patch-snapshot", "PatchSnapshot", ["applied", "rejected"]))
    before = load(f"{WAV}/🧫️fixtures/🔊️set-data/⬅️before.json")
    patched = copy.deepcopy(before)
    patched["data"]["value"][1:3] = [7, -7, 9]
    write_pair(WAV, "🩹️patch-data", before, patched)
    edited = copy.deepcopy(before)
    edited["data"]["value"][2] = 300
    write_pair(WAV, "🩹️patch-snapshot", before, edited)
    template = next(fixture for fixture in document["fixtureManifests"] if fixture["mutation"] == "set-data")
    document["fixtureManifests"].append(fixture_entry(template, WAV, "patch-data", "🩹️patch-data", "PatchData{index:1,removeCount:2,data:pcm16[7,-7,9]} splices three PCM16 samples over the two at indices 1..3 — the recording grows by one sample while fmt, otherChunks and chunkOrder stay untouched."))
    document["fixtureManifests"].append(fixture_entry(template, WAV, "patch-snapshot", "🩹️patch-snapshot", "PatchSnapshot{edits:[set /data/value/2 = 300]} is the editor's compact path-addressed patch: one sample changes in place and every other member of the snapshot stays byte-identical."))
    dump(path, document)


def mp4():
    path = f"{MP4}/🔮️oracles/🔣️.json"
    document = load(path)
    rows = document["mutationManifests"][0]["mutations"]
    snapshot = next(row for row in rows if row["id"] == "set-snapshot")
    insert_sorted(rows, manifest_row(snapshot, "patch-snapshot", "PatchSnapshot", ["applied", "rejected"]))
    before = load(f"{MP4}/🧫️fixtures/⭐set-sample-sync/⬅️before.json")
    after = copy.deepcopy(before)
    after["tracks"][0]["width"] = 32
    write_pair(MP4, "🩹️patch-snapshot", before, after)
    template = next(fixture for fixture in document["fixtureManifests"] if fixture["mutation"] == "set-sample-sync")
    document["fixtureManifests"].append(fixture_entry(template, MP4, "patch-snapshot", "🩹️patch-snapshot", "Handcrafted Mp4Snapshot before/after pair on the same committed single-track AVC/H.264 base as set-sample-sync. PatchSnapshot{edits:[set /tracks/0/width = 32]} is the editor's compact path-addressed patch: the tkhd width changes in place and every sample, the codec and the movie header stay untouched."))
    dump(path, document)


if __name__ == "__main__":
    {"wav": wav, "mp4": mp4}[sys.argv[1]]()
