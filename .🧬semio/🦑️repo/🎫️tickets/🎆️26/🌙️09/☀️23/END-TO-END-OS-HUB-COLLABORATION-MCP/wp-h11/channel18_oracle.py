"""🔢️ H11 (ticket 26/09/23, session 13): independent Python oracle of the document-open catalog encoding
(`documentOpenCatalogEncoding` in 🌎️hub/📦️packages/🦀️rust/📜️script.ts) and of the frozen GIS Map binding digest, used to move
the hand-written fixtures from appChannelVersion 17 to 18 after G11's CHANNEL_VERSION bump. Verifies every fixture's
current expected values first, then rewrites version + dependent digests. usage: python3 channel18_oracle.py [--write]"""
import hashlib, json, struct, sys
from collections import OrderedDict

WRITE = "--write" in sys.argv
OLD, NEW = 17, 18

def lp(b): return struct.pack(">Q", len(b)) + b

def catalog_encoding(rows):
    out = [b"semio/hub/openable-document-catalog/v1\0", struct.pack(">I", len(rows))]
    for r in rows:
        p, a, pd, s, g = r["package"], r["artifact"], r["parentDialect"], r["surface"], r["grant"]
        fields = [p["pluginId"].encode(), p["packageId"].encode(), p["version"].encode(), bytes.fromhex(p["componentSha256"]), bytes.fromhex(p["componentBlake3"]), bytes.fromhex(p["descriptorByteSha256"]), struct.pack(">I", p["executionProtocol"]["appChannelVersion"]), a["kind"].encode(), a["schema"].encode(), bytes.fromhex(a["packSchemaHash"]), pd["artifactKind"].encode(), pd["standard"].encode(), pd["subset"].encode(), s["surfaceId"].encode(), s["appId"].encode(), s["windowKindId"].encode(), s["role"].encode(), s["rendererTarget"].encode(), bytes([int(g["read"]), int(g["write"]), int(g["observe"])])]
        out.append(b"".join(lp(x) for x in fields))
    return b"".join(out)

def bump(value):
    if isinstance(value, dict):
        for key, child in value.items():
            if key == "appChannelVersion" and child == OLD:
                value[key] = NEW
            else:
                bump(child)
    elif isinstance(value, list):
        for child in value:
            bump(child)

def replace_text(value, old, new):
    if isinstance(value, dict):
        return OrderedDict((k, replace_text(v, old, new)) for k, v in value.items())
    if isinstance(value, list):
        return [replace_text(v, old, new) for v in value]
    return new if value == old else value

def plan_fixture(path):
    d = json.load(open(path), object_pairs_hook=OrderedDict)
    enc = catalog_encoding(d["catalogRows"])
    assert enc.hex() == d["catalogEncoding"]["expectedHex"], "catalog encoding oracle disagrees with the fixture before the bump"
    old_generation = d["catalogEncoding"]["expectedGenerationId"]
    assert hashlib.sha256(enc).hexdigest() == old_generation
    bump(d)
    enc = catalog_encoding(d["catalogRows"])
    new_generation = hashlib.sha256(enc).hexdigest()
    d = replace_text(d, old_generation, new_generation)
    d["catalogEncoding"]["expectedHex"] = enc.hex()
    return d, f"generation {old_generation[:12]}… → {new_generation[:12]}…"

def binding_digest(binding): return hashlib.sha256(b"semio.hub.gis-map-frozen-binding/v1\x00" + json.dumps(binding, separators=(",", ":"), ensure_ascii=False).encode()).hexdigest()

def frozen_fixture(path):
    d = json.load(open(path), object_pairs_hook=OrderedDict)
    assert binding_digest(d["binding"]) == d["expectedDigest"]
    bump(d)
    d["expectedDigest"] = binding_digest(d["binding"])
    return d, f"digest → {d['expectedDigest'][:12]}…"

def plain_fixture(path):
    d = json.load(open(path), object_pairs_hook=OrderedDict)
    bump(d)
    return d, "version only"

PLAN = "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"
FROZEN = "🌎️hub/🧫️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json"
for path, rewrite in [(PLAN, plan_fixture), (FROZEN, frozen_fixture)]:
    d, note = rewrite(path)
    print(("WRITE " if WRITE else "DRY ") + path + ": " + note)
    if WRITE:
        open(path, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
