"""✅ c4p11 live-tree checks: paths, reachability, duplicate mounts, docstring emoji, regions, test names, sizes.
usage: c4pcheck.py"""
import collections, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER
import splitter as S

RX_TEST = re.compile(r"#\[\s*(?:[\w:]*::)?(?:test|async_test|tokio::test)\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)")
EMOJI_FIRST = re.compile(r"^//[!/]? (\S+)")
ok = True


def mounts(text, base):
    t = S.mask_keep_strings(text)
    out = []
    for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', t):
        out.append(("path", os.path.normpath(os.path.join(base, m.group(1)))))
    for m in re.finditer(r'\binclude!\(\s*"([^"]+)"\s*\)', t):
        out.append(("include", os.path.normpath(os.path.join(base, m.group(1)))))
    return out


def closure(lib_abs):
    seen, order = set(), []

    def visit(p):
        p = os.path.normpath(p)
        if p in seen or not os.path.isfile(p):
            return
        seen.add(p)
        order.append(p)
        for kind, target in mounts(read(p), os.path.dirname(p)):
            visit(target)
    visit(lib_abs)
    return order


def regions(ls):
    o, c = collections.Counter(), collections.Counter()
    for l in ls:
        m = re.match(r"^\s*//\s?#region (.*)$", l)
        if m:
            o[m.group(1).strip()] += 1
        m = re.match(r"^\s*//\s?#endregion (.*)$", l)
        if m:
            c[m.group(1).strip()] += 1
    return o, c


rows = []
for key in ORDER:
    spec = PLANS[key]
    base_dir = os.path.dirname(spec["rel"])
    root_abs = ROOT + "/" + spec["rel"]
    files = [spec["rel"]] + [f"{base_dir}/{c['folder']}/🦀️.rs" for c in spec["children"]]
    cl = closure(ROOT + "/" + (spec["lib"] or spec["rel"]))
    cl_rel = [os.path.relpath(p, ROOT) for p in cl]
    cnt = collections.Counter(cl_rel)
    missing = []
    for p in cl:
        for kind, target in mounts(read(p), os.path.dirname(p)):
            if not os.path.isfile(target):
                missing.append((os.path.relpath(p, ROOT)[-50:], kind, os.path.relpath(target, ROOT)[-60:]))
    firsts = collections.defaultdict(list)
    for rel in files[1:]:
        ls = lines_of(read(ROOT + "/" + rel))
        m = EMOJI_FIRST.match(ls[0])
        first = m.group(1) if m else None
        o, c = regions(ls)
        firsts[first].append(rel)
        bad = []
        if len(rel.encode("utf-8")) > 240:
            bad.append("path>240")
        if len(ls) > 1500:
            bad.append("lines>1500")
        if cnt.get(rel, 0) != 1:
            bad.append(f"reachable={cnt.get(rel, 0)}")
        if first is None:
            bad.append("no emoji doc")
        rows.append((key, rel.split("/")[-2], len(ls), len(rel.encode("utf-8")), cnt.get(rel, 0), first, sum(o.values()), sum(c.values()), bad))
        if bad:
            ok = False
    root_first = EMOJI_FIRST.match(lines_of(read(root_abs))[0])
    rf = root_first.group(1) if root_first else None
    if rf in firsts:
        ok = False
        print(f"[{key}] child docstring emoji repeats the root's: {rf}")
    dup = {k: v for k, v in firsts.items() if len(v) > 1}
    if dup:
        ok = False
    # region pairing: a region with both ends in the original must keep both ends in one file
    orig = lines_of(base_text(spec["rel"]))
    oo, oc = regions(orig)
    per_file = {}
    for rel in files:
        per_file[rel] = regions(lines_of(read(ROOT + "/" + rel)))
    split_regions = []
    for name in set(oo) | set(oc):
        if oo[name] and oc[name]:
            holders = [rel for rel, (o, c) in per_file.items() if o[name] or c[name]]
            both = [rel for rel, (o, c) in per_file.items() if o[name] and c[name]]
            if not both:
                split_regions.append(name)
    tot_o = sum(sum(o.values()) for o, c in per_file.values())
    tot_c = sum(sum(c.values()) for o, c in per_file.values())
    # duplicate mounts: any file mounted more than once in the closure
    dups = {k: v for k, v in cnt.items() if v > 1}
    # test names
    a = sorted(m.group(1) for m in RX_TEST.finditer("\n".join(orig)))
    b = sorted(m.group(1) for rel in files for m in RX_TEST.finditer(read(ROOT + "/" + rel)))
    same_tests = a == b
    if missing or dups or split_regions or not same_tests or (tot_o, tot_c) != (sum(oo.values()), sum(oc.values())):
        ok = False
    print(f"[{key}] files={len(files)} closure={len(cl)} missing={missing} duplicate-mounts={dups} duplicate-first-emoji={dup} regions orig=({sum(oo.values())},{sum(oc.values())}) new=({tot_o},{tot_c}) region-pairs-split={split_regions} test-fns {len(a)}=={len(b)} identical={same_tests}")
for r in rows:
    print(f"  {r[0]:11} {r[2]:5} lines {r[3]:3} bytes reach={r[4]} first={r[5]} regions=({r[6]},{r[7]}) {r[1]} {r[8] or ''}")
print("MAXLINES", max(r[2] for r in rows), "MAXBYTES", max(r[3] for r in rows))
print("CHECKS", "OK" if ok else "FAILED")
