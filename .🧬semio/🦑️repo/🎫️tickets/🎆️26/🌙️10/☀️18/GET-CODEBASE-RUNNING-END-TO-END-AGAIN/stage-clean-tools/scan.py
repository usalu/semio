#!/usr/bin/env python3
"""Classify every file inside the ticket's staged* dirs against the live tree and HEAD.

Usage: python3 scan.py <repo-root> <ticket-dir> <out.json>
Walks only the ticket dir (no symlink following, skips the generated folder).
"""
import hashlib, json, os, re, subprocess, sys

root, C, out = sys.argv[1], sys.argv[2], sys.argv[3]
GEN = "🗑️generated"
STAGED_RE = re.compile(r"^staged(?!.*-tools$)")


def run(*a):
    return subprocess.run(a, cwd=root, capture_output=True, check=True).stdout


head = {}
for rec in run("git", "ls-tree", "-r", "-z", "HEAD").split(b"\0"):
    if not rec:
        continue
    meta, path = rec.split(b"\t", 1)
    head[path.decode()] = meta.split()[2].decode()
index = {}
for rec in run("git", "ls-files", "-s", "-z").split(b"\0"):
    if not rec:
        continue
    meta, path = rec.split(b"\t", 1)
    index[path.decode()] = meta.split()[1].decode()
TOP = {p.split("/")[0] for p in head} | {p.split("/")[0] for p in index}
TOP |= {n for n in os.listdir(root)}
TOP -= {"C"}


def gsha(data):
    return hashlib.sha1(b"blob %d\0" % len(data) + data).hexdigest()


roots = []
for dp, dns, fns in os.walk(C, followlinks=False):
    rel = os.path.relpath(dp, C)
    dns[:] = [d for d in dns if d != GEN]
    if rel == ".":
        continue
    parts = rel.split("/")
    if any(STAGED_RE.match(p) for p in parts[:-1]):
        continue
    if STAGED_RE.match(parts[-1]) and len(parts) >= 2:
        roots.append(rel)
    elif STAGED_RE.match(parts[-1]) and len(parts) == 1:
        roots.append(rel)
roots.sort()

cp = os.path.relpath(C, root)
pres = tuple(cp + "/" + r + "/" for r in roots)
head = {p: o for p, o in head.items() if not p.startswith(pres)}
index = {p: o for p, o in index.items() if not p.startswith(pres)}
cdup_head = {p: o for p, o in head.items() if p.startswith(cp + "/")}
untracked = [
    x.decode()
    for x in run("git", "ls-files", "-o", "--exclude-standard", "-z").split(b"\0")
    if x
]
untracked = [p for p in untracked if not p.startswith(pres)]
corpus = {p: o for p, o in list(head.items()) + list(index.items()) if not p.startswith(cp + "/")}
for p in untracked:
    if not p.startswith(cp + "/"):
        corpus[p] = None
head_by_oid = {}
for p, o in corpus.items():
    if o:
        head_by_oid.setdefault(o, []).append(p)
cdup_by_oid = {}
for p, o in cdup_head.items():
    cdup_by_oid.setdefault(o, []).append(p)
by_base = {}
for p in corpus:
    by_base.setdefault(os.path.basename(p), []).append(p)
SCRIPT_RE = re.compile(
    r"(^(apply|plan|manifest|ops|edits|report|log|bases|anchors|stamp|oracle|overlay|make|gen|check|test|verify|seed|probe)[-_.\w]*\.(py|sh|json|mjs|cjs|txt)$)|\.(py|sh|diff|patch|mjs|cjs)$|^(new-files|sets|cases|targets|roster|derived)[-\w.]*\.json$|^(plan|manifest|ops|edits|apply)[-\w.]*\.(json|ts)$|^(apply|plan)\w*\.ts$"
)


def lineset(data):
    try:
        t = data.decode("utf-8")
    except Exception:
        return None
    return {l.strip() for l in t.splitlines() if l.strip()}


def fuzzy(name, data):
    ls = lineset(data)
    if ls is None or len(ls) < 3:
        return None
    best, bp = 0.0, None
    for cand in by_base.get(name, [])[:3000]:
        lp = os.path.join(root, cand)
        try:
            if not os.path.isfile(lp) or os.path.getsize(lp) > 4 * len(data) + 4096:
                continue
            cs = lineset(open(lp, "rb").read())
        except OSError:
            continue
        if not cs:
            continue
        j = len(ls & cs) / len(ls | cs)
        if j > best:
            best, bp = j, cand
    return (bp, best) if best >= 0.5 else None

PATHKEYS = ("path", "file", "target", "dest", "repoPath", "to")
HEX = re.compile(r"[0-9a-f]{64}")
IDRE = re.compile(r"^[\w./-]{2,}$")


def harvest(obj, out):
    if isinstance(obj, dict):
        pv = next((obj[k] for k in PATHKEYS if isinstance(obj.get(k), str) and obj[k].split("/")[0] in TOP and "/" in obj[k]), None)
        if pv:
            for k, v in obj.items():
                if k not in PATHKEYS and isinstance(v, str) and (HEX.fullmatch(v) or IDRE.match(v)) and v != pv:
                    out.setdefault(v, set()).add(pv)
        for v in obj.values():
            harvest(v, out)
    elif isinstance(obj, list):
        for v in obj:
            harvest(v, out)


refmaps = {}
for r in roots:
    m = {}
    for dp, dns, fns in os.walk(os.path.join(C, r), followlinks=False):
        for fn in fns:
            if fn.endswith(".json") and os.path.getsize(os.path.join(dp, fn)) < 40_000_000:
                try:
                    harvest(json.load(open(os.path.join(dp, fn))), m)
                except Exception:
                    pass
    refmaps[r] = {k: next(iter(v)) for k, v in m.items() if len(v) == 1}
# nested roots share parent's maps via their own walk; global hex map as fallback
globalhex = {}
for r, m in refmaps.items():
    for k, v in m.items():
        if HEX.fullmatch(k):
            globalhex.setdefault(k, v)

rows = []
for r in roots:
    base = os.path.join(C, r)
    for dp, dns, fns in os.walk(base, followlinks=False):
        for fn in fns:
            full = os.path.join(dp, fn)
            rel = os.path.relpath(full, base)
            row = {"root": r, "rel": rel}
            if os.path.islink(full):
                row.update(cls="symlink", live=None)
                rows.append(row)
                continue
            data = open(full, "rb").read()
            row["size"] = len(data)
            row["sha1"] = gsha(data)
            parts = rel.split("/")
            live = None
            how = None
            rm = refmaps[r]
            sh = hashlib.sha256(data).hexdigest()
            for k in (sh, fn, rel, os.path.splitext(fn)[0], rel.split("/", 1)[-1], "/".join(parts[-2:])):
                if k in rm:
                    fe, le = os.path.splitext(fn)[1], os.path.splitext(rm[k])[1]
                    if k != sh and fe in (".ts", ".json", ".rs", ".md", ".tsx") and fe != le:
                        continue
                    live, how = rm[k], "ref:" + ("sha256" if k == sh else "id")
                    break
            if live is None and sh in globalhex:
                live, how = globalhex[sh], "ref:sha256-global"
            if live is None:
                for i in range(len(parts)):
                    if parts[i] in TOP:
                        cand = "/".join(parts[i:])
                        if (
                            cand in head
                            or cand in index
                            or os.path.lexists(os.path.join(root, cand))
                        ):
                            live, how = cand, "suffix"
                            break
            shaped = None
            if live is None:
                for i in range(len(parts) - 1):
                    if parts[i] in TOP:
                        shaped = "/".join(parts[i:])
                        break
            row["map"] = how
            if live is not None:
                lp = os.path.join(root, live)
                row["live"] = live
                oid = head.get(live)
                row["head_eq"] = oid == row["sha1"] if oid else None
                if os.path.isfile(lp):
                    ld = open(lp, "rb").read()
                    row["live_eq"] = ld == data
                    row["live_sha1"] = gsha(ld)
                else:
                    row["live_eq"] = None
                if row["live_eq"]:
                    row["cls"] = "identical-live"
                elif row["live_eq"] is None:
                    if row["head_eq"]:
                        row["cls"] = "head-live-gone"
                    else:
                        row["cls"] = "orphan"
                elif row["head_eq"]:
                    row["cls"] = "head-live-changed"
                else:
                    row["cls"] = "different"
            else:
                hm = head_by_oid.get(row["sha1"])
                fz = None
                if hm:
                    live, how = hm[0], "hash"
                    row["moved_to"] = hm[:3]
                elif shaped:
                    row["cls"] = "orphan"
                    row["shaped"] = shaped
                elif SCRIPT_RE.search(fn):
                    cd = cdup_by_oid.get(row["sha1"])
                    row["cls"] = "ticket-dup" if cd else "tool-input"
                    if cd:
                        row["live"] = cd[0]
                        row["live_eq"] = os.path.isfile(os.path.join(root, cd[0])) and open(os.path.join(root, cd[0]), "rb").read() == data
                        if not row["live_eq"]:
                            row["cls"] = "tool-input"
                else:
                    fz = fuzzy(fn, data)
                    if fz:
                        live, how = fz[0], "fuzzy%.2f" % fz[1]
                    else:
                        row["cls"] = "orphan"
                if live is not None:
                    row["map"] = how
                    lp = os.path.join(root, live)
                    row["live"] = live
                    oid = head.get(live) or index.get(live)
                    row["head_eq"] = (oid == row["sha1"]) if oid else None
                    row["live_eq"] = open(lp, "rb").read() == data if os.path.isfile(lp) else None
                    if row["live_eq"]:
                        row["cls"] = "identical-live"
                    elif row["live_eq"] is None:
                        row["cls"] = "head-live-gone" if row["head_eq"] else "orphan"
                    elif row["head_eq"]:
                        row["cls"] = "head-live-changed"
                    else:
                        row["cls"] = "different"
            lv = row.get("live")
            if lv and row["cls"] in ("identical-live", "different", "head-live-changed", "head-live-gone") and any(x.startswith("staged") for x in lv.split("/")[:-1]):
                row["cls"] = "orphan"
                row["note"] = "counterpart is another staged copy: " + lv
            if row["cls"] == "orphan" and len(data) > 1_048_576:
                row["cls"] = "tool-data"
            rows.append(row)
json.dump({"roots": roots, "rows": rows}, open(out, "w"), ensure_ascii=False, indent=0)
import collections

c = collections.Counter(r["cls"] for r in rows)
print(len(roots), "roots", len(rows), "files", dict(c))
