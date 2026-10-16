"""Per-host closure sizes of the test mounts of a crate and a first-fit-decreasing packing into companions. usage: pack.py <crate-id> [max-bytes] [max-files] [max-lines]"""
import os, re, sys, json
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import genmirror, modtree, union
from stubgen import mask

INC = re.compile(r'include_(?:str|bytes)!\s*\(\s*(?:concat!\(\s*)?"([^"]+)"')


def measure(cid):
    c = union.cfg_of(cid)
    lib = f"{union.W}/{cid}/snap/{c['p_lib']}"
    rows = []
    for host_mp, name, host_file, d, tfile in genmirror.test_mounts(lib):
        files = genmirror.subtree_files(lib, host_mp + (name,))
        emb = set()
        nlines = 0
        nbytes = 0
        for f in files:
            t = open(f, encoding="utf-8").read()
            nbytes += len(t.encode("utf-8"))
            nlines += sum(1 for l in t.split("\n") if l.strip() and not l.strip().startswith("//"))
            for m in INC.finditer(t):
                p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
                if os.path.exists(p):
                    emb.add(p)
        rows.append(dict(host="::".join(host_mp) or "(root)", name=name, files=files, emb=emb, lines=nlines, bytes=nbytes + sum(os.path.getsize(p) for p in emb)))
    return rows


if __name__ == "__main__":
    cid = sys.argv[1]
    maxb = int(sys.argv[2]) if len(sys.argv) > 2 else 245000
    maxf = int(sys.argv[3]) if len(sys.argv) > 3 else 28
    maxl = int(sys.argv[4]) if len(sys.argv) > 4 else 3500
    rows = measure(cid)
    for r in sorted(rows, key=lambda r: -r["bytes"]):
        print(f"{r['bytes']:>8} {r['lines']:>5} files={len(r['files'])} emb={len(r['emb'])}  {r['host']}::{r['name']}")
    bins = []
    for r in sorted(rows, key=lambda r: -r["bytes"]):
        for b in bins:
            files = set(b["files"]) | set(r["files"]) | b["emb"] | r["emb"]
            emb = b["emb"] | r["emb"]
            nb = sum(os.path.getsize(f) for f in files)
            nl = b["lines"] + r["lines"]
            if len(files) + 1 <= maxf and nb <= maxb and nl <= maxl:
                b["hosts"].append(r["host"] + "::" + r["name"]); b["files"] = set(b["files"]) | set(r["files"]); b["emb"] = emb; b["lines"] = nl
                break
        else:
            bins.append(dict(hosts=[r["host"] + "::" + r["name"]], files=set(r["files"]), emb=set(r["emb"]), lines=r["lines"]))
    for i, b in enumerate(bins):
        files = set(b["files"]) | b["emb"]
        print(f"bin {i+1}: files={len(files)+1} bytes={sum(os.path.getsize(f) for f in files)} lines={b['lines']} hosts={b['hosts']}")
