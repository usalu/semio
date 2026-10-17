"""✂️ c4p13 splitter for the FEM engine crate roots (Package 13).
Stages (never installs) the children of PLANS from the pre-edit snapshots `🗑️generated/c4p13-orig/N.rs`, then verifies, installs or restores.
A piece is a plain depth-0 line range (a, b) or an impl slice ("slice", impl_start, a, b): a slice repeats the impl header when it starts inside the body and closes with `}` when it ends inside the body.
Private definitions another file names become `pub(super)`; each file keeps balanced region markers.
usage: c13.py stage | lossless [stage|live] | bodies [stage|live] | check | install | restore | report"""
import collections, hashlib, json, os, re, shutil, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
ORIG = GEN + "/c4p13-orig"
WORK = "/private/tmp/claude-501/-Users-ueli-Documents-semio/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p13-work"
STAGE = WORK + "/stage"
sys.path.insert(0, F + "/frs-tools")
import splitter as S
from rslex import lex, outline
from stubgen import mask

M = "✏️s/🔨️modules/🏗️fem/⚙️engine"
FE = "️"


def D(d, n, key, pkg):
    return dict(n=n, key=key, pkg=pkg, rel=f"{M}/{d}/🦀️.rs")


def C(emoji, kebab, doc, *pieces):
    folder = emoji + FE + kebab
    return dict(folder=folder, mod=kebab.replace("-", "_"), emoji=emoji + FE, doc=doc, pieces=list(pieces))


PLANS = {}
PLANS["elements2d"] = dict(D("📏️elements2d", 1, "elements2d", "semio-s-fem-elements2d"), children=[
    C("🪢", "bars-and-beams", "Segment geometry, the axial `Bar2` truss and the Euler-Bernoulli `BeamEb2` frame member.", (9, 412)),
    C("🔲", "plane-continuum", "Plane-stress and plane-strain continuum family: `Tri3Cst`, `Tri6Lst`, `Quad4` and `Quad8`.", (414, 868)),
    C("🍽", "plate-dkt", "The `PlateDkt` Batoz Discrete Kirchhoff Triangle thin-plate bending element.", (870, 1087)),
])
PLANS["elements3d"] = dict(D("🧱️elements3d", 8, "elements3d", "semio-s-fem-elements3d"), children=[
    C("🌉", "bars-and-frames", "Line geometry, the axial `Bar3` truss and the `Frame3` frame member with torsion and member loads.", (9, 464)),
    C("🧱", "solids", "Solid constitutive helpers and the `Tet4` and `Hex8` solid continuum elements.", (466, 875)),
    C("🐚", "shell-facet", "The `ShellFacet3` flat facet shell: CST membrane, DKT bending and drilling stabilization.", (877, 1094)),
])
PLANS["subspace"] = dict(D("🔢️sparse/🎵️subspace", 2, "subspace", "semio-s-fem-sparse-subspace"), children=[
    C("🎼", "modal-input", "Modal input construction that feeds the subspace iteration.", (10, 261)),
    C("🌀", "job-core", "Dense Jacobi eigen kernel, subspace state types and the job constructor, accessors and checkpoint owners.", (262, 580), ("slice", 582, 582, 948)),
    C("🚶", "iteration-steps", "The bounded iteration steps of the subspace job: operator, factor solves, orthogonalization, Jacobi, sorting, residuals.", ("slice", 582, 950, 1624)),
    C("♻", "restore-cursor", "Checkpoint restore cursor of the subspace job.", (1626, 1953)),
    C("🏃", "run-and-publish", "The subspace job run loop, its interactive step, close turns and the one-shot `subspace_iteration`.", (1955, 2334)),
])
PLANS["pcg"] = dict(D("🔢️sparse/🔁️pcg", 3, "pcg", "semio-s-fem-sparse-pcg"), children=[
    C("🪜", "job-and-steps", "PCG statistics, checkpoint and publication state, and the checkpointed job with its bounded steps.", (10, 490)),
    C("🌱", "construction-and-restore", "Bounded PCG job construction and the retained checkpoint restore cursor.", (493, 955)),
    C("🏁", "run-and-entry", "The PCG job run loop, its interactive step and the one-shot `pcg` entry.", (958, 1120)),
])
PLANS["ldlt"] = dict(D("🔢️sparse/🔺️ldlt", 4, "ldlt", "semio-s-fem-sparse-ldlt"), children=[
    C("🔩", "factor-and-job", "The LDLT factor, its one-shot factorization, the checkpointed job and the factor solve.", (8, 630), (1081, 1126)),
    C("🔄", "restore-cursor", "Checkpoint restore cursor of the LDLT job.", (632, 867)),
    C("🚀", "run-and-close", "The LDLT job run loop, close turns and its interactive step.", (869, 1079)),
])
PLANS["sparse"] = dict(D("🔢️sparse", 5, "sparse", "semio-s-fem-sparse"), children=[
    C("🤝", "numerical-protocol", "Shared numerical job protocol: outcome desk, close ladder, batch steps and owner close helpers.", (11, 402)),
    C("🗃", "storage-and-ordering", "COO, CSR and symmetric CSC storage and the reverse-Cuthill-McKee ordering.", (404, 725), (1240, 1305)),
    C("📄", "checkpoint-pages", "Numerical checkpoint pages: identity, mounted scalar slots, page cursors, owner advance and restore.", (728, 1238)),
])
PLANS["mesh"] = dict(D("🕸️mesh", 6, "mesh", "semio-s-fem-mesh"), children=[
    C("🗺", "planar-domain", "Planar domain, mounted domain owners, mesh options and errors, and point-in-polygon.", (10, 261)),
    C("🔷", "triangulation", "Owned constrained Bowyer-Watson triangulation, input preparation and the one-shot `triangulate`.", (263, 1135)),
    C("🧵", "mesh-job", "The incremental `MeshJob`: stages, cursors, constraint recovery, classification and publication.", (1137, 2058)),
    C("🕹", "mesh-job-run", "The `MeshJob` run loop, interactive step and retirement.", (2060, 2449), (2452, 2567)),
])
PLANS["assembly"] = dict(D("🧮️analyses/🧩️assembly", 7, "assembly", "semio-s-fem-analyses-assembly"), children=[
    C("🧷", "job-state-and-construction", "Assembly job stages, checkpoints, plan and the bounded job construction.", (17, 860)),
    C("🪡", "element-build", "The checkpointed assembly job: element build, partition merge and retained close.", (862, 1508)),
    C("🔶", "csr-build-and-run", "The CSR build, the assembly run loop and the one-shot assembled system.", (1510, 2041)),
])
ORDER = ["elements2d", "elements3d", "subspace", "pcg", "ldlt", "sparse", "mesh", "assembly"]
RX_OPEN = re.compile(r"^\s*//\s?#region (.*)$")
RX_CLOSE = re.compile(r"^\s*//\s?#endregion (.*)$")


def read(p):
    with open(p, encoding="utf-8") as f:
        return f.read()


def write(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w", encoding="utf-8") as f:
        f.write(t)


def lines_of(t):
    ls = t.split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def multiset(ls):
    return collections.Counter(l.strip() for l in ls if l.strip())


def orig_text(spec):
    return read(f"{ORIG}/{spec['n']}.rs")


def child_rel(spec, c):
    return f"{os.path.dirname(spec['rel'])}/{c['folder']}/🦀️.rs"


class W(S.Plan):
    def __init__(self):
        pass

    def _widen_struct(self, body, it, hi, depths, ext_all, ext_fields, edits):
        head = body[hi]
        m = re.search(r"(struct\s+\w+(?:<[^>]*>)?\s*\()(.*)\)\s*;\s*$", head)
        if m and not ("{" in head and "(" not in head.split("{")[0]):
            if it["name"] in ext_all:
                parts = [p.strip() for p in m.group(2).split(",") if p.strip()]
                new = ", ".join(p if p.startswith("pub") else "pub(super) " + p for p in parts)
                base = edits[hi][0] if hi in edits else head
                m2 = re.search(r"(struct\s+\w+(?:<[^>]*>)?\s*\()(.*)\)\s*;\s*$", base)
                edits[hi] = (base[:m2.end(1)] + new + ");", f"tuple fields {it['name']}")
            return
        S.Plan._widen_struct(self, body, it, hi, depths, ext_all, ext_fields, edits)


WID = W()


def balance(piece):
    stack, needs = [], []
    for l in piece:
        m = RX_OPEN.match(l)
        if m:
            stack.append(m.group(1).strip())
            continue
        m = RX_CLOSE.match(l)
        if m:
            n = m.group(1).strip()
            if stack and stack[-1] == n:
                stack.pop()
            else:
                needs.append(n)
    return list(reversed(needs)), list(reversed(stack))


def join_pieces(pieces):
    wrapped = []
    for p in pieces:
        while p and not p[0].strip():
            p = p[1:]
        while p and not p[-1].strip():
            p = p[:-1]
        o, c = balance(p)
        wrapped.append([o, p, c])
    for a, b in zip(wrapped, wrapped[1:]):
        if a[2] and a[2] == list(reversed(b[0])):
            a[2], b[0] = [], []
    out = []
    for o, p, c in wrapped:
        if out:
            out.append("")
        out += [f"// #region {n}" for n in o] + p + [f"// #endregion {n}" for n in c]
    return out


def impl_info(items, s):
    for it in items:
        if it["start"] + 1 == s and it["kind"] == "impl":
            return it
    raise SystemExit(f"no impl starts at line {s}")


def build(key):
    spec = PLANS[key]
    src = orig_text(spec)
    ls = lines_of(src)
    n = len(ls)
    depths, inlit = lex(src)
    items = outline(src)
    owner = [None] * (n + 2)
    bodies = {}
    for c in spec["children"]:
        pieces = []
        for p in c["pieces"]:
            if p[0] == "slice":
                _, s, a, b = p
                it = impl_info(items, s)
                e = it["end"] + 1
                h = s
                while "{" not in mask(ls[h - 1]):
                    h += 1
                assert s <= a <= b <= e, (key, c["folder"], p, e)
                if a > h:
                    assert depths[a - 1] == 1 and not inlit[a - 1], (key, p, "slice start not at impl body depth")
                else:
                    assert a == s, (key, p)
                if b < e:
                    assert depths[b] == 1 and not inlit[b], (key, p, "slice end not at impl body depth")
                hdr = [l for l in ls[s - 1:h] if not l.strip().startswith("///")]
                assert all(l.startswith("impl") or l.strip().startswith("#[") for l in hdr), (key, p, hdr)
                text = ([] if a <= h else hdr) + ls[a - 1:b] + (["}"] if b < e else [])
            else:
                a, b = p
                assert depths[a - 1] == 0 and not inlit[a - 1], (key, c["folder"], a, "range start not at depth 0")
                assert b >= n or (depths[b] == 0 and not inlit[b]), (key, c["folder"], b, "range end not at depth 0")
                text = ls[a - 1:b]
            for i in range(a, b + 1):
                assert owner[i] is None, (key, c["folder"], i, "overlap", owner[i])
                owner[i] = c["folder"]
            pieces.append((a, text))
        pieces.sort(key=lambda x: x[0])
        c["_first"] = pieces[0][0]
        bodies[c["folder"]] = join_pieces([t for _, t in pieces])
    root_lines = [ls[i - 1] for i in range(1, n + 1) if owner[i] is None]
    remainder = "\n".join(root_lines)
    others = [p for p in S.crate_files(ROOT + "/" + spec["rel"]) if os.path.normpath(p) != os.path.normpath(ROOT + "/" + spec["rel"])]
    ext_all0, ext_fields0, ext_methods0 = set(), set(), set()
    for p in others:
        _f, a_ = S.tokenize(read(p))
        ext_all0 |= a_
        fl, me = S.member_refs(read(p))
        ext_fields0 |= fl
        ext_methods0 |= me
    rem_first, rem_all = S.tokenize(remainder)
    rem_fields, rem_methods = S.member_refs(remainder)
    toks = {f: S.tokenize("\n".join(b)) for f, b in bodies.items()}
    mems = {f: S.member_refs("\n".join(b)) for f, b in bodies.items()}
    root_names = set()
    for l in root_lines:
        if l.startswith("use "):
            root_names |= S.tokenize(l)[1]
    out, rep = {}, dict(children={})
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
            assert not body_lit[ln], (key, f, label, "widened line inside a multi-line literal")
            body[ln] = newline
            wid.append(label)
        text_body = "\n".join(body) + "\n"
        dd, _il = lex(text_body)
        assert dd[-1] == 0 or len(dd) <= len(body) + 1, (key, f)
        its = outline(text_body)
        named = [i for i in its if i["kind"] not in ("use", "impl", "other") and i["name"]]
        vis = ""
        if any(i["vis"] == "pub" for i in named):
            vis = "pub "
        elif any(i["vis"].startswith("pub(crate)") for i in named):
            vis = "pub(crate) "
        needs_use = any(i["vis"] for i in named)
        exported = set()
        for g in toks:
            if g != f:
                exported |= {i["name"] for i in outline("\n".join(bodies[g]) + "\n") if i["name"]}
        super_ok = bool(toks[f][0] & (root_names | exported | rem_all))
        c["_vis"], c["_use"] = vis, needs_use
        hdr = f"//! {c['emoji']} {c['doc']}\n\nuse super::*;\n\n"
        out[child_rel(spec, c)] = hdr + text_body
        rep["children"][f] = dict(lines=len(lines_of(hdr + text_body)), widened=wid, vis=vis.strip() or "private", use=needs_use, super_used=super_ok)
    starts = {c["_first"]: c for c in spec["children"]}
    root = []
    for i in range(1, n + 1):
        if i in starts:
            c = starts[i]
            if root and root[-1].strip():
                root.append("")
            root.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
            root.append(f"mod {c['mod']};")
            if c["_use"]:
                root.append(f"{c['_vis']}use {c['mod']}::*;")
            root.append("")
        if owner[i] is None:
            root.append(ls[i - 1])
    tidy = []
    for l in root:
        if l.strip() == "" and tidy and tidy[-1].strip() == "":
            continue
        tidy.append(l)
    while tidy and not tidy[-1].strip():
        tidy.pop()
    out[spec["rel"]] = "\n".join(tidy) + "\n"
    rep["root_lines"] = len(tidy)
    rep["orig_lines"] = n
    rep["root_remainder_nonblank"] = [l for l in root_lines if l.strip()]
    return out, rep, ls


def stage(keys):
    allrep = {}
    for key in keys:
        out, rep, ls = build(key)
        sd = f"{STAGE}/{key}"
        if os.path.exists(sd):
            shutil.rmtree(sd)
        for rel, txt in out.items():
            write(f"{sd}/{rel}", txt)
        rep["files"] = sorted(out)
        allrep[key] = rep
        kids = {k.split(FE)[-1]: v["lines"] for k, v in rep["children"].items()}
        print(f"[{key}] orig={rep['orig_lines']} root={rep['root_lines']} children={kids}")
        for f, r in rep["children"].items():
            print(f"    {f.split(FE)[-1]}: widened {len(r['widened'])} vis={r['vis']} use={r['use']} super_used={r['super_used']}")
    write(WORK + "/stage-report.json", json.dumps(allrep, ensure_ascii=False, indent=1))


def files_of(key):
    spec = PLANS[key]
    return [spec["rel"]] + [child_rel(spec, c) for c in spec["children"]]


def path_of(key, rel, which):
    return (f"{STAGE}/{key}/{rel}") if which == "stage" else (ROOT + "/" + rel)


def cls_removed_added():
    pass


def lossless(which, keys):
    ok = True
    for key in keys:
        spec = PLANS[key]
        orig = lines_of(orig_text(spec))
        new = []
        for rel in files_of(key):
            new.extend(lines_of(read(path_of(key, rel, which))))
        gone = multiset(orig) - multiset(new)
        added = multiset(new) - multiset(orig)
        hdrs = {l.strip() for l in orig if re.match(r"^impl\b", l)}

        def cls(l):
            if l.startswith("#[path = ") or re.match(r"^(pub(\([^)]*\))? )?use \w+::\*;$", l) or re.match(r"^mod \w+;$", l):
                return "wiring"
            if l.startswith("//!"):
                return "child docstring"
            if RX_OPEN.match(l) or RX_CLOSE.match(l):
                return "region marker"
            if "pub(super)" in l:
                return "widened"
            if l in hdrs:
                return "impl header copy"
            if l == "}":
                return "impl close"
            return "other"
        ga, ad = collections.Counter(), collections.Counter()
        for l, c in gone.items():
            ga[cls(l)] += c
        for l, c in added.items():
            ad[cls(l)] += c
        print(f"[{key}] original={len(orig)} lines files={len(files_of(key))} new total={len(new)}")
        print(f"   removed {sum(gone.values())} {dict(ga)}   added {sum(added.values())} {dict(ad)}")
        for l, c in gone.items():
            twin = [a for a in added if "pub(super)" in a and a.replace("pub(super) ", "") == l]
            if not twin and cls(l) != "widened":
                ok = False
                print("   !! removed without widened twin:", c, l[:150])
        for l, c in added.items():
            if cls(l) == "other":
                ok = False
                print("   !! added other:", c, l[:150])
        reg_o = collections.Counter(re.sub(r"^\s*//\s?#(end)?region ", "", l) for l in orig if RX_OPEN.match(l) or RX_CLOSE.match(l))
        print("   region markers original:", sum(1 for l in orig if RX_OPEN.match(l)), "open", sum(1 for l in orig if RX_CLOSE.match(l)), "close;",
              "new:", sum(1 for l in new if RX_OPEN.match(l)), "open", sum(1 for l in new if RX_CLOSE.match(l)), "close")
    print("LOSSLESS", "OK" if ok else "FAILED")
    return ok


def fn_bodies(text):
    m = mask(text)
    assert len(m) == len(text)
    res = []
    for mm in re.finditer(r"\bfn\s+([A-Za-z_]\w*)", m):
        i, depth = mm.end(), 0
        while i < len(m):
            ch = m[i]
            if ch in "([":
                depth += 1
            elif ch in ")]":
                depth -= 1
            elif depth == 0 and ch == ";":
                break
            elif depth == 0 and ch == "{":
                d, j = 0, i
                while j < len(m):
                    if m[j] == "{":
                        d += 1
                    elif m[j] == "}":
                        d -= 1
                        if d == 0:
                            break
                    j += 1
                sig = re.sub(r"\s+", " ", text[mm.start():i]).strip()
                res.append((mm.group(1), hashlib.sha256((sig + "\0" + text[i:j + 1]).encode()).hexdigest()))
                break
            i += 1
    return res


def bodies(which, keys):
    ok = True
    for key in keys:
        spec = PLANS[key]
        a = collections.Counter(fn_bodies(orig_text(spec)))
        b = collections.Counter()
        for rel in files_of(key):
            b.update(fn_bodies(read(path_of(key, rel, which))))
        same = a == b
        ok &= same
        print(f"[{key}] fn items original={sum(a.values())} new={sum(b.values())} signature+body multiset identical={same}")
        if not same:
            for k, v in (a - b).items():
                print("   lost", k[0], v)
            for k, v in (b - a).items():
                print("   extra", k[0], v)
    print("BODIES", "OK" if ok else "FAILED")
    return ok


def mounts(text, base):
    t = S.mask_keep_strings(text)
    return [os.path.normpath(os.path.join(base, m.group(1))) for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', t)]


def closure(lib):
    seen, order = set(), []

    def visit(p):
        p = os.path.normpath(p)
        if p in seen or not os.path.exists(p):
            return
        seen.add(p)
        order.append(p)
        for t in mounts(read(p), os.path.dirname(p)):
            visit(t)
    visit(lib)
    return order


RX_TEST = re.compile(r"#\[\s*(?:[\w:]*::)?(?:test|async_test|tokio::test)\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)")


def check(keys):
    ok = True
    emojis = collections.defaultdict(list)
    for key in keys:
        spec = PLANS[key]
        root = ROOT + "/" + spec["rel"]
        cl = closure(root)
        cnt = collections.Counter(os.path.relpath(p, ROOT) for p in cl)
        dups = {k: v for k, v in cnt.items() if v > 1}
        miss = []
        for p in cl:
            for t in mounts(read(p), os.path.dirname(p)):
                if not os.path.isfile(t):
                    miss.append(os.path.relpath(t, ROOT))
        mod_names = collections.Counter(re.findall(r"^\s*(?:pub(?:\([^)]*\))? )?mod (\w+);", "\n".join(read(p) for p in cl), re.M))
        dupmods = {k: v for k, v in mod_names.items() if v > 1}
        orig = orig_text(spec)
        a = sorted(m.group(1) for m in RX_TEST.finditer(orig))
        b = sorted(m.group(1) for rel in files_of(key) for m in RX_TEST.finditer(read(ROOT + "/" + rel)))
        bad = []
        for c in spec["children"]:
            rel = child_rel(spec, c)
            p = ROOT + "/" + rel
            if not os.path.isfile(p):
                bad.append(("missing", rel))
                continue
            ls = lines_of(read(p))
            if len(rel.encode()) > 240:
                bad.append(("path>240", rel))
            if len(ls) > 1000:
                bad.append(("lines>1000", rel, len(ls)))
            if cnt.get(rel, 0) != 1:
                bad.append(("reach", rel, cnt.get(rel, 0)))
            m = re.match(r"^//! (\S+) ", ls[0])
            emojis[m.group(1) if m else None].append(rel)
            ro, rc = collections.Counter(), collections.Counter()
            for l in ls:
                x = RX_OPEN.match(l)
                if x:
                    ro[x.group(1).strip()] += 1
                x = RX_CLOSE.match(l)
                if x:
                    rc[x.group(1).strip()] += 1
            if ro != rc:
                bad.append(("regions unbalanced", rel))
        rl = lines_of(read(root))
        print(f"[{key}] root={len(rl)} lines closure={len(cl)} dup-mounts={dups} dup-mod-names={dupmods} missing={miss} test fns {len(a)}=={len(b)} identical={a == b} problems={bad}")
        ok &= not dups and not miss and not dupmods and a == b and not bad
    d = {k: v for k, v in emojis.items() if len(v) > 1 or k is None}
    print("duplicate or missing docstring emoji:", d)
    ok &= not d
    print("CHECKS", "OK" if ok else "FAILED")
    return ok


def install(keys):
    jp = GEN + "/c4p13-installed.json"
    journal = json.load(open(jp, encoding="utf-8")) if os.path.exists(jp) else {}
    for key in keys:
        spec = PLANS[key]
        if key in journal:
            print(f"[{key}] already installed")
            continue
        sd = f"{STAGE}/{key}"
        rels = files_of(key)
        live = read(ROOT + "/" + spec["rel"])
        assert live == orig_text(spec), f"{key}: live root differs from the snapshot"
        allowed = {os.path.abspath(ROOT + "/" + r) for r in rels}
        for r in rels[1:]:
            p = ROOT + "/" + r
            assert not os.path.exists(p), r
            assert os.path.abspath(p) in allowed
            write(p, read(f"{sd}/{r}"))
        write(ROOT + "/" + spec["rel"], read(f"{sd}/{spec['rel']}"))
        journal[key] = dict(root=spec["rel"], children=rels[1:])
        write(jp, json.dumps(journal, ensure_ascii=False, indent=1))
        print(f"[{key}] installed root + {len(rels) - 1} children")


def restore(keys):
    jp = GEN + "/c4p13-installed.json"
    journal = json.load(open(jp, encoding="utf-8")) if os.path.exists(jp) else {}
    for key in keys:
        spec = PLANS[key]
        write(ROOT + "/" + spec["rel"], orig_text(spec))
        for r in files_of(key)[1:]:
            p = ROOT + "/" + r
            if os.path.exists(p):
                os.unlink(p)
                try:
                    os.rmdir(os.path.dirname(p))
                except OSError:
                    pass
        journal.pop(key, None)
        print(f"[{key}] restored")
    write(jp, json.dumps(journal, ensure_ascii=False, indent=1))


if __name__ == "__main__":
    a = sys.argv[1:]
    cmd = a[0] if a else "stage"
    rest = a[1:]
    which = "live" if cmd in ("lossless", "bodies") and rest[:1] == ["live"] else "stage"
    if cmd in ("lossless", "bodies") and rest[:1] in (["live"], ["stage"]):
        rest = rest[1:]
    keys = rest or ORDER
    {"stage": lambda: stage(keys), "lossless": lambda: lossless(which, keys), "bodies": lambda: bodies(which, keys),
     "check": lambda: check(keys), "install": lambda: install(keys), "restore": lambda: restore(keys)}[cmd]()
