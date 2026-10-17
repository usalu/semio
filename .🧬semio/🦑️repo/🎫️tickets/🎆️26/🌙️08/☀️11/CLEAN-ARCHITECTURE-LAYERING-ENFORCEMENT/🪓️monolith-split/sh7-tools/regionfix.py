"""🧭 sh7 region repair: make every split child balanced per file by repeating spanning `//#region` pairs.
usage: regionfix.py plan|apply|verify [group ...]   (groups: bridge host model components cursors scene interaction unit unit2)
Pairing is by nesting (what IDE folding does), labels are ignored. Only comment lines are added; nothing else is touched."""
import collections, os, re, sys

F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ROOT = "/Users/ueli/Documents/semio"
sys.path.insert(0, F + "/c4p1-tools")
from plans import PLANS, ORDER

MARK = re.compile(r"^(\s*)//\s?#(region|endregion)(?:\s+(.*))?$")
MOUNT = re.compile(r'^\s*(?:#\[\s*path\s*=\s*"([^"]+)"\s*\]|include!\(\s*"([^"]+)"\s*\);)\s*$')


def lines_of(path):
    t = open(path, encoding="utf-8").read()
    ls = t.split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def group_files(key):
    spec = PLANS[key]
    root = ROOT + "/" + spec["rel"]
    folders = {c["folder"] for c in spec["children"]}
    return root, folders


def walk(path, folders, root, out, spans):
    ls = lines_of(path)
    start = len(out)
    spans.setdefault(path, []).append(["enter", len(out)])
    for i, l in enumerate(ls):
        m = MARK.match(l)
        if m:
            out.append(dict(file=path, line=i, kind="o" if m.group(2) == "region" else "c", label=(m.group(3) or "").strip()))
            continue
        mm = MOUNT.match(l)
        if mm:
            rel = mm.group(1) or mm.group(2)
            if rel.split("/")[0] in folders:
                if path == root:
                    spans.setdefault("order", []).append(os.path.normpath(os.path.join(os.path.dirname(path), rel)))
                walk(os.path.normpath(os.path.join(os.path.dirname(path), rel)), folders, root, out, spans)
    spans[path][-1].append(len(out))


def pair(events):
    stack, pairs, uclose = [], {}, []
    for idx, e in enumerate(events):
        if e["kind"] == "o":
            stack.append(idx)
        elif stack:
            pairs[stack.pop()] = idx
        else:
            uclose.append(idx)
    return pairs, uclose, stack


def plan_group(key):
    root, folders = group_files(key)
    ev, spans = [], {}
    walk(root, folders, root, ev, spans)
    pairs, uclose, dangling = pair(ev)
    dangling = set(dangling)
    closeof = dict(pairs)
    order = spans.get("order", [])
    notes = []
    for o, c in pairs.items():
        fo, fc = ev[o]["file"], ev[c]["file"]
        if fo != fc and (fo == root or fc == root):
            notes.append(f"ROOT-CROSS pair open {os.path.basename(os.path.dirname(fo))}:{ev[o]['line']+1} close {os.path.basename(os.path.dirname(fc))}:{ev[c]['line']+1} {ev[o]['label']}")
    plans = {}
    for f in order:
        s, e_ = spans[f][0][1], spans[f][0][2]
        stack = []
        for i in range(0, s):
            if ev[i]["kind"] == "o":
                stack.append(i)
            elif stack:
                stack.pop()
        entry = [o for o in stack if o not in dangling]
        pre, suf = [], []
        for u in reversed(uclose):
            if s <= u < e_:
                pre.append(("o", ev[u]["label"]))
        for o in entry:
            c = closeof.get(o)
            label = ev[o]["label"]
            if c is not None and s <= c < e_ and ev[c]["label"]:
                label = ev[c]["label"]
            pre.append(("o", label))
        for i in range(s, e_):
            if ev[i]["kind"] == "o":
                stack.append(i)
            elif stack:
                stack.pop()
        for o in reversed(stack):
            if o in dangling:
                if s <= o < e_:
                    suf.append(("c", ev[o]["label"]))
            else:
                suf.append(("c", ev[o]["label"]))
        plans[f] = dict(pre=pre, suf=suf, n=e_ - s)
    return root, plans, order, notes


def header_end(ls, is_fragment):
    if is_fragment:
        i = 1
    else:
        i = next(k for k, l in enumerate(ls) if l.strip() == "use super::*;") + 1
    while i < len(ls) and ls[i].strip() == "":
        i += 1
    return i


def local_check(ls):
    depth, bad = 0, False
    for l in ls:
        m = MARK.match(l)
        if m:
            if m.group(2) == "region":
                depth += 1
            else:
                depth -= 1
                if depth < 0:
                    return False
    return depth == 0


def main():
    mode = sys.argv[1]
    groups = sys.argv[2:] or ORDER
    bad = 0
    for key in groups:
        root, plans, order, notes = plan_group(key)
        frag = PLANS[key]["mode"] == "include"
        print(f"== {key}: {len(order)} files")
        for n in notes:
            print("  NOTE", n)
        for f in order:
            p = plans[f]
            ls = lines_of(f)
            ok = local_check(ls)
            rel = os.path.relpath(f, ROOT)[-60:]
            print(f"  child markers={p['n']:2} balanced={ok!s:5} pre={[l for _, l in p['pre']]} suf={[l for _, l in p['suf']]} {rel}")
            if mode == "apply" and (p["pre"] or p["suf"]):
                new = list(ls)
                if p["pre"]:
                    at = header_end(new, frag)
                    new[at:at] = [f"//#region {lab}" for _, lab in p["pre"]]
                if p["suf"]:
                    new += [f"//#endregion {lab}" for _, lab in p["suf"]]
                open(f, "w", encoding="utf-8").write("\n".join(new) + "\n")
            if mode == "verify" and not ok:
                bad += 1
    if mode == "verify":
        print("UNBALANCED CHILDREN:", bad)
        sys.exit(1 if bad else 0)


main()
