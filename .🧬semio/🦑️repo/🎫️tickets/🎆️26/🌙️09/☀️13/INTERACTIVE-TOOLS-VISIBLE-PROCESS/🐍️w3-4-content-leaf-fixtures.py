"""📦️ W3-4: authors the committed specification vectors of the `append-content` and `remove-content`
remodeling mutation kinds on the `⭐replace-sparse/✨️swaps-in-an-6d9ae4` toy base, with the content id the
remodeling content digest assigns to the leaf bytes."""
import base64
import json
import os
import struct

ANY = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
FIXTURES = os.path.join(ANY, "🧫️fixtures", "🧬️mutations")
MASK = (1 << 64) - 1


def digest_id(prefix, data):
    digest = [0x6C62272E07BB0142, 0x62B821756295C58D, 0x9E3779B185EBCA87, 0xC2B2AE3D27D4EB4F]
    length = 0
    for byte in data:
        length = (length + 1) & MASK
        digest[0] = ((digest[0] ^ byte) * 0x00000100000001B3) & MASK
        digest[1] = ((digest[1] ^ rotl(digest[0], 17) ^ length) * 0x9E3779B185EBCA87) & MASK
        digest[2] = ((digest[2] ^ rotl(digest[1], 29) ^ byte) * 0xC2B2AE3D27D4EB4F) & MASK
        digest[3] = ((digest[3] ^ rotl(digest[2], 41) ^ rotl(length, 7)) * 0x165667B19E3779F9) & MASK
    return f"{prefix}-{digest[0]:016x}{digest[1]:016x}{digest[2]:016x}{digest[3]:016x}-{length:016x}"


def rotl(value, bits):
    return ((value << bits) | (value >> (64 - bits))) & MASK


def load(path):
    return json.load(open(path, encoding="utf-8"))


def save(path, document):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")


def empty_diff():
    return {key: None for key in ["artifact", "schema", "id", "streams", "assets", "durableArtifacts", "calibration", "params", "gcps", "results"]}


def with_content(snapshot, content_id, artifact):
    after = json.loads(json.dumps(snapshot))
    store = dict(after["durableArtifacts"])
    store[content_id] = artifact
    after["durableArtifacts"] = {key: store[key] for key in sorted(store)}
    return after


def vector(kind_dir, case_dir, before, mutation, outcome, after=None, diff=None):
    root = os.path.join(FIXTURES, kind_dir, case_dir)
    save(os.path.join(root, "📸️snapshot", "⬅️before", "🔣️.json"), before)
    save(os.path.join(root, "📸️snapshot", "➡️after", "🔣️.json"), after if after is not None else before)
    save(os.path.join(root, "🦠️mutation", "🔣️.json"), mutation)
    save(os.path.join(root, "🎯️outcome", "🔣️.json"), outcome)
    absent = os.path.join(root, "🔺️diff", "🚫️.absent")
    diff_path = os.path.join(root, "🔺️diff", "🔣️.json")
    if diff is None:
        os.makedirs(os.path.dirname(absent), exist_ok=True)
        open(absent, "w").close()
    else:
        save(diff_path, diff)


base = load(os.path.join(FIXTURES, "⭐replace-sparse", "✨️swaps-in-an-6d9ae4", "📸️snapshot", "⬅️before", "🔣️.json"))
leaf = struct.pack("<9f", 0.5, 1.0, -0.25, 1.5, 0.0, 2.0, -1.0, 0.25, 0.75)
content_id = digest_id("remodeling-asset", leaf)
chunk = base64.b64encode(leaf).decode("ascii")
artifact = {"kind": "sparse", "mime": None, "width": 0, "height": 0, "chunks": [chunk]}
append = {"mutation": "appendContent", "contentId": content_id, "kind": "sparse", "mime": None, "width": 0, "height": 0, "first": 0, "chunks": [chunk]}
published = with_content(base, content_id, artifact)
applied_diff = empty_diff()
applied_diff["durableArtifacts"] = published["durableArtifacts"]
vector("📦append-content", "🧱️appends-sparse-leaves", base, append, {"status": "applied"}, published, applied_diff)
gap = dict(append, first=2)
vector("📦append-content", "🚫️refuses-a-gap", base, gap, {"status": "rejected", "code": "mutation.content-gap", "path": [content_id]})
vector("📦append-content", "🔁️warns-that-the-leaves-exist", published, append, {"status": "applied", "messages": [{"level": "warning", "code": "mutation.no-op"}]}, published, empty_diff())
remove = {"mutation": "removeContent", "contentId": content_id, "from": 0}
removed_diff = empty_diff()
removed_diff["durableArtifacts"] = base["durableArtifacts"]
vector("🔪remove-content", "🗑️drops-the-content", published, remove, {"status": "applied"}, base, removed_diff)
vector("🔪remove-content", "🚫️refuses-missing-content", base, remove, {"status": "rejected", "code": "mutation.target-missing", "path": [content_id]})
print(content_id)
