"""✂️ c4p23 splitter: stages (never installs) the children of `plans.PLANS` from the pre-edit blobs of `BASE_REV`.
mod mode: each range becomes `<folder>/🦀️.rs` (`//!` doc, `use super::*;`, the lines verbatim; private definitions another file names become `pub(super)`),
the root gets `#[path] mod x; <vis> use x::*;` at the position of the range.
include mode: each range becomes a fragment spliced back with `include!` (module identity, test paths and `use` scope unchanged; no widening).
usage: c4psplit.py stage [key...]   |   c4psplit.py report"""
import json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER
import splitter as S
from rslex import lex, outline, multiline_literal_lines

DOCABLE = {"fn", "struct", "enum", "trait", "impl", "mod", "const", "static", "type", "union"}


class Widener(S.Plan):
    def __init__(self):
        pass


WID = Widener()


def block_comment_edits(body, ext_all, ext_fields, ext_methods):
    """The frs outline reads a `/** .. */`-led item as one `other` item; widen the definition behind the comment the same way as a plain item."""
    from rslex import ITEM_HEAD
    text = "\n".join(body) + "\n"
    depths, inlit = lex(text)
    edits = {}
    for it in outline(text):
        if it["kind"] != "other" or not body[it["start"]].lstrip().startswith("/*"):
            continue
        h = next((k for k in range(it["start"], it["end"] + 1) if not inlit[k] and depths[k] == 0 and body[k].strip() and not body[k].lstrip().startswith(("/*", "*", "//", "#["))), None)
        if h is None:
            continue
        m = ITEM_HEAD.match(body[h])
        if not m:
            continue
        kind, vis = m.group("kw"), (m.group("vis") or "").strip()
        sub = dict(start=h, end=it["end"], kind=kind, vis=vis, name=(S.IDENT.search(m.group("rest")).group(0) if S.IDENT.search(m.group("rest")) else None), head=body[h].strip()[:110])
        if kind in ("fn", "struct", "enum", "trait", "const", "static", "type", "union") and not vis and sub["name"] in ext_all:
            mm = re.match(r"^(\s*)(.*)$", body[h])
            edits[h] = (mm.group(1) + "pub(super) " + mm.group(2), f"{kind} {sub['name']}")
        if kind == "struct":
            WID._widen_struct(body, sub, h, depths, ext_all, ext_fields, edits)
        if kind == "impl":
            WID._widen_impl(body, sub, h, depths, ext_methods, edits)
    return [(h, nl, label) for h, (nl, label) in sorted(edits.items())]


USE_HEAD = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?use\b")


def hoist_uses(body):
    """Top-level `use` statements (with their attribute and doc lines directly above) leave a child body: a private import is invisible outside its module, and the root's imports reach every child through `use super::*;`."""
    text = "\n".join(body) + "\n"
    drop = set()
    local_mods = {it["name"] for it in outline(text) if it["kind"] == "mod" and it["name"]}
    for it in outline(text):
        if it["kind"] != "use":
            continue
        s = None
        for k in range(it["end"], it["start"] - 1, -1):
            if USE_HEAD.match(body[k]):
                s = k
                break
        if s is None:
            continue
        head = re.match(r"^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?(\w+)", body[s])
        if head and head.group(1) in local_mods:
            continue
        a = s
        while a - 1 >= it["start"] and (body[a - 1].lstrip().startswith("#[") or body[a - 1].lstrip().startswith("///")):
            a -= 1
        drop.update(range(a, it["end"] + 1))
    keep = [l for i, l in enumerate(body) if i not in drop]
    gone = [l for i, l in enumerate(body) if i in drop]
    return keep, gone


def lib_files(lib_rel):
    seen, order = set(), []

    def visit(p):
        p = os.path.normpath(p)
        if p in seen or not os.path.isfile(p):
            return
        seen.add(p)
        order.append(p)
        txt = S.mask_keep_strings(read(p))
        for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', txt):
            visit(os.path.join(os.path.dirname(p), m.group(1)))
        for m in re.finditer(r'\binclude!\(\s*"([^"]+)"\s*\)', txt):
            visit(os.path.join(os.path.dirname(p), m.group(1)))
    visit(ROOT + "/" + lib_rel)
    return order


def top_vis(body_text):
    best = ""
    for it in outline(body_text):
        if it["kind"] in ("use", "impl", "other") or not it["name"]:
            continue
        v = it["vis"]
        if v == "pub":
            return "pub "
        if v.startswith("pub(crate)"):
            best = "pub(crate) "
    return best


def build(key):
    spec = PLANS[key]
    rel = spec["rel"]
    src = base_text(rel)
    ls = lines_of(src)
    mll = multiline_literal_lines(src)
    depths, inlit = lex(src)
    mode = spec["mode"]
    rngs = []
    for c in spec["children"]:
        for a, b in c["ranges"]:
            assert 1 <= a <= b <= len(ls), (key, c["folder"], a, b)
            assert depths[a - 1] == 0 and not inlit[a - 1], (key, c["folder"], a, "range does not start at depth 0")
            nxt_depth = depths[b] if b < len(depths) else 0
            nxt_lit = inlit[b] if b < len(inlit) else False
            assert nxt_depth == 0 and not nxt_lit, (key, c["folder"], b, "range does not end at depth 0")
            rngs.append((a, b, c["folder"]))
    rngs.sort()
    for (a1, b1, f1), (a2, b2, f2) in zip(rngs, rngs[1:]):
        assert b1 < a2, (key, f1, f2, "ranges overlap")
    owner = [None] * (len(ls) + 2)
    for a, b, f in rngs:
        for i in range(a, b + 1):
            owner[i] = f
    root_lines = [ls[i - 1] for i in range(1, len(ls) + 1) if owner[i] is None]
    remainder = "\n".join(root_lines)
    bodies = {}
    for c in spec["children"]:
        body = []
        for a, b in c["ranges"]:
            body.extend(ls[a - 1:b])
        bodies[c["folder"]] = body
    hoisted = {}
    if mode == "mod":
        for f in list(bodies):
            bodies[f], hoisted[f] = hoist_uses(bodies[f])
    out = {}
    report = dict(children={})
    base_dir = os.path.dirname(rel)
    if mode == "mod":
        mine = {os.path.normpath(ROOT + "/" + rel)} | {os.path.normpath(f"{ROOT}/{base_dir}/{c['folder']}/🦀️.rs") for c in spec["children"]}
        others = [p for p in lib_files(spec["lib"]) if os.path.normpath(p) not in mine]
        ext_all0, ext_fields0, ext_methods0 = set(), set(), set()
        for p in others:
            f_, a_ = S.tokenize(read(p))
            ext_all0 |= a_
            fl, me = S.member_refs(read(p))
            ext_fields0 |= fl
            ext_methods0 |= me
        rem_first, rem_all = S.tokenize(remainder)
        rem_fields, rem_methods = S.member_refs(remainder)
        toks = {f: S.tokenize("\n".join(b)) for f, b in bodies.items()}
        mems = {f: S.member_refs("\n".join(b)) for f, b in bodies.items()}
        for c in spec["children"]:
            f = c["folder"]
            body = list(bodies[f])
            ext_all = set(ext_all0) | rem_all
            ext_fields = set(ext_fields0) | rem_fields
            ext_methods = set(ext_methods0) | rem_methods
            for g in toks:
                if g != f:
                    ext_all |= toks[g][1]
                    ext_fields |= mems[g][0]
                    ext_methods |= mems[g][1]
            edits = WID.widen_edits(body, ext_all, ext_fields, ext_methods)
            have = {e[0] for e in edits}
            edits = edits + [e for e in block_comment_edits(body, ext_all, ext_fields, ext_methods) if e[0] not in have]
            _d, body_lit = lex("\n".join(body) + "\n")
            wid = []
            for ln, newline, label in edits:
                assert not body_lit[ln], (key, f, label, "widened line lies inside a multi-line literal or block comment")
                body[ln] = newline
                wid.append(label)
            while body and body[0].strip() == "":
                body.pop(0)
            while body and body[-1].strip() == "":
                body.pop()
            glob = "" if c.get("noglob") else "use super::*;\n\n"
            text = f"//! {c['doc']}\n\n{glob}" + "\n".join(body) + "\n"
            out[f"{base_dir}/{f}/🦀️.rs"] = text
            c["_vis"] = top_vis("\n".join(body) + "\n")
            named = [it for it in outline("\n".join(body) + "\n") if it["name"] and it["kind"] not in ("use", "impl", "other")]
            exported = [it["name"] for it in named if it["vis"].startswith("pub") or it["name"] in ext_all]
            if not exported:
                c["_vis"] = None
            report["children"][f] = dict(lines=len(lines_of(text)), widened=wid, vis=("none" if c["_vis"] is None else (c["_vis"].strip() or "private")))
    else:
        for c in spec["children"]:
            f = c["folder"]
            body = list(bodies[f])
            for i, l in enumerate(body):
                if l.strip() == '#[path = "."]' or re.search(r'#\[\s*path\s*=', l):
                    raise SystemExit(f"{key}: #[path] in a fragment needs a re-base: {l.strip()[:80]}")
                body[i] = re.sub(r'(\binclude(?:_str|_bytes)?!\(\s*")([^"/][^"]*")', lambda m: m.group(1) + "../" + m.group(2), l)
            while body and body[0].strip() == "":
                body.pop(0)
            while body and body[-1].strip() == "":
                body.pop()
            last = next((l for l in reversed(body) if l.strip()), "")
            assert not last.strip().startswith("///") and not last.strip().startswith("#["), (f, "fragment ends with an orphan doc or attribute")
            its = outline("\n".join(body) + "\n")
            first = its[0] if its else None
            marker = "///" if first and first["kind"] in DOCABLE else "//"
            text = f"{marker} {c['doc']}\n\n" + "\n".join(body) + "\n"
            out[f"{base_dir}/{f}/🦀️.rs"] = text
            report["children"][f] = dict(lines=len(lines_of(text)), first=(first["kind"], first["name"]) if first else None, marker=marker)
    root = []
    starts = {c["ranges"][0][0]: c for c in spec["children"]}
    in_range = set()
    for c in spec["children"]:
        for a, b in c["ranges"]:
            in_range.update(range(a, b + 1))
    for i in range(1, len(ls) + 1):
        if i in starts:
            c = starts[i]
            if root and root[-1].strip():
                root.append("")
            if mode == "mod":
                if hoisted.get(c["folder"]):
                    root.extend(hoisted[c["folder"]])
                    root.append("")
                root.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
                root.append(f"mod {c['mod']};")
                if c["_vis"] is not None:
                    root.append(f"{c['_vis']}use {c['mod']}::*;")
            else:
                root.append(f'include!("{c["folder"]}/🦀️.rs");')
            root.append("")
        if i in in_range:
            continue
        root.append(ls[i - 1])
    tidy = []
    for l in root:
        if l.strip() == "" and tidy and tidy[-1].strip() == "":
            continue
        tidy.append(l)
    while tidy and tidy[-1].strip() == "":
        tidy.pop()
    root_text = "\n".join(tidy) + "\n"
    for a, b in spec.get("subs", []):
        assert root_text.count(a) == 1, (key, "sub anchor not unique", a[:70], root_text.count(a))
        root_text = root_text.replace(a, b)
    out[rel] = root_text
    report["root_lines"] = len(tidy)
    report["orig_lines"] = len(ls)
    return out, report, ls


def stage(keys):
    allrep = {}
    for key in keys:
        out, report, ls = build(key)
        sd = f"{STAGE}/{key}"
        if os.path.exists(sd):
            shutil.rmtree(sd)
        for rel, txt in out.items():
            write(f"{sd}/{rel}", txt)
        new = []
        for txt in out.values():
            new.extend(lines_of(txt))
        gone, added = diff_multiset(ls, new)
        report["removed"] = sum(gone.values())
        report["added"] = sum(added.values())
        report["files"] = sorted(out)
        allrep[key] = report
        kids = {k: v['lines'] for k, v in report['children'].items()}
        print(f"[{key}] orig={report['orig_lines']} root={report['root_lines']} children={kids} removed={report['removed']} added={report['added']}")
        for f, r in report["children"].items():
            if r.get("widened"):
                print(f"    {f}: widened {len(r['widened'])}")
    os.makedirs(WORK, exist_ok=True)
    json.dump(allrep, open(f"{WORK}/stage-report.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    return allrep


if __name__ == "__main__":
    a = sys.argv[1:]
    cmd = a[0] if a else "stage"
    keys = a[1:] or ORDER
    if cmd == "stage":
        stage(keys)
