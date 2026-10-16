"""Per test file: active lines (census counter when available), bytes and the data files it still embeds (after srcread)."""
import re, os, json, sys
from lib import *
inputs = {}
for l in read(INPUTS).strip().split("\n"):
    r = l.split("\t")
    inputs[r[3]] = (r[0], int(r[1]), int(r[2]))
def embeds(path):
    t = read(path)
    out = {}
    for m in re.finditer(r'include_(?:str|bytes)!\("([^"]+)"\)', blank(t) if False else t):
        q = os.path.normpath(os.path.join(os.path.dirname(path), m.group(1)))
        if os.path.exists(q):
            out[os.path.relpath(q, REPO)] = os.path.getsize(q)
    return out
def metrics(path):
    rel_ = os.path.relpath(path, REPO)
    kind, b, l = inputs.get(rel_, ("?", os.path.getsize(path), len(read(path).split("\n"))))
    return dict(path=rel_, lines=l, bytes=os.path.getsize(path), embeds=embeds(path))
if __name__ == "__main__":
    rows = []
    for p in test_sources():
        if "/🧪️tests/" in p or "worker-cell" in p or "outcome" in p or "sqlite" in p or "test-hosts" in p:
            m = metrics(REPO + "/" + p)
            rows.append(m)
    json.dump(rows, open(SCRATCH + "/metrics.json", "w"), ensure_ascii=False, indent=1)
    print(len(rows), sum(r["lines"] for r in rows), sum(r["bytes"] for r in rows), sum(sum(r["embeds"].values()) for r in rows))
