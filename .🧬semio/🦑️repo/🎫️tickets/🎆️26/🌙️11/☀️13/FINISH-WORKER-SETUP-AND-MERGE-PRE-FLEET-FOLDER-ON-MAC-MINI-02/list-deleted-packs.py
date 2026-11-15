"""📦 Lists the pack files that the repack of g1-server deleted at 02:03:59, the last put of each and whether its blob is still in the store."""
import glob, json, os, sys
store = "/Volumes/semio-store/v1"
last, deleted = {}, {}
for seg in glob.glob(f"{store}/origins/*/segments/*.jsonl"):
    with open(seg, encoding="utf-8", errors="replace") as f:
        for line in f:
            if ".git/objects/" not in line:
                continue
            try:
                e = json.loads(line)
            except ValueError:
                continue
            p = e["path"]
            if e["operation"] == "put" and (p not in last or e["hlc"] > last[p]["hlc"]):
                last[p] = e
            if e["operation"] == "delete" and e["hlc"].startswith("17946182") and (p not in deleted or e["hlc"] > deleted[p]):
                deleted[p] = e["hlc"]
loose = 0
for p in sorted(deleted):
    e = last.get(p)
    if "/pack/" not in p:
        loose += 1
        continue
    have = e and os.path.exists(f"{store}/blobs/{e['sha256'][:2]}/{e['sha256']}")
    print(p.split("/")[-1], e and e["bytes"], e and e["sha256"], "blob" if have else "MISSING")
print("loose deletes", loose, file=sys.stderr)
