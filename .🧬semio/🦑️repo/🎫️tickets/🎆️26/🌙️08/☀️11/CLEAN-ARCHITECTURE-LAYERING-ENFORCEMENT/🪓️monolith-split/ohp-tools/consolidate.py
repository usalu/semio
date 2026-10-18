"""🧹 Merges the single-name `use` lines this fleet tool appended (lines added versus HEAD that match `use a::b::Leaf;`) into the file's existing braced `use` of the same path, drops names already imported, and groups the rest into one braced `use` per crate.
usage: consolidate.py <repo-rel file>... [--dry]"""
import re, subprocess, sys, collections, os
R = "/Users/ueli/Documents/semio"
dry = "--dry" in sys.argv
for rel in [x for x in sys.argv[1:] if not x.startswith("--")]:
    p = os.path.join(R, rel)
    cur = open(p, encoding="utf-8").read().split("\n")
    head = subprocess.run(["git", "-c", "core.fsmonitor=false", "show", "HEAD:" + rel], cwd=R, capture_output=True, text=True).stdout.split("\n")
    hs = collections.Counter(head)
    added_idx = []
    seen = collections.Counter()
    for i, l in enumerate(cur):
        seen[l] += 1
        if re.match(r"^use [A-Za-z0-9_:]+::[A-Za-z0-9_]+;$", l) and seen[l] > hs.get(l, 0):
            added_idx.append(i)
    if not added_idx: print(rel.split("artifacts/")[-1][-60:], "nothing to merge"); continue
    singles = [cur[i][4:-1] for i in added_idx]
    rest = [l for i, l in enumerate(cur) if i not in set(added_idx)]
    text = "\n".join(rest)
    keep = collections.OrderedDict()
    for s in singles:
        prefix, leaf = s.rsplit("::", 1)
        if re.search(r"^use [^;]*\b" + re.escape(leaf) + r"\b[^;]*;", text, re.M) or re.search(r"^\s+pub use [^;]*\b" + re.escape(leaf) + r"\b[^;]*;", text, re.M):
            continue
        segs = prefix.split("::")
        done_merge = False
        for k in range(len(segs), 0, -1):
            pre = "::".join(segs[:k]); remainder = "::".join(segs[k:] + [leaf])
            m = re.search(r"^use " + re.escape(pre) + r"::\{([^}]*)\};$", text, re.M)
            if m:
                names = [x.strip() for x in m.group(1).split(",") if x.strip()]
                if remainder not in names:
                    names.append(remainder)
                    text = text.replace(m.group(0), f"use {pre}::{{{', '.join(names)}}};", 1)
                done_merge = True
                break
        if done_merge:
            continue
        keep.setdefault(prefix, []).append(leaf)
    byc = collections.OrderedDict()
    for prefix, leaves in keep.items():
        c, _, sub = prefix.partition("::")
        for lf in leaves:
            byc.setdefault(c, []).append((sub + "::" if sub else "") + lf)
    newlines = [f"use {c}::{{{', '.join(v)}}};" if len(v) > 1 else f"use {c}::{v[0]};" for c, v in byc.items()]
    out = text.split("\n")
    pos = added_idx[0] - sum(1 for j in added_idx if j < added_idx[0])
    out[pos:pos] = newlines
    n0, n1 = len(cur), len(out)
    print(rel.split("artifacts/")[-1][-60:], f"{n0} -> {n1} lines; singles {len(singles)} -> {len(newlines)} lines")
    if not dry: open(p, "w", encoding="utf-8").write("\n".join(out))
