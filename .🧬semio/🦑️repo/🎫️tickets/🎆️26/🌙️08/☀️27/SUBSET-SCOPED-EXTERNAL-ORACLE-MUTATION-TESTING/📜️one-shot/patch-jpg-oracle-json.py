import json, sys

path = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧪️oracle/🔣️.json"
with open(path, "r", encoding="utf-8") as f:
    data = json.load(f)

WITNESSABLE = {"insert-other-segment", "remove-other-segment", "replace-pixels", "change-re-encode-quality"}
UNCARRIED = {"change-jfif-header", "replace-quant-table", "remove-quant-table", "replace-huffman-table", "remove-huffman-table", "change-restart-interval"}
READER_ORACLE = "image-jpeg-jfif-1-01-mutate-reader"

manifest = data["mutationManifests"][0]
assert manifest["artifact"] == "s.stdio.jpg"

touched = []
for mutation in manifest["mutations"]:
    mid = mutation["id"]
    reqs = mutation["oracleRequirements"]
    assert len(reqs) == 1, f"{mid} has unexpected oracleRequirements shape"
    req = reqs[0]
    assert req == {"capability": "jpg-jfif-1-01-mutate", "qualifyingKind": "third-party-library"}, f"{mid} req shape drifted: {req}"
    if mid in WITNESSABLE:
        req["oracle"] = READER_ORACLE
        touched.append((mid, "witnessable"))
    elif mid in UNCARRIED:
        req["capability"] = "jpg-jfif-1-01-mutate-uncarried"
        touched.append((mid, "uncarried"))
    else:
        raise AssertionError(f"unclassified mutation id {mid}")

assert len(touched) == 10, touched

with open(path, "w", encoding="utf-8") as f:
    json.dump(data, f, ensure_ascii=False, indent=2)
    f.write("\n")

for mid, kind in touched:
    print(f"{mid}: {kind}")
