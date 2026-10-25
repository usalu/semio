#!/usr/bin/env python3
"""Delete classified staged source copies (only files inside staged roots of the ticket) and prune empty dirs.

Usage: python3 delete.py <ticket-dir> <inv.json> <inventory.tsv> [--go]
Dry run unless --go. Re-hashes each file before removal and skips files changed since the scan.
"""
import hashlib, json, os, sys

C, invp, tsv = sys.argv[1:4]
go = "--go" in sys.argv
inv = json.load(open(invp))
DELETE = ("identical-live", "different", "head-live-changed", "head-live-gone", "orphan")
Cr = os.path.realpath(C)


def gsha(data):
    return hashlib.sha1(b"blob %d\0" % len(data) + data).hexdigest()


with open(tsv, "w", encoding="utf-8") as f:
    f.write("root\tclass\tmap\tpatched\tstaged_rel\tcounterpart\n")
    for r in inv["rows"]:
        f.write("\t".join([r["root"], r["cls"], str(r.get("map") or ""), "y" if r.get("patched") else "", r["rel"], r.get("live") or r.get("shaped") or ""]) + "\n")

n = skipped = 0
for r in inv["rows"]:
    if r["cls"] not in DELETE:
        continue
    p = os.path.join(C, r["root"], r["rel"])
    rp = os.path.realpath(p)
    if not rp.startswith(Cr + os.sep) or not os.path.basename(r["root"]).startswith("staged") or os.path.islink(p):
        print("REFUSE", p)
        skipped += 1
        continue
    if r["cls"] in ("different", "head-live-changed", "head-live-gone", "orphan") and not r.get("patched"):
        print("NOPATCH", p)
        skipped += 1
        continue
    try:
        ok = gsha(open(p, "rb").read()) == r["sha1"]
    except OSError:
        continue
    if not ok:
        print("CHANGED", p)
        skipped += 1
        continue
    n += 1
    if go:
        os.remove(p)
removed_dirs = 0
if go:
    for r in sorted(set(x["root"] for x in inv["rows"])):
        base = os.path.join(C, r)
        for dp, dns, fns in os.walk(base, topdown=False, followlinks=False):
            if not os.path.islink(dp) and not os.listdir(dp):
                os.rmdir(dp)
                removed_dirs += 1
print("files", "deleted" if go else "to-delete", n, "skipped", skipped, "dirs-removed", removed_dirs)
