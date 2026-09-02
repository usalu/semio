import json, os

SEMIO = "/Users/ueli/Documents/semio"
os.chdir(SEMIO)

with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/component_dirs.txt") as f:
    dirs = [l.strip()[2:] for l in f if l.strip()]

no_schema_file = []
payload_mismatch = []
for d in dirs:
    entries = os.listdir(d)
    schema_files = [e for e in entries if e.startswith("🔣️") and e.endswith(".schema.json")]
    jpath = os.path.join(d, "🔣️.json")
    cpath = os.path.join(d, "🔣️component.json")
    j = json.load(open(jpath, encoding="utf-8"))
    c = json.load(open(cpath, encoding="utf-8"))
    if not schema_files:
        no_schema_file.append({"dir": d, "j_payloadSchema": j.get("payloadSchema"), "c_payloadSchema": c.get("payloadSchema")})
    else:
        real = schema_files[0]
        if c.get("payloadSchema") != real:
            payload_mismatch.append({"dir": d, "real": real, "j": j.get("payloadSchema"), "c": c.get("payloadSchema"), "j_matches_real": j.get("payloadSchema")==real})

print(f"Dirs with NO schema.json file present: {len(no_schema_file)}")
for x in no_schema_file[:10]:
    print(" ", x)

print(f"\nDirs where component.json's payloadSchema != real filename: {len(payload_mismatch)}")
j_matches_count = sum(1 for x in payload_mismatch if x["j_matches_real"])
print(f"  of those, .json's payloadSchema DOES match real filename: {j_matches_count}")
for x in payload_mismatch[:15]:
    print(" ", x)
