"""Static verification of the osm splits against the pre-split snapshot. usage: verify.py <stage|live> [targets]"""
import collections, os, re, subprocess, sys, unicodedata
sys.path.insert(0, os.path.dirname(__file__))
from osmlib import *
import osmeng, plans

mode = sys.argv[1]
names = sys.argv[2].split(",") if len(sys.argv) > 2 else ["services", "transport", "remote", "bridge"]
LS = set(l.rstrip("\n") for l in open(f"{F}/🗑️generated/osm-ls-files.txt", encoding="utf-8"))
used_emoji = collections.Counter()
for l in LS:
    for p in l.split("/")[:-1]:
        if p and "️" in p[:4]:
            used_emoji[p[:p.index("️") + 1]] += 1


def tree_root(snap):
    return f"{W}/stage-{snap}" if mode == "stage" else ROOT


def snap_dir(snap):
    return f"{W}/snap-{snap}"


def rustfmt_ok(path):
    p = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config-path", f"{ROOT}/rustfmt.toml"], input=read(path), capture_output=True, text=True)
    return p.returncode in (0, 1), p.stderr[:300]


def regions(text):
    st, bad, names = [], [], []
    for i, l in enumerate(lines_of(text), 1):
        s = l.strip()
        if s.startswith("//#region"):
            st.append(s[9:].strip()); names.append(s[9:].strip().split(" — ")[0])
        elif s.startswith("//#endregion"):
            if not st:
                bad.append(i)
            else:
                st.pop()
    return (not st and not bad), names


NORM_MARK = re.compile(r"^\s*//#(?:end)?region\b")
ok_all = True


def report(msg, ok=True):
    global ok_all
    ok_all = ok_all and ok
    print(("PASS " if ok else "FAIL ") + msg)


for name in names:
    files = plans.GROUPS[name]()
    snap = "services" if name == "services" else "mcp"
    tr = tree_root(snap)
    for f in files:
        base = os.path.dirname(f.rel)
        orig = lines_of(read(os.path.join(snap_dir(snap), f.rel)))
        newfiles = [f.rel] + [os.path.join(base, c["dir"], "🦀️.rs") for c in f.children]
        newlines = []
        info = []
        for nf in newfiles:
            p = os.path.join(tr, nf)
            t = read(p)
            ls = lines_of(t)
            newlines += ls
            ok, nm = regions(t)
            fmt, err = rustfmt_ok(p)
            first = ls[0] if ls else ""
            doc_ok = first.startswith("//! ") and unicodedata.category(first[4]) in ("So", "Sk", "Sm") or ord(first[4]) > 0x2000
            info.append((nf, len(ls), ok, fmt, doc_ok, len(nf.encode("utf-8"))))
        for nf, n, ok, fmt, doc_ok, plen in info:
            if not ok or not fmt or not doc_ok or n > 1500 or plen > 240:
                report(f"{nf[-80:]} lines={n} regions_balanced={ok} rustfmt_parse={fmt} doc_emoji={doc_ok} pathbytes={plen}", False)
        report(f"[{f.label}] {len(newfiles)} files: max lines {max(i[1] for i in info)}, longest path {max(i[5] for i in info)} bytes, all regions balanced={all(i[2] for i in info)}, all rustfmt-parse={all(i[3] for i in info)}, all start with emoji doc={all(i[4] for i in info)}", all(i[2] and i[3] and i[4] and i[1] <= 1500 and i[5] <= 240 for i in info))
        # lossless multiset
        o = multiset([l for l in orig if not NORM_MARK.match(l)])
        n = multiset([l for l in newlines if not NORM_MARK.match(l)])
        removed, added = o - n, n - o
        print(f"   multiset: original distinct={len(o)} total={sum(o.values())}; new total={sum(n.values())}; removed-from-original={sum(removed.values())} added={sum(added.values())}")
        norm = lambda l: l.replace("pub(super) ", "").replace('"../', '"').replace("../", "") if True else l
        o2 = collections.Counter()
        for l in orig:
            if NORM_MARK.match(l) or not l.strip():
                continue
            o2[norm(l.strip())] += 1
        n2 = collections.Counter()
        for l in newlines:
            if NORM_MARK.match(l) or not l.strip():
                continue
            n2[norm(l.strip())] += 1
        r2, a2 = o2 - n2, n2 - o2
        write(f"{F}/🗑️generated/osm-verify-{f.label}-removed.txt", "\n".join(f"{c}x {k}" for k, c in sorted(r2.items())))
        write(f"{F}/🗑️generated/osm-verify-{f.label}-added.txt", "\n".join(f"{c}x {k}" for k, c in sorted(a2.items())))
        print(f"   after normalising pub(super)/../ edits: removed={sum(r2.values())} added={sum(a2.values())}  (logs osm-verify-{f.label}-removed/added.txt)")
    # path resolution and duplicate mounts over the whole tree
    pr = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
    def mounts(root):
        mc = collections.Counter()
        missing = []
        for dp, dn, fn in os.walk(os.path.join(root, M + "/🛎️services" if name == "services" else M + "/🌉️mcp")):
            dn[:] = [d for d in dn if d not in SKIP]
            for fx in fn:
                if not fx.endswith(".rs"):
                    continue
                p = os.path.join(dp, fx)
                t = read(p)
                for m in pr.finditer(t):
                    tgt = os.path.normpath(os.path.join(dp, m.group(1)))
                    mc[tgt] += 1
                    if not os.path.exists(tgt):
                        missing.append((p, m.group(1)))
        return mc, missing
    mc_new, miss_new = mounts(tr)
    mc_old, miss_old = mounts(snap_dir(snap))
    dup_new = {k for k, v in mc_new.items() if v > 1}
    dup_old = {k for k, v in mc_old.items() if v > 1}
    report(f"[{name}] #[path] mounts: new tree {sum(mc_new.values())} (old {sum(mc_old.values())}); missing targets new={len(miss_new)} old={len(miss_old)}; duplicate-mounted targets new={len(dup_new)} old={len(dup_old)}; new duplicates={len(dup_new - dup_old)}", len(dup_new - dup_old) == 0 and len(miss_new) <= len(miss_old))
    for k in sorted(dup_new - dup_old):
        print("   NEW DUP", k)
    for (p, m) in miss_new:
        if (p.replace(tr, snap_dir(snap)), m) not in set(miss_old):
            print("   NEW MISSING", p[-70:], m)
    # sibling emoji uniqueness and registered emoji
    for f in files:
        base = os.path.join(tr, os.path.dirname(f.rel))
        sibs = [d for d in os.listdir(base) if os.path.isdir(os.path.join(base, d))]
        em = collections.defaultdict(list)
        for d in sibs:
            if "️" in d[:4]:
                em[d[:d.index("️") + 1]].append(d)
        newd = [c["dir"] for c in f.children]
        for d in newd:
            e = d[:d.index("️") + 1]
            okk = len(em[e]) == 1 and used_emoji.get(e, 0) > 0 and d[len(e):len(e) + 1] != ""
            report(f"   dir {d}: unique among siblings={len(em[e])==1}, emoji used elsewhere in repo={used_emoji.get(e,0)}", okk)
print("ALL PASS" if ok_all else "SOME FAIL")
