#!/usr/bin/env python3
"""🗂️ Compares the graph stores two harness runs wrote (serial against N workers), class by class: closure inventories, resolution receipts, input precision receipts, record blocks, project records (the digest and the sha256 of the project value inside the header, then the whole bytes) and parser record packs (by content, their names carry a clock and a process id).
usage: compare-store.py TAG_A TAG_B [DIRECTORY]   (default /private/tmp/graph-parallel). Needs the `brotli` command for the records. Prints one JSON line; exit 0 when every class that must be byte identical is, namely closures, receipts and the value of every record."""
import collections, hashlib, json, os, pathlib, subprocess, sys

a, b = sys.argv[1], sys.argv[2]
base = pathlib.Path(sys.argv[3] if len(sys.argv) > 3 else "/private/tmp/graph-parallel")


def walk(tag):
    root, out = base / tag, {}
    for path in sorted(root.rglob("*")):
        if path.is_file():
            rel = str(path.relative_to(root))
            if not rel.startswith(("lease", "optional")):
                out[rel] = path
    return out


def klass(rel):
    if rel.startswith(("build-input-closures/", "build-lock-closures/")):
        return "closures"
    if rel.startswith("inference/rr/"):
        return "receipts-rr"
    if rel.startswith("inference/ip/"):
        return "receipts-ip"
    if "/ob/" in rel:
        return "blocks"
    if ".records/" in rel:
        return "packs"
    return "records" if rel.startswith("inference/np/") else "other"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def header(path):
    text = subprocess.run(["brotli", "-dc", str(path)], capture_output=True).stdout
    line = text.split(b"\n", 1)[0]
    return json.loads(line)


fa, fb = walk(a), walk(b)
report = collections.defaultdict(lambda: collections.Counter())
for rel in sorted(set(fa) | set(fb)):
    k = klass(rel)
    if k == "packs":
        continue
    if rel not in fa or rel not in fb:
        report[k]["onlyA" if rel in fa else "onlyB"] += 1
    elif digest(fa[rel]) == digest(fb[rel]):
        report[k]["identical"] += 1
    else:
        report[k]["differ"] += 1
        if k == "records":
            ha, hb = header(fa[rel]), header(fb[rel])
            report[k]["valueSha256Equal" if ha["resultSha256"] == hb["resultSha256"] else "valueSha256Differ"] += 1
            report[k]["digestEqual" if ha["digest"] == hb["digest"] else "digestDiffer"] += 1
packs = [collections.Counter(digest(path) for rel, path in files.items() if klass(rel) == "packs") for files in (fa, fb)]
report["packs"].update({"contentIdentical": sum((packs[0] & packs[1]).values()), "contentOnlyA": sum((packs[0] - packs[1]).values()), "contentOnlyB": sum((packs[1] - packs[0]).values())})
print(json.dumps({k: dict(v) for k, v in sorted(report.items())}))
must = [report[k].get("differ", 0) + report[k].get("onlyA", 0) + report[k].get("onlyB", 0) for k in ("closures", "receipts-rr", "receipts-ip")] + [report["records"].get("valueSha256Differ", 0), report["records"].get("digestDiffer", 0)]
sys.exit(1 if any(must) else 0)
