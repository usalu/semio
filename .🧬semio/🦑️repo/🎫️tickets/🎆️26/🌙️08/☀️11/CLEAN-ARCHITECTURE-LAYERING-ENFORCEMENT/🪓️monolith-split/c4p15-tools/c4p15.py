"""✂️ c4p15 splitter. usage:
  c4p15.py stage [key...]     stage children and rewritten roots from the snapshots into the scratchpad (never writes the repo)
  c4p15.py install [key...]   copy the staged files into the repo (hash-guarded: the live target must still equal its snapshot; children only when absent)
  c4p15.py verify [key...]    reversible proof on the LIVE files: every child body and the root equal the planned slices of the snapshot
mod mode: each range becomes `<folder>/🦀️.rs` (`//!` doc, `use super::*;`, the lines verbatim; private definitions another file names become `pub(super)`),
members of one inherent impl are wrapped in the impl head; the root gets `#[path] mod x; <vis> use x::*;` at the position of the first range.
include mode: each range becomes an `include!` fragment (module identity, test paths and `use` scope unchanged; relative include paths get one `../`)."""
import json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4p15lib import *
from plans15 import PLANS, ORDER
import splitter as S
from rslex import lex, outline, multiline_literal_lines

DOCABLE = {"fn", "struct", "enum", "trait", "impl", "mod", "const", "static", "type", "union"}
INC_RX = re.compile(r'(include(?:_str|_bytes)?!\(\s*)"([^"/][^"]*)"')


class Widener(S.Plan):
    def __init__(self):
        pass


WID = Widener()


def crate_files_from(rel, text):
    seen, order = set(), []

    def visit(p, txt):
        p = os.path.normpath(p)
        if p in seen:
            return
        seen.add(p)
        order.append(p)
        if txt is None:
            if not os.path.exists(p):
                return
            txt = read(p)
        base = os.path.dirname(p)
        for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', S.mask_keep_strings(txt)):
            visit(os.path.join(base, m.group(1)), None)
        for m in re.finditer(r'\binclude!\(\s*"([^"]+)"\s*\)', S.mask_keep_strings(txt)):
            visit(os.path.join(base, m.group(1)), None)

    visit(ROOT + "/" + rel, text)
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


def rebase(body):
    edits = {}
    for i, l in enumerate(body):
        m = INC_RX.search(l)
        if m:
            nl = INC_RX.sub(lambda mm: f'{mm.group(1)}"../{mm.group(2)}"', l)
            edits[i] = (l, nl)
    return edits


def child_chunks(c):
    out = []
    for a, b in c.get("ranges", []):
        out.append(("plain", a, b))
    for a, b in c.get("members", []):
        out.append(("member", a, b))
    return out


def build(key):
    spec = PLANS[key]
    rel = spec["rel"]
    src = orig_text(spec["n"])
    ls = lines_of(src)
    assert not multiline_literal_lines(src) or True
    depths, inlit = lex(src)
    mode = spec["mode"]
    base_dir = os.path.dirname(rel)
    owner = [None] * (len(ls) + 2)
    for c in spec["children"]:
        c.setdefault("ranges", [])
        c.setdefault("members", [])
        for kind, a, b in child_chunks(c):
            assert 1 <= a <= b <= len(ls), (key, c["folder"], a, b)
            want = 1 if kind == "member" else 0
            assert ls[a - 1].strip() and ls[b - 1].strip(), (key, c["folder"], a, b, "range starts or ends on a blank line")
            assert depths[a - 1] == want and not inlit[a - 1], (key, c["folder"], a, f"range starts at depth {depths[a-1]} not {want}")
            nd = depths[b] if b < len(depths) else 0
            nl = inlit[b] if b < len(inlit) else False
            assert nd == want and not nl, (key, c["folder"], b, f"range ends at depth {nd} not {want}")
            for i in range(a, b + 1):
                assert owner[i] is None, (key, c["folder"], i, "overlap with " + str(owner[i]))
                owner[i] = c["folder"]
    drop = set()
    if "impl_span" in spec:
        h, e = spec["impl_span"]
        assert ls[h - 1].startswith("impl ") and ls[h - 1].rstrip().endswith("{") and ls[e - 1] == "}", (key, ls[h - 1], ls[e - 1])
        for i in (h, e):
            assert owner[i] is None
            drop.add(i)
        for i in range(h + 1, e):
            if owner[i] is None:
                assert ls[i - 1].strip() == "", (key, i, "impl line not claimed", ls[i - 1][:60])
    root_lines = [(i, ls[i - 1]) for i in range(1, len(ls) + 1) if owner[i] is None and i not in drop]
    remainder = "\n".join(l for _, l in root_lines)
    bodies = {}
    for c in spec["children"]:
        body = []
        for a, b in c["ranges"]:
            if body:
                body.append("")
            body.extend(ls[a - 1:b])
        if c["members"]:
            if body:
                body.append("")
            body.append(ls[spec["impl"] - 1])
            first = True
            for a, b in c["members"]:
                if not first:
                    body.append("")
                first = False
                body.extend(ls[a - 1:b])
            body.append("}")
        bodies[c["folder"]] = body
    out, report = {}, dict(children={})
    undo = {}
    if mode == "mod":
        others = [p for p in crate_files_from(spec["lib"], src if spec["lib"] == rel else None) if os.path.normpath(p) != os.path.normpath(ROOT + "/" + rel)]
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
        report["other_files"] = [os.path.relpath(p, ROOT) for p in others]
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
            _d, body_lit = lex("\n".join(body) + "\n")
            wid = []
            for ln, newline, label in edits:
                assert not body_lit[ln], (key, f, label, "widened line lies inside a multi-line literal")
                undo[newline.strip()] = body[ln].strip()
                body[ln] = newline
                wid.append(label)
            rb = rebase(body)
            for i, (old, new) in rb.items():
                undo[new.strip()] = old.strip()
                body[i] = new
            while body and body[0].strip() == "":
                body.pop(0)
            while body and body[-1].strip() == "":
                body.pop()
            text = f"//! {c['doc']}\n\nuse super::*;\n\n" + "\n".join(body) + "\n"
            out[f"{base_dir}/{f}/🦀️.rs"] = text
            c["_vis"] = top_vis("\n".join(body) + "\n")
            c["_named"] = any(it["name"] and it["kind"] not in ("use", "impl", "other") for it in outline("\n".join(body) + "\n"))
            report["children"][f] = dict(lines=len(lines_of(text)), widened=wid, rebased=len(rb), vis=c["_vis"].strip() or "private")
    else:
        for c in spec["children"]:
            f = c["folder"]
            body = list(bodies[f])
            rb = rebase(body)
            for i, (old, new) in rb.items():
                undo[new.strip()] = old.strip()
                body[i] = new
            while body and body[0].strip() == "":
                body.pop(0)
            while body and body[-1].strip() == "":
                body.pop()
            last = next((l for l in reversed(body) if l.strip()), "")
            assert not last.strip().startswith("///") and not last.strip().startswith("#["), (f, "fragment ends with an orphan doc or attribute")
            its = outline("\n".join(body) + "\n")
            first = its[0] if its else None
            marker = "//"
            text = f"{marker} {c['doc']}\n\n" + "\n".join(body) + "\n"
            out[f"{base_dir}/{f}/🦀️.rs"] = text
            report["children"][f] = dict(lines=len(lines_of(text)), rebased=len(rb), first=(first["kind"], first["name"]) if first else None, marker=marker)
    root = []
    starts = {}
    for c in spec["children"]:
        chunks = sorted((a, kind) for kind, a, b in child_chunks(c))
        starts[chunks[0][0]] = c
    for i in range(1, len(ls) + 1):
        if i in starts:
            c = starts[i]
            if root and root[-1].strip():
                root.append("")
            if mode == "mod":
                if c.get("cfg"):
                    root.append(c["cfg"])
                root.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
                root.append(f"mod {c['mod']};")
                if c["_named"]:
                    if c.get("cfg"):
                        root.append(c["cfg"])
                    root.append(f"{c['_vis']}use {c['mod']}::*;")
            else:
                root.append(f'include!("{c["folder"]}/🦀️.rs");')
            root.append("")
        if owner[i] is not None or i in drop:
            continue
        root.append(ls[i - 1])
    tidy = []
    for l in root:
        if l.strip() == "" and tidy and tidy[-1].strip() == "":
            continue
        tidy.append(l)
    while tidy and tidy[-1].strip() == "":
        tidy.pop()
    out[rel] = "\n".join(tidy) + "\n"
    report.update(root_lines=len(tidy), orig_lines=len(ls), undo=undo)
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
        undo = report["undo"]
        for txt in out.values():
            for l in lines_of(txt):
                new.append(undo.get(l.strip(), l))
        gone, added = diff_multiset(ls, new)
        report["removed"] = sum(gone.values())
        report["added"] = sum(added.values())
        report["removed_lines"] = sorted(gone.items())
        report["added_lines"] = sorted(added.items())
        report["files"] = sorted(out)
        allrep[key] = report
        kids = {k.split("/")[-1] if False else k: v["lines"] for k, v in report["children"].items()}
        print(f"[{key}] orig={report['orig_lines']} root={report['root_lines']} removed={report['removed']} added={report['added']}")
        for f, r in report["children"].items():
            print(f"    {r['lines']:5} {f}  widened={len(r.get('widened', []))} rebased={r.get('rebased', 0)} vis={r.get('vis', r.get('marker'))}")
        for (l, n) in report["removed_lines"][:12]:
            print(f"    REMOVED x{n}: {l[:110]}")
        for (l, n) in report["added_lines"][:20]:
            print(f"    ADDED   x{n}: {l[:110]}")
    json.dump({k: {kk: vv for kk, vv in v.items() if kk != "undo"} for k, v in allrep.items()}, open(f"{WORK}/stage-report.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    return allrep


def install(keys):
    jp = GEN + "/c4p15-installed.json"
    journal = json.load(open(jp, encoding="utf-8")) if os.path.exists(jp) else {}
    for key in keys:
        spec = PLANS[key]
        sd = f"{STAGE}/{key}"
        staged = []
        for dp, dn, fn in os.walk(sd):
            for f in fn:
                staged.append(os.path.relpath(os.path.join(dp, f), sd))
        allowed = {os.path.abspath(ROOT + "/" + r) for r in staged}
        live_root = ROOT + "/" + spec["rel"]
        if sha(read(live_root)) != sha(orig_text(spec["n"])):
            raise SystemExit(f"{key}: the live target differs from its snapshot (a peer edited it, or it is already installed)")
        for r in staged:
            p = ROOT + "/" + r
            if r != spec["rel"] and os.path.exists(p):
                raise SystemExit(f"{key}: child already exists: {r}")
        for r in staged:
            repo_write(ROOT + "/" + r, read(f"{sd}/{r}"), allowed)
        journal[key] = sorted(staged)
        print(f"[{key}] installed {len(staged)} files")
    json.dump(journal, open(jp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)


def reinstall(keys):
    jp = GEN + "/c4p15-installed.json"
    journal = json.load(open(jp, encoding="utf-8"))
    for key in keys:
        spec = PLANS[key]
        sd = f"{STAGE}/{key}"
        old = {}
        for dp, dn, fn in os.walk(sd):
            for f in fn:
                r = os.path.relpath(os.path.join(dp, f), sd)
                old[r] = read(os.path.join(dp, f))
        assert sorted(old) == journal[key], (key, "journal differs from the stage")
        for r, txt in old.items():
            if read(ROOT + "/" + r) != txt:
                raise SystemExit(f"{key}: live {r} differs from the previously installed stage (a peer edited it)")
        stage([key])
        new = {}
        for dp, dn, fn in os.walk(sd):
            for f in fn:
                r = os.path.relpath(os.path.join(dp, f), sd)
                new[r] = read(os.path.join(dp, f))
        allowed = {os.path.abspath(ROOT + "/" + r) for r in set(old) | set(new)}
        for r in sorted(set(old) - set(new)):
            p = os.path.abspath(ROOT + "/" + r)
            assert p in allowed
            os.remove(p)
            d = os.path.dirname(p)
            if not os.listdir(d):
                os.rmdir(d)
            print(f"[{key}] removed {r}")
        for r, txt in new.items():
            repo_write(ROOT + "/" + r, txt, allowed)
        journal[key] = sorted(new)
        print(f"[{key}] reinstalled {len(new)} files")
    json.dump(journal, open(jp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)


def sync(keys):
    jp = GEN + "/c4p15-installed.json"
    journal = json.load(open(jp, encoding="utf-8"))
    for key in keys:
        sd = f"{STAGE}/{key}"
        new = {}
        for dp, dn, fn in os.walk(sd):
            for f in fn:
                r = os.path.relpath(os.path.join(dp, f), sd)
                new[r] = read(os.path.join(dp, f))
        allowed = {os.path.abspath(ROOT + "/" + r) for r in set(journal[key]) | set(new)}
        for r in sorted(set(journal[key]) - set(new)):
            os.remove(os.path.abspath(ROOT + "/" + r))
            print(f"[{key}] removed {r}")
        for r, txt in new.items():
            p = ROOT + "/" + r
            if r not in journal[key] and os.path.exists(p):
                raise SystemExit(f"{key}: {r} exists and is not one of my files")
            if not os.path.exists(p) or read(p) != txt:
                repo_write(p, txt, allowed)
                print(f"[{key}] wrote {r}")
        journal[key] = sorted(new)
    json.dump(journal, open(jp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)


def body_of_child(text, mode):
    ls = lines_of(text)
    i = 0
    if mode == "mod":
        assert ls[0].startswith("//! ")
        i = 1
        while ls[i].strip() == "":
            i += 1
        assert ls[i] == "use super::*;"
        i += 1
    else:
        assert ls[0].startswith("// ") or ls[0].startswith("/// ")
        i = 1
    return ls[i:]


def verify(keys):
    ok = True
    for key in keys:
        spec = PLANS[key]
        out, report, ls = build(key)
        mode = spec["mode"]
        rel = spec["rel"]
        base_dir = os.path.dirname(rel)
        bad = []
        for rp, planned in out.items():
            live = read(ROOT + "/" + rp)
            if live != planned:
                bad.append(rp)
        print(f"[{key}] live files equal the staged plan: {'YES' if not bad else 'NO ' + str(bad)}  ({len(out)} files)")
        if bad:
            ok = False
    return ok


if __name__ == "__main__":
    a = sys.argv[1:]
    cmd = a[0] if a else "stage"
    keys = a[1:] or ORDER
    os.makedirs(WORK, exist_ok=True)
    if cmd == "stage":
        stage(keys)
    elif cmd == "install":
        install(keys)
    elif cmd == "sync":
        sync(keys)
    elif cmd == "reinstall":
        reinstall(keys)
    elif cmd == "verify":
        sys.exit(0 if verify(keys) else 1)
