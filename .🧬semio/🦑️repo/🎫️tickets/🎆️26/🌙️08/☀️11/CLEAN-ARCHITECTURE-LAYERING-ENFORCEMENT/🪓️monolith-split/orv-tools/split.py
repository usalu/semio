"""Generates the split of one crate root into stage/: children, wiring root and visibility widening.
Writes only under rcr-work (sandbox); install.py copies the staged result into the real tree."""
import json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC
from snapshot import copy_files, snap_dir, closure_files
from reach import reach, scan, descendants

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
        if k in TOPKINDS and nm and nm != "_":
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



REGION_RX = re.compile(r"^\s*// ?#(end)?region")
PATHLIT_RX = re.compile(r'(#\[path\s*=\s*"|include!\("|include_str!\("|include_bytes!\(")(?!/)([^"]+)"')


def prefix_paths(text):
    return PATHLIT_RX.sub(lambda m: m.group(1) + "../" + m.group(2) + '"', text)


def interior_lines(src):
    s = set()
    for k, x, sl, el, idx in lex(src):
        if k in ("str", "comment") and el > sl:
            s.update(range(sl + 2, el + 2))
    return s


def check_boundaries(orig, claims):
    src = "\n".join(orig)
    toks = lex(src)
    nl = len(orig)
    nocut = [False] * (nl + 1)
    items = top_items(src)
    for it in items:
        hdr = it["hdr"]
        if not hdr: continue
        idx = None
        for q, t in enumerate(toks):
            if t[4] == hdr[0][4]:
                idx = q; break
        q = idx
        while q > 0 and toks[q - 1][0] != "comment" and not (toks[q - 1][1] in (";", "}", "{") and toks[q - 1][0] == "p"):
            q -= 1
        cur = toks[q][2]
        r = q - 1
        while r >= 0 and toks[r][0] == "comment" and toks[r][3] == cur - 1 and not REGION_RX.match(toks[r][1]):
            cur = toks[r][2]; r -= 1
        for ln in range(cur + 1, it["end"] + 1):
            nocut[ln] = True
    bad = []
    for ch, (a, b) in claims:
        if nocut[a - 1]: bad.append((ch, "cut before", a))
        if b < nl and nocut[b]: bad.append((ch, "cut after", b))
    return bad


def check_inline_body(orig, a, b):
    seg = orig[a - 1:b]
    body = "\n".join(seg)
    its = top_items(body)
    assert its, "inline body has no items"
    cover = set()
    for it in its:
        cover.update(range(it["first_line"], it["end"] + 1))
    for i, l in enumerate(seg):
        s = l.strip()
        if s and not s.startswith("//") and i not in cover:
            return False
    return True


def dedent_pairs(pairs, n, interior):
    out = []
    for l, ln in pairs:
        if ln not in interior and l.startswith(" " * n):
            l = l[n:]
        out.append((l, ln))
    return out


def src_path(name):
    return f"{F}/orv-tools/orig/{name}.rs.txt"


def build(name):
    t = TARGETS[name]
    spec = SPEC[name]
    orig = lines_of(read(src_path(name)))
    nl = len(orig)
    interior = interior_lines("\n".join(orig))
    owner = [None] * (nl + 2)
    claims = []
    for c in spec["children"]:
        for a, b in c["ranges"]:
            for i in range(a, b + 1):
                assert owner[i] is None, f"line {i} claimed by {owner[i]} and {c['folder']}"
                owner[i] = c["folder"]
            if c.get("inline_body"):
                assert check_inline_body(orig, a, b), f"{c['folder']}: inline body is not a complete item sequence"
            else:
                claims.append((c["folder"], (a, b)))
    bad = check_boundaries(orig, claims)
    assert not bad, f"{name}: cuts inside an item: {bad}"
    drop = set(spec.get("drop_lines", []))
    for d in drop:
        assert owner[d] is None, f"dropped line {d} is claimed"
    root_keep = [i for i in range(1, nl + 1) if owner[i] is None and i not in drop]
    children = []
    for c in spec["children"]:
        pairs = []
        for ri, (a, b) in enumerate(c["ranges"]):
            seg = [(orig[i - 1], i) for i in range(a, b + 1)]
            if ri:
                while pairs and pairs[-1][0].strip() == "": pairs.pop()
                while seg and seg[0][0].strip() == "": seg = seg[1:]
                pairs.append(("", 0))
            pairs.extend(seg)
        if c.get("dedent"):
            pairs = dedent_pairs(pairs, c["dedent"], interior)
        while pairs and pairs[0][0].strip() == "": pairs.pop(0)
        while pairs and pairs[-1][0].strip() == "": pairs.pop()
        body = [l for l, _ in pairs]
        text = "\n".join(body)
        if not c.get("no_prefix"):
            text = prefix_paths(text)
        for old, new in c["fixes"]:
            assert text.count(old) == 1, (name, c["folder"], old, text.count(old))
            text = text.replace(old, new)
        regs = region_names(body)
        if c.get("head_doc"):
            regline = c["head_doc"]
        elif regs:
            regline = f"//! Holds the {', '.join('`' + r + '`' for r in regs)} region{'s' if len(regs) != 1 else ''} of the module; the root re-exports its items."
        else:
            regline = "//! Holds items of the module; the root re-exports them."
        head = f"//! {c['emoji']} {c['title']}: {c['doc']}\n//!\n{regline}\n\n"
        children.append(dict(spec=c, body=head + text + "\n", head_len=len(head)))
    return orig, owner, root_keep, children, interior


def mount_lines(c, glob_kind):
    out = []
    if c["macro_use"]:
        out.append("#[macro_use]")
    out.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
    vis = c.get("vis", "")
    out.append(f'{vis + " " if vis else ""}mod {c["mod"]};')
    gk = glob_kind[c["folder"]]
    if c.get("glob", True):
        if gk == "pub": out.append(f'pub use {c["mod"]}::*;')
        elif gk == "priv": out.append(f'use {c["mod"]}::*;')
    return out


def make_root(name, orig, root_keep, children, glob_kind, interior):
    spec = SPEC[name]
    anchor = spec["anchor"]
    main = [ch for ch in children if ch["spec"].get("into") is None]
    groups = {}
    for ch in children:
        L = ch["spec"].get("into")
        if L is not None:
            groups.setdefault(L, []).append(ch)

    def block_of(chs, wrap):
        lines = []
        if wrap: lines += ["//#region 🔖️Children", ""]
        first = True
        for ch in chs:
            if not first: lines.append("")
            first = False
            lines += mount_lines(ch["spec"], glob_kind)
        if wrap: lines += ["", "//#endregion 🔖️Children"]
        return lines

    out = []
    done_main = False
    pending = sorted(groups)
    for i in root_keep:
        while pending and pending[0] <= i:
            L = pending.pop(0)
            tight = all(ch["spec"].get("tight") for ch in groups[L])
            if not tight: out.append(("", False))
            out += [(l, False) for l in block_of(groups[L], False)]
            if not tight: out.append(("", False))
        if not done_main and main and i > anchor:
            out.append(("", False))
            out += [(l, False) for l in block_of(main, True)]
            out.append(("", False))
            done_main = True
        out.append((orig[i - 1], i in interior))
    for L in pending:
        out.append(("", False))
        out += [(l, False) for l in block_of(groups[L], False)]
    if not done_main and main:
        out.append(("", False))
        out += [(l, False) for l in block_of(main, True)]
    res = []
    for l, inter in out:
        if l.strip() == "" and not inter and res and res[-1][0].strip() == "" and not res[-1][1]:
            continue
        res.append((l, inter))
    while res and res[-1][0].strip() == "": res.pop()
    return "\n".join(l for l, _ in res) + "\n"


def descendant_texts(name):
    t = TARGETS[name]
    orig_text = read(src_path(name))
    order, side, children, plain = reach(f"{ROOT}/{t['file']}", root_text=orig_text)
    return {p: read(p) for p in order[1:] if os.path.isfile(p)}, children


def own_descendants(name, child):
    t = TARGETS[name]
    vpath = f"{ROOT}/{t['pkgdir']}/{child['spec']['folder']}/🦀️.rs"
    m, inc, plain = scan(vpath, child["body"])
    out = set()
    for q in m + [i for i in inc if i.endswith(".rs")]:
        if os.path.isfile(q):
            order, side, ch, pl = reach(q)
            out.update(order)
    return out


def build_all(name):
    t = TARGETS[name]
    orig, owner, root_keep, children, interior = build(name)
    desc, dchildren = descendant_texts(name)
    glob_kind = {ch["spec"]["folder"]: "pub" for ch in children}
    root_text = make_root(name, orig, root_keep, children, glob_kind, interior)
    ids = {"<root>": file_ids(root_text)}
    for ch in children:
        ids[ch["spec"]["folder"]] = file_ids(ch["body"])
    dids = {p: file_ids(x) for p, x in desc.items()}
    report = []
    for ch in children:
        f = ch["spec"]["folder"]
        own = own_descendants(name, ch)
        others = [v for k, v in ids.items() if k != f] + [v for p, v in dids.items() if p not in own]
        edits = plan_widen(ch["body"], others)
        if os.environ.get("ORV_NOWIDEN") == "1" or ch["spec"].get("widen") is False:
            edits = []
        ch["edits"] = edits
        ch["text"] = apply_edits(ch["body"], edits)
        for e in edits:
            report.append((f,) + tuple(e[2]))
    provided = {}
    root_glob = False
    root_names = set()
    mod_names = {ch["spec"]["mod"] for ch in children}
    for it in top_items(root_text):
        if it["kind"] == "use":
            hd = it["hdr"]
            first = [tk[1] for tk in hd if tk[0] == "id" and tk[1] not in ("use", "pub")][:1]
            if first and first[0] in mod_names:
                continue
            for q, tk in enumerate(hd):
                nxt = hd[q + 1][1] if q + 1 < len(hd) else ""
                if tk[0] == "id" and tk[1] not in ("use", "pub", "crate", "self", "super", "as") and nxt in (",", "}", ";"):
                    root_names.add(tk[1])
                if tk[1] == "*" and tk[0] == "p":
                    root_glob = True
        elif it["kind"] in TOPKINDS and it["name"]:
            root_names.add(it["name"])
        elif it["kind"] == "mod":
            root_names.add(it["name"])
        elif it["kind"] not in ("impl",):
            root_names |= macro_defined(it)
    for ch in children:
        names = set()
        for it in top_items(ch["body"]):
            if it["kind"] in TOPKINDS and it["name"]:
                names.add(it["name"])
            elif it["kind"] == "mod":
                names.add(it["name"])
            elif it["kind"] not in ("impl", "use"):
                names |= macro_defined(it)
        provided[ch["spec"]["folder"]] = names
    for ch in children:
        f = ch["spec"]["folder"]
        prov = set(root_names)
        for g, ns in provided.items():
            if g != f:
                prov |= ns
        mine = file_ids(ch["body"])[0]
        ch["needs_super"] = ch["spec"].get("super_glob", True) and (root_glob or bool(mine & prov))
        uses = list(ch["spec"]["imports"]) + (["use super::*;"] if ch["needs_super"] else [])
        if uses:
            hl = ch["head_len"]
            ch["text"] = ch["text"][:hl] + "\n".join(uses) + "\n\n" + ch["text"][hl:]
    for ch in children:
        f = ch["spec"]["folder"]
        its = top_items(ch["text"])
        has_pub = any((i["vis"].startswith("pub") and not i["vis"].startswith("pub(super)")) for i in its if i["kind"] != "impl")
        has_super = any(i["vis"].startswith("pub(super)") for i in its)
        glob_kind[f] = "pub" if has_pub else ("priv" if has_super else "none")
    root_text = make_root(name, orig, root_keep, children, glob_kind, interior)
    return orig, children, root_text, report, glob_kind


def main():
    names = sys.argv[1].split(",") if sys.argv[1] != "all" else list(TARGETS)
    for name in names:
        t = TARGETS[name]
        orig, children, root_text, report, glob_kind = build_all(name)
        sd = stage_dir(name)
        sandbox_under(sd)
        if os.path.exists(sd): shutil.rmtree(sd)
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
        write(f"{F}/🗑️generated/orv-{name}-widen.tsv", "\n".join("\t".join(map(str, r)) for r in report) + "\n")


if __name__ == "__main__":
    main()
