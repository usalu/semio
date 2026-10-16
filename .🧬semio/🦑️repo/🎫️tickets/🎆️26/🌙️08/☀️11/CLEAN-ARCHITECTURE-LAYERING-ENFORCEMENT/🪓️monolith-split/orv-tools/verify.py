"""Static verification of the staged (or installed) split: losslessness, item set, regions, paths, emoji, mod-name collisions, sizes.
usage: verify.py <name|all> [stage|real]"""
import collections, json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC
from split import src_path

MARK = re.compile(r"^\s*// ?#(end)?region ")
UNPREFIX = re.compile(r'(#\[path\s*=\s*"|include!\("|include_str!\("|include_bytes!\(")\.\./')


def norm_line(l, child=False):
    s = l.strip()
    s = s.replace("pub(super) ", "")
    if child:
        s = UNPREFIX.sub(lambda m: m.group(1), s)
    return s


def item_texts(text, skip=(), child=False):
    lines = text.split("\n")
    out = []
    for it in top_items(text):
        seg = [norm_line(l, child) for l in lines[it["first_line"]:it["end"] + 1] if norm_line(l, child) not in skip and not l.strip().startswith("#![")]
        seg = [l for l in seg if l and not MARK.match(l) and not (l.startswith("//") and not l.startswith("///"))]
        if seg:
            out.append("\n".join(seg))
    return out


def files_of(name, which):
    t = TARGETS[name]
    base = f"{W}/{name}/stage/{t['pkgdir']}" if which == "stage" else f"{ROOT}/{t['pkgdir']}"
    root = f"{base}/🦀️.rs"
    kids = [(c["folder"], f"{base}/{c['folder']}/🦀️.rs") for c in SPEC[name]["children"]]
    return base, root, kids


def resolve_exists(which, name, p, lit):
    target = os.path.normpath(os.path.join(os.path.dirname(p), lit))
    if os.path.isfile(target) or os.path.isdir(target):
        return True
    if which == "stage":
        t = TARGETS[name]
        stage_pk = f"{W}/{name}/stage/{t['pkgdir']}"
        real_pk = f"{ROOT}/{t['pkgdir']}"
        if p.startswith(stage_pk):
            alt = os.path.normpath(os.path.join(os.path.dirname(real_pk + p[len(stage_pk):]), lit))
            return os.path.isfile(alt) or os.path.isdir(alt)
    return False


def verify(name, which):
    t = TARGETS[name]
    spec = SPEC[name]
    base, root, kids = files_of(name, which)
    ok = True
    out = [f"== {name} ({which})"]
    orig = read(src_path(name))
    ol = lines_of(orig)
    rt = read(root)
    texts = {"<root>": rt}
    for f, p in kids:
        texts[f] = read(p)
    drops = [i for i in spec.get("drop_lines", [])]
    newl = []
    for k, v in texts.items():
        newl += [norm_line(l, k != "<root>") for l in lines_of(v)]
    mo = collections.Counter(norm_line(l) for l in ol if l.strip())
    mn = collections.Counter(l for l in newl if l.strip())
    lost = mo - mn
    extra = mn - mo
    allowed = collections.Counter(norm_line(ol[i - 1]) for i in drops)
    dropped_ok = lost & allowed
    lost = lost - allowed
    if dropped_ok:
        out.append(f"intentionally dropped lines: {dict(dropped_ok)}")
    out.append(f"lines: original {len(ol)} raw / {sum(mo.values())} non-blank; new {len(newl)} raw / {sum(mn.values())} non-blank")
    out.append(f"lost non-blank lines: {sum(lost.values())}")
    for l, n in lost.most_common(10): out.append(f"   LOST {n}x {l[:100]}")
    out.append(f"extra non-blank lines: {sum(extra.values())} (docstrings, uses, mounts, replaced mod lines)")
    if lost: ok = False
    orig_eff = "\n".join(l for i, l in enumerate(ol, 1) if i not in drops)
    io = collections.Counter(item_texts(orig_eff))
    inew = collections.Counter()
    for k, v in texts.items():
        inew.update(item_texts(v, child=(k != "<root>")))
    ilost = io - inew
    iextra = inew - io
    out.append(f"top-level items: original {sum(io.values())}, new {sum(inew.values())}; lost {sum(ilost.values())}; extra {sum(iextra.values())}")
    for l, n in ilost.most_common(5): out.append(f"   ITEM-LOST {n}x {l[:160]!r}")
    for l, n in iextra.most_common(12): out.append(f"   ITEM-EXTRA {n}x {l[:120]!r}")
    if spec.get("inline_items_ok"):
        pass
    elif sum(ilost.values()) > spec.get("items_lost_ok", 0): ok = False
    def regs(ls): return collections.Counter(l.strip() for l in ls if MARK.match(l))
    ro = regs(ol); rn = regs(newl)
    d = rn - ro
    dl = ro - rn
    out.append(f"region markers: original {sum(ro.values())} new {sum(rn.values())}; lost {dict(dl)}; added {dict(d)}")
    if dl: ok = False
    checked = 0; missing = []
    for k, v in texts.items():
        p = root if k == "<root>" else dict(kids)[k]
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]|include_str!\("([^"]+)"\)|include!\("([^"]+)"\)|include_bytes!\("([^"]+)"\)', v):
            lit = next(g for g in m.groups() if g)
            checked += 1
            if not resolve_exists(which, name, p, lit): missing.append((k, lit))
    out.append(f"path/include checks: {checked}, missing: {len(missing)} {missing[:5]}")
    if missing: ok = False
    for f, p in kids:
        if not (len(f) > 1 and f[1:2] == "️" or (len(f) > 2 and f[2:3] == "️")):
            out.append(f"   FOLDER-EMOJI-NO-VS16 {f}"); ok = False
        first = texts[f].split("\n")[0]
        em = next(c["emoji"] for c in spec["children"] if c["folder"] == f)
        if not first.startswith(f"//! {em} "):
            out.append(f"   DOCSTRING-EMOJI-MISMATCH {f}: {first[:40]}"); ok = False
        if len(f"{t['pkgdir']}/{f}/🦀️.rs".encode("utf-8")) > 240:
            out.append(f"   PATH-TOO-LONG {f}"); ok = False
    emo = [c["emoji"] for c in spec["children"]]
    if len(set(emo)) != len(emo): out.append("   DUPLICATE sibling emoji"); ok = False
    sibs = os.listdir(f"{ROOT}/{t['pkgdir']}")
    sibem = set()
    for s in sibs:
        i = 0
        while i < len(s) and ord(s[i]) > 127: i += 1
        sibem.add(s[:i].replace("️", ""))
    clash = [e for e in emo if e.replace("️", "") in sibem and which == "stage"]
    if clash: out.append(f"   emoji collides with existing sibling: {clash}"); ok = False
    longest = max(len(f"{t['pkgdir']}/{f}/🦀️.rs".encode("utf-8")) for f, _ in kids)
    out.append(f"children: {len(kids)}; longest child path {longest} bytes (limit 240)")
    modnames = [c["mod"] for c in spec["children"]] + ["tests"]
    coll = []
    for k, v in texts.items():
        for it in top_items(v):
            if it["kind"] in ("struct", "enum", "trait", "type", "union", "fn", "const", "static") and it["name"] in modnames:
                coll.append((k, it["kind"], it["name"]))
    out.append(f"module-name collisions: {coll}")
    if coll: ok = False
    for f, p in [("<root>", root)] + kids:
        ls = lines_of(read(p))
        act = sum(1 for l in ls if l.strip() and not l.strip().startswith("//"))
        out.append(f"   {f:<34} {len(ls):5} raw {act:5} active")
    out.append("RESULT " + ("OK" if ok else "FAIL"))
    return ok, "\n".join(out)


if __name__ == "__main__":
    names = list(TARGETS) if sys.argv[1] == "all" else sys.argv[1].split(",")
    which = sys.argv[2] if len(sys.argv) > 2 else "stage"
    allok = True
    for n in names:
        if n not in SPEC: continue
        ok, txt = verify(n, which)
        allok &= ok
        print(txt)
    print("ALL OK" if allok else "SOME FAILED")
