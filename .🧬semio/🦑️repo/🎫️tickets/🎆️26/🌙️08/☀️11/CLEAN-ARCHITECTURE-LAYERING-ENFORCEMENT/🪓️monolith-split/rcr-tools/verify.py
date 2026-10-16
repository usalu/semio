"""Static verification of the staged (or installed) split: losslessness, item set, paths, emoji, mod-name collisions, definition scan.
usage: verify.py <name|all> [stage|real]"""
import collections, json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC
import split

MARK = re.compile(r"^\s*// ?#(end)?region ")


def norm_line(l):
    s = l.strip()
    s = s.replace("pub(super) ", "")
    s = s.replace('include_str!("../../../', 'include_str!("../../')
    return s


def item_texts(text, skip=()):
    lines = text.split("\n")
    out = []
    for it in top_items(text):
        seg = [norm_line(l) for l in lines[it["first_line"]:it["end"] + 1] if norm_line(l) not in skip and not l.strip().startswith("#![")]
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


def verify(name, which):
    t = TARGETS[name]
    base, root, kids = files_of(name, which)
    ok = True
    out = [f"== {name} ({which})"]
    orig = read(f"{F}/rcr-tools/orig/{name}.rs.txt")
    ol = lines_of(orig)
    assert "pub(super)" not in orig
    rt = read(root)
    texts = {"<root>": rt}
    for f, p in kids:
        texts[f] = read(p)
    newl = []
    for k, v in texts.items():
        newl += lines_of(v)
    mo = collections.Counter(norm_line(l) for l in ol if l.strip())
    mn = collections.Counter(norm_line(l) for l in newl if l.strip())
    lost = mo - mn
    extra = mn - mo
    allowed = collections.Counter(norm_line(ol[i - 1]) for i in SPEC[name].get("drop_lines", []))
    dropped_ok = lost & allowed
    lost = lost - allowed
    if dropped_ok:
        out.append(f"import lines moved from the root into the children that use them: {dict(dropped_ok)}")
    out.append(f"lines: original {len(ol)} raw / {sum(mo.values())} non-blank; new {len(newl)} raw / {sum(mn.values())} non-blank")
    out.append(f"lost non-blank lines: {sum(lost.values())}")
    for l, n in lost.most_common(10): out.append(f"   LOST {n}x {l[:100]}")
    out.append(f"extra non-blank lines: {sum(extra.values())} (children docs/uses/mounts)")
    if lost: ok = False
    # item multiset
    dropped = {norm_line(ol[i - 1]) for i in SPEC[name].get("drop_lines", [])}
    io = collections.Counter(item_texts(orig, dropped))
    inew = collections.Counter()
    for k, v in texts.items():
        inew.update(item_texts(v))
    ilost = io - inew
    iextra = inew - io
    out.append(f"top-level items: original {sum(io.values())}, new {sum(inew.values())}; lost {sum(ilost.values())}; extra {sum(iextra.values())}")
    for l, n in ilost.most_common(5): out.append(f"   ITEM-LOST {n}x {l[:120]!r}")
    for l, n in iextra.most_common(8): out.append(f"   ITEM-EXTRA {n}x {l[:120]!r}")
    if ilost: ok = False
    # region markers
    def regs(ls): return collections.Counter(l.strip() for l in ls if MARK.match(l))
    ro = regs(ol); rn = regs(newl)
    d = rn - ro
    dl = ro - rn
    out.append(f"region markers: original {sum(ro.values())} new {sum(rn.values())}; lost {dict(dl)}; added {dict(d)}")
    if dl: ok = False
    # path check: every #[path] and include_str/include!
    checked = 0; missing = []
    for k, v in texts.items():
        p = root if k == "<root>" else dict(kids)[k]
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]|include_str!\("([^"]+)"\)|include!\("([^"]+)"\)|include_bytes!\("([^"]+)"\)', v):
            lit = next(g for g in m.groups() if g)
            target = os.path.normpath(os.path.join(os.path.dirname(p), lit))
            if which == "stage":
                # staged tree mirrors the real one only for the module tree; side files exist there too
                pass
            checked += 1
            if not os.path.isfile(target): missing.append((k, lit))
    out.append(f"path/include checks: {checked}, missing: {len(missing)} {missing[:5]}")
    if missing: ok = False
    # children files and folders
    names = []
    for f, p in kids:
        names.append(f)
        d = f
        if not (len(d) > 1 and d[1:2] == "️" or (len(d) > 2 and d[2:3] == "️")):
            out.append(f"   FOLDER-EMOJI-NO-VS16 {d}")
            ok = False
        first = texts[f].split("\n")[0]
        em = SPEC[name]["children"][[c['folder'] for c in SPEC[name]['children']].index(f)]["emoji"]
        if not first.startswith(f"//! {em} "):
            out.append(f"   DOCSTRING-EMOJI-MISMATCH {f}: {first[:40]}")
            ok = False
        if len(f"{t['pkgdir']}/{f}/🦀️.rs".encode("utf-8")) > 240:
            out.append(f"   PATH-TOO-LONG {f}"); ok = False
    emo = [c["emoji"] for c in SPEC[name]["children"]]
    if len(set(emo)) != len(emo): out.append("   DUPLICATE sibling emoji"); ok = False
    sib = {"📋️", "📜️", "🦀️", "📋", "📜", "🦀"}
    if any(e in sib or e.replace("️", "") in {"📋", "📜", "🦀"} for e in emo): out.append("   emoji collides with sibling file"); ok = False
    longest = max(len(f"{t['pkgdir']}/{f}/🦀️.rs".encode("utf-8")) for f, _ in kids)
    out.append(f"children: {len(kids)}; longest child path {longest} bytes (limit 240)")
    # mod-name collisions
    modnames = [c["mod"] for c in SPEC[name]["children"]] + ["tests"]
    coll = []
    allnames = collections.Counter()
    for k, v in texts.items():
        for it in top_items(v):
            if it["kind"] in ("struct", "enum", "trait", "type", "union", "mod", "fn", "const", "static") and it["name"] in modnames:
                coll.append((k, it["kind"], it["name"]))
            if it["kind"] == "use":
                ids = [x[1] for x in it["hdr"] if x[0] == "id"]
                for m in modnames:
                    if m in ids and not (it["hdr"] and it["vis"] and False):
                        coll.append((k, "use", m))
    coll = [c for c in coll if not (c[1] == "mod")]
    # the mount lines themselves are `mod`/`use` items of the root; filter those
    coll = [c for c in coll if not (c[0] == "<root>" and c[1] == "use")]
    out.append(f"module-name collisions: {coll}")
    if coll: ok = False
    # sizes
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
        ok, txt = verify(n, which)
        allok &= ok
        print(txt)
    print("ALL OK" if allok else "SOME FAILED")
