"""Generates the split of one crate root into stage/: children, wiring root and visibility widening.
Writes only under rcr-work (sandbox); install.py copies the staged result into the real tree."""
import json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC
from snapshot import copy_tree, snap_dir

TOPKINDS = ("fn", "const", "static", "struct", "enum", "trait", "type", "union")
MEMBERKINDS = ("fn", "const", "type")
NOISE = {"self", "Self", "crate", "super"}


def stage_dir(name):
    return f"{W}/{name}/stage"


def sandbox_under(path):
    rp = os.path.realpath(path)
    assert rp.startswith(os.path.realpath(W) + os.sep), f"write outside sandbox: {rp}"


def seq_items(toks):
    depth = 0
    out = []
    t0 = 0
    for idx, t in enumerate(toks):
        k, x = t[0], t[1]
        if k == "p":
            if x in "{([":
                depth += 1
            elif x in "})]":
                depth -= 1
                if depth == 0 and x == "}":
                    nxt = toks[idx + 1][1] if idx + 1 < len(toks) else ""
                    if nxt != ";":
                        out.append((t0, idx)); t0 = idx + 1
            elif x == ";" and depth == 0:
                out.append((t0, idx)); t0 = idx + 1
    return [toks[a:b + 1] for a, b in out]


def strip_attrs(sub):
    j = 0
    while j < len(sub):
        k, x = sub[j][0], sub[j][1]
        if k == "comment":
            j += 1; continue
        if x == "#" and k == "p":
            d = 0; j += 1
            if sub[j][1] == "!": j += 1
            while True:
                if sub[j][1] == "[": d += 1
                elif sub[j][1] == "]":
                    d -= 1
                    if d == 0: break
                j += 1
            j += 1; continue
        break
    return sub[j:]


def sig_ids(toks):
    """identifiers of a signature: tokens up to the first brace or semicolon at depth 0"""
    out = set()
    depth = 0
    for t in toks:
        if t[0] == "p":
            if t[1] in "([": depth += 1
            elif t[1] in ")]": depth -= 1
            elif t[1] == "{" and depth == 0: break
            elif t[1] == ";" and depth == 0: break
        if t[0] == "id": out.add(t[1])
    return out


def split_fields(toks, start):
    open_tok = toks[start][1]
    depth = 0; angle = 0; i = start; segs = []; cur = []
    n = len(toks)
    while i < n:
        k, x = toks[i][0], toks[i][1]
        if k == "p":
            if x in "{([":
                depth += 1
                if depth == 1 and i == start:
                    i += 1; continue
            elif x in "})]":
                depth -= 1
                if depth == 0:
                    segs.append(cur); break
            elif x == "<" and depth == 1: angle += 1
            elif x == ">" and depth == 1 and not (i > 0 and toks[i - 1][1] == "-"): angle = max(0, angle - 1)
            elif x == "," and depth == 1 and angle == 0:
                segs.append(cur); cur = []; i += 1; continue
        cur.append(toks[i]); i += 1
    out = []
    for s in segs:
        s = [t for t in s if t[0] != "comment"]
        s = strip_attrs(s)
        if not s: continue
        vis = s[0][1] == "pub"
        jj = 1 if vis else 0
        if vis and jj < len(s) and s[jj][1] == "(":
            while s[jj][1] != ")": jj += 1
            jj += 1
        name = None
        if open_tok == "{" and jj < len(s) and s[jj][0] == "id": name = s[jj][1]
        out.append(dict(name=name, vis=vis, pos=s[0][4], ids={t[1] for t in s if t[0] == "id"}))
    return out


def macro_defined(it):
    """names an item-position macro invocation may define (identifier after an item keyword inside it) plus its own name"""
    out = set()
    hd = it["hdr"]
    for q, tk in enumerate(hd):
        if tk[0] == "id" and tk[1] in ("struct", "enum", "trait", "fn", "type", "const", "static", "mod", "union") and q + 1 < len(hd) and hd[q + 1][0] == "id":
            out.add(hd[q + 1][1])
    return out


def file_ids(text):
    toks = lex(text)
    allids = set(); member = set(); field = set()
    for i, t in enumerate(toks):
        if t[0] == "id":
            allids.add(t[1])
            if i and toks[i - 1][1] in (".", ":"): member.add(t[1])
            if i and toks[i - 1][1] == "." and not (i + 1 < len(toks) and toks[i + 1][1] == "(") and not (i + 2 < len(toks) and toks[i + 1][1] == ":" and toks[i + 2][1] == ":"): field.add(t[1])
    # format-string captures `{name}` and `{name:?}`
    for t in toks:
        if t[0] == "str":
            for m in re.finditer(r"\{([A-Za-z_][A-Za-z0-9_]*)(?::[^{}]*)?\}", t[1]):
                allids.add(m.group(1))
    return allids, member, field


def plan_widen(text, others):
    """others: list of (allids, memberids) of the other files. Returns edits [(pos, 'pub(super) ', label)]."""
    items = top_items(text)
    toks_all = lex(text)
    cand = []
    privnames = {}
    info = []
    for it in items:
        k, nm, vis = it["kind"], it["name"], it["vis"]
        if k in TOPKINDS and nm:
            if vis == "":
                privnames[nm] = it
    wid = {}

    def used_elsewhere_any(n):
        return any(n in o[0] for o in others)

    def used_member(n):
        return any(n in o[1] for o in others)

    def mark(pos, label, ids):
        wid[pos] = (label, ids)

    for it in items:
        k, nm, vis = it["kind"], it["name"], it["vis"]
        hdr = it["hdr"]
        if not hdr:
            continue
        if k in TOPKINDS and nm:
            if vis == "" and used_elsewhere_any(nm):
                ids = sig_ids(hdr) if k in ("fn", "trait") else {t[1] for t in hdr if t[0] == "id"}
                mark(hdr[0][4], ("item", k, nm), ids)
            if k == "struct":
                sk = [q for q, t in enumerate(hdr) if t[0] == "id" and t[1] == "struct"][0]
                idx = None
                for q in range(sk + 1, len(hdr)):
                    if hdr[q][0] == "p" and hdr[q][1] in "{(":
                        idx = q; break
                    if hdr[q][0] == "p" and hdr[q][1] == ";": break
                if idx is not None:
                    ot = hdr[idx][1]
                    for f in split_fields(hdr, idx):
                        if f["vis"]: continue
                        if ot == "{":
                            fname = f["name"]
                            if fname and (any(fname in o[2] for o in others) or any((nm in o[0] and fname in o[0]) for o in others)):
                                mark(f["pos"], ("field", nm, fname), f["ids"])
                        else:
                            if any(nm in o[0] for o in others):
                                mark(f["pos"], ("tuplefield", nm, ""), f["ids"])
        elif k == "impl":
            br = None
            for q, t in enumerate(hdr):
                if t[0] == "p" and t[1] == "{": br = q; break
            if br is None: continue
            head = hdr[:br]
            if any(t[0] == "id" and t[1] == "for" for t in head): continue
            depth = 0; endq = None
            for q in range(br, len(hdr)):
                t = hdr[q]
                if t[0] == "p" and t[1] in "{([": depth += 1
                elif t[0] == "p" and t[1] in "})]":
                    depth -= 1
                    if depth == 0: endq = q; break
            body = hdr[br + 1:endq]
            tname = " ".join(t[1] for t in head if t[0] != "comment")
            for sub in seq_items(body):
                h = strip_attrs(sub)
                if not h: continue
                mv, mk, mn = name_kind(h)
                if mk in MEMBERKINDS and mv == "" and mn and used_member(mn):
                    mark(h[0][4], ("member", tname, mn), sig_ids(h))
    # closure: private top-level types named in widened signatures
    changed = True
    widened_names = {lab[2] for lab, ids in wid.values() if lab[0] == "item"}
    while changed:
        changed = False
        for pos, (lab, ids) in list(wid.items()):
            for n in ids:
                if n in privnames and n not in widened_names and privnames[n]["kind"] in ("struct", "enum", "type", "trait", "union", "const", "static"):
                    it = privnames[n]
                    hdr = it["hdr"]
                    mark(hdr[0][4], ("item", it["kind"], n, "via-signature"), {t[1] for t in hdr if t[0] == "id"})
                    widened_names.add(n)
                    changed = True
    return [(pos, "pub(super) ", lab) for pos, (lab, ids) in sorted(wid.items())]


def apply_edits(text, edits):
    for pos, ins, _ in sorted(edits, key=lambda e: -e[0]):
        text = text[:pos] + ins + text[pos:]
    return text


def region_names(lines):
    out = []
    for l in lines:
        m = re.match(r"^\s*// ?#region (.*)$", l)
        if m: out.append(m.group(1))
    return out


def snake(s):
    return s.replace("-", "_")


def check_boundaries(orig, claims):
    src = "\n".join(orig)
    toks = lex(src)
    nl = len(orig)
    nocut = [False] * (nl + 1)
    items = top_items(src)
    for it in items:
        hdr = it["hdr"]
        if not hdr: continue
        first_real = hdr[0][2]
        # first attr/hdr token line = hdr_line minus attribute lines; recompute from tokens directly
        start = it["first_line"]
        # attached comments only (contiguous run before first non-comment token)
        sub_first = None
        idx = None
        for q, t in enumerate(toks):
            if t[4] == hdr[0][4]:
                idx = q; break
        # walk back over attribute tokens: find first token of the item that is not a comment
        q = idx
        while q > 0 and toks[q - 1][0] != "comment" and not (toks[q - 1][1] in (";", "}", "{") and toks[q - 1][0] == "p"):
            q -= 1
        cur = toks[q][2]
        r = q - 1
        while r >= 0 and toks[r][0] == "comment" and toks[r][3] == cur - 1:
            cur = toks[r][2]; r -= 1
        attached = cur
        for ln in range(attached + 1, it["end"] + 1):
            nocut[ln] = True
    bad = []
    for ch, (a, b) in claims:
        if nocut[a - 1]: bad.append((ch, "cut before", a))
        if b < nl and nocut[b]: bad.append((ch, "cut after", b))
    return bad


def build(name):
    t = TARGETS[name]
    spec = SPEC[name]
    orig = lines_of(read(f"{F}/rcr-tools/orig/{name}.rs.txt"))
    nl = len(orig)
    owner = [None] * (nl + 2)
    claims = []
    for c in spec["children"]:
        for a, b in c["ranges"]:
            for i in range(a, b + 1):
                assert owner[i] is None, f"line {i} claimed by {owner[i]} and {c['folder']}"
                owner[i] = c["folder"]
            claims.append((c["folder"], (a, b)))
    bad = check_boundaries(orig, claims)
    assert not bad, f"{name}: cuts inside an item: {bad}"
    # root lines
    drop = set(spec.get("drop_lines", []))
    root_keep = [i for i in range(1, nl + 1) if owner[i] is None and i not in drop]
    for i in root_keep:
        pass
    children = []
    for c in spec["children"]:
        body = []
        for ri, (a, b) in enumerate(c["ranges"]):
            seg = orig[a - 1:b]
            if ri:
                while body and body[-1].strip() == "": body.pop()
                while seg and seg[0].strip() == "": seg = seg[1:]
                body.append("")
            body.extend(seg)
        while body and body[0].strip() == "": body.pop(0)
        while body and body[-1].strip() == "": body.pop()
        text = "\n".join(body)
        for old, new in c["fixes"]:
            assert text.count(old) == 1, (name, c["folder"], old, text.count(old))
            text = text.replace(old, new)
        regs = region_names(body)
        regline = f"//! Holds the {', '.join('`' + r + '`' for r in regs)} region{'s' if len(regs) != 1 else ''} of the crate root; the root re-exports its items." if regs else "//! Holds items of the crate root; the root re-exports them."
        head = f"//! {c['emoji']} {c['title']}: {c['doc']}\n//!\n{regline}\n\n"
        children.append(dict(spec=c, body=head + text + "\n", head_len=len(head)))
    return orig, owner, root_keep, children


def make_root(name, orig, owner, root_keep, children, glob_kind):
    spec = SPEC[name]
    anchor = spec["anchor"]
    block = ["//#region 🔖️Children", ""]
    for ch in children:
        c = ch["spec"]
        if c["macro_use"]:
            block.append("#[macro_use]")
        block.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
        block.append(f"mod {c['mod']};")
        gk = glob_kind[c["folder"]]
        if gk == "pub":
            block.append(f"pub use {c['mod']}::*;")
        elif gk == "priv":
            block.append(f"use {c['mod']}::*;")
        block.append("")
    block.pop()
    block.append("")
    block.append("//#endregion 🔖️Children")
    out = []
    inserted = False
    for i in root_keep:
        if not inserted and i > anchor:
            out.append("")
            out.extend(block)
            out.append("")
            inserted = True
        out.append(orig[i - 1])
    if not inserted:
        out.append("")
        out.extend(block)
    # collapse blank runs
    res = []
    for l in out:
        if l.strip() == "" and res and res[-1].strip() == "":
            continue
        res.append(l)
    while res and res[-1].strip() == "": res.pop()
    return "\n".join(res) + "\n"


def build_all(name, dry=False):
    t = TARGETS[name]
    orig, owner, root_keep, children = build(name)
    tests_files = []
    for dp, dn, fn in os.walk(f"{ROOT}/{t['tree']}/🧪️tests"):
        for f in fn:
            if f.endswith(".rs"):
                tests_files.append(os.path.join(dp, f))
    # first pass: wiring-free root text for id sets
    glob_kind = {ch["spec"]["folder"]: "pub" for ch in children}
    root_text = make_root(name, orig, owner, root_keep, children, glob_kind)
    ids = {"<root>": file_ids(root_text)}
    for ch in children:
        ids[ch["spec"]["folder"]] = file_ids(ch["body"])
    # test files that are children of the root (only unit test mounted via #[path]) count as other files
    for p in tests_files:
        rel = os.path.relpath(p, f"{ROOT}/{t['tree']}")
        if rel.endswith("🔬️unit/🦀️.rs"):
            ids["<tests>"] = file_ids(read(p))
    report = []
    for ch in children:
        f = ch["spec"]["folder"]
        others = [v for k, v in ids.items() if k != f]
        # strip the header docstring/`use super::*` from edits by planning on the whole text (positions are absolute)
        edits = plan_widen(ch["body"], others)
        if os.environ.get("RCR_NOWIDEN") == "1":
            edits = []
        ch["edits"] = edits
        ch["text"] = apply_edits(ch["body"], edits)
        for e in edits:
            report.append((f,) + tuple(e[2]))
    provided = {}
    root_toks_items = top_items(root_text)
    root_glob = False
    root_names = set()
    for it in root_toks_items:
        if it["kind"] == "use":
            hd = it["hdr"]
            first = [tk[1] for tk in hd if tk[0] == "id" and tk[1] not in ("use", "pub")][:1]
            if first and first[0] in {ch["spec"]["mod"] for ch in children}:
                continue
            for q, tk in enumerate(hd):
                nxt = hd[q + 1][1] if q + 1 < len(hd) else ""
                if tk[0] == "id" and tk[1] not in ("use", "pub", "crate", "self", "super", "as") and nxt in (",", "}", ";"):
                    root_names.add(tk[1])
                if tk[1] == "*" and tk[0] == "p":
                    root_glob = True
        elif it["kind"] in TOPKINDS and it["name"]:
            root_names.add(it["name"])
        elif it["kind"] not in ("impl", "mod"):
            root_names |= macro_defined(it)
    for ch in children:
        names = set()
        for it in top_items(ch["body"]):
            if it["kind"] in TOPKINDS and it["name"]:
                names.add(it["name"])
            elif it["kind"] not in ("impl", "mod", "use"):
                names |= macro_defined(it)
        provided[ch["spec"]["folder"]] = names
    for ch in children:
        f = ch["spec"]["folder"]
        prov = set(root_names)
        for g, ns in provided.items():
            if g != f:
                prov |= ns
        mine = file_ids(ch["body"])[0]
        ch["needs_super"] = root_glob or bool(mine & prov)
        uses = list(ch["spec"]["imports"]) + (["use super::*;"] if ch["needs_super"] else [])
        if uses:
            hl = ch["head_len"]
            ch["text"] = ch["text"][:hl] + "\n".join(uses) + "\n\n" + ch["text"][hl:]
    # glob kinds
    for ch in children:
        f = ch["spec"]["folder"]
        its = top_items(ch["text"])
        has_pub = any(i["vis"].startswith("pub") and not i["vis"].startswith("pub(super)") and i["kind"] not in ("use",) or (i["kind"] == "use" and i["vis"].startswith("pub")) for i in its)
        has_super = any(i["vis"].startswith("pub(super)") for i in its)
        # members/fields widened do not export names
        glob_kind[f] = "pub" if has_pub else ("priv" if has_super else "none")
    root_text = make_root(name, orig, owner, root_keep, children, glob_kind)
    return orig, children, root_text, report, glob_kind


def main():
    names = sys.argv[1].split(",") if sys.argv[1] != "all" else list(TARGETS)
    for name in names:
        t = TARGETS[name]
        orig, children, root_text, report, glob_kind = build_all(name)
        sd = stage_dir(name)
        sandbox_under(sd)
        if os.path.exists(sd): shutil.rmtree(sd)
        copy_tree(t["tree"], sd)
        pk = f"{sd}/{t['pkgdir']}"
        write(f"{pk}/🦀️.rs", root_text)
        for ch in children:
            write(f"{pk}/{ch['spec']['folder']}/🦀️.rs", ch["text"])
        rep = [f"{name}: root {len(lines_of(root_text))} lines"]
        for ch in children:
            ls = lines_of(ch["text"])
            act = sum(1 for l in ls if l.strip() and not l.strip().startswith("//"))
            rep.append(f"  {ch['spec']['folder']:<34} {len(ls):5} lines {act:5} active  glob={glob_kind[ch['spec']['folder']]}  widened={len(ch['edits'])}")
        print("\n".join(rep))
        write(f"{F}/🗑️generated/rcr-{name}-widen.tsv", "\n".join("\t".join(map(str, r)) for r in report) + "\n")


if __name__ == "__main__":
    main()
