#!/usr/bin/env python3
"""Write one unified diff per staged root for files that differ from live (or have no counterpart),
verify every diff round-trips to the staged bytes, and emit the inventory table.

Usage: python3 patch.py <repo-root> <ticket-dir> <inv.json> <residual-dir> <summary.json>
"""
import difflib, hashlib, json, os, sys, collections

root, C, invp, resid, summ = sys.argv[1:6]
inv = json.load(open(invp))
PATCHED = ("different", "head-live-changed", "head-live-gone", "orphan")
DELETE = ("identical-live",) + PATCHED


def split(t):
    out = t.split("\n")
    last = out.pop()
    lines = [l + "\n" for l in out]
    if last:
        lines.append(last)
    return lines


def udiff(old, new, fa, fb):
    out = []
    for l in difflib.unified_diff(old, new, fa, fb, n=3, lineterm="\n"):
        if not l.endswith("\n"):
            l += "\n\\ No newline at end of file\n"
        out.append(l)
    return out


def apply(old, patch_lines):
    res, i, pos, last = [], 0, 0, None
    pl = patch_lines
    while i < len(pl) and not pl[i].startswith("@@"):
        i += 1
    while i < len(pl):
        h = pl[i]
        a = h.split(" ")[1]
        start, _, cnt = a[1:].partition(",")
        start = int(start)
        cnt = int(cnt) if cnt else 1
        s0 = start - 1 if cnt else start
        res.extend(old[pos:s0])
        pos = s0
        i += 1
        while i < len(pl) and not pl[i].startswith("@@"):
            l = pl[i]
            if l.startswith("\\"):
                if last in ("ctx", "new"):
                    res[-1] = res[-1].rstrip("\n")
                i += 1
                continue
            c, body = l[0], l[1:]
            if c == " ":
                res.append(old[pos]); pos += 1; last = "ctx"
            elif c == "-":
                pos += 1; last = "old"
            elif c == "+":
                res.append(body); last = "new"
            i += 1
    res.extend(old[pos:])
    return res



def strip_nonl(patch_lines):
    return patch_lines


by_root = collections.defaultdict(list)
for r in inv["rows"]:
    by_root[r["root"]].append(r)

summary = {}
bad = []
os.makedirs(resid, exist_ok=True)
for rt, rows in sorted(by_root.items()):
    slug = rt.replace("/", "__")
    pr = [r for r in rows if r["cls"] in PATCHED]
    cnt = collections.Counter(r["cls"] for r in rows)
    entry = {"files": len(rows), "classes": dict(cnt), "patch": None, "patch_bytes": 0}
    if pr:
        chunks = []
        for r in sorted(pr, key=lambda x: x["rel"]):
            sp = os.path.join(C, rt, r["rel"])
            new_t = open(sp, "rb").read().decode("utf-8")
            live = r.get("live")
            lp = os.path.join(root, live) if live else None
            if live and r["cls"] != "orphan" and r["cls"] != "head-live-gone" and os.path.isfile(lp):
                old_t = open(lp, "rb").read().decode("utf-8")
                fa, fb = "a/" + live, "b/" + live
            else:
                old_t = ""
                fa = "/dev/null"
                fb = "b/" + (live or r.get("shaped") or "staged/" + rt + "/" + r["rel"])
            if r.get("note"):
                pass
            old, new = split(old_t), split(new_t)
            d = udiff(old, new, fa, fb)
            hdr = "# stage-clean %s staged=%s/%s map=%s\n" % (r["cls"], rt, r["rel"], r.get("map"))
            if not d:
                d = []
            ok = True
            if d:
                got = "".join(apply(old, split("".join(d))))
                ok = got == new_t
            if not ok:
                bad.append((rt, r["rel"]))
            chunks.append(hdr + "".join(d))
            r["patched"] = True
        body = "".join(chunks)
        pp = os.path.join(resid, slug + ".patch")
        open(pp, "w", encoding="utf-8").write(body)
        entry["patch"] = os.path.relpath(pp, C)
        entry["patch_bytes"] = len(body.encode("utf-8"))
    summary[rt] = entry
json.dump({"summary": summary, "bad": bad}, open(summ, "w"), ensure_ascii=False, indent=1)
json.dump(inv, open(invp, "w"), ensure_ascii=False)
print("roots", len(summary), "patches", sum(1 for e in summary.values() if e["patch"]), "bytes", sum(e["patch_bytes"] for e in summary.values()), "roundtrip-bad", len(bad))
