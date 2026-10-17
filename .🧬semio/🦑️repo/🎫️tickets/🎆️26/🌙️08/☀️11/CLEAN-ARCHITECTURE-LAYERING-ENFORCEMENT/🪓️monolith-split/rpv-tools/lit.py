"""🧱 RPV E0451 scan: rustc reports E0451 (private field in a struct literal or struct pattern) only in the privacy pass, which never runs while any other error exists (micro tests m1 and m3), so the union oracle is blind to it.
This scan finds brace-struct literals/patterns whose struct is defined in the family and names a field that is not visible at the use site: not `pub` for another crate, or private/`pub(super)` outside the defining module (module tree from `mod x;`, `#[path]` and `include!`;
inline modules are honoured for the same file, descendants of the defining file always count as inside). `..base` needs every field visible. Fields behind `cfg(any(test, feature = "testkit"))` count as `pub`.
Read-only. usage: lit.py <family.json> [<out-tag>]   (POS_PREFIX=rpv)"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../pos-tools")
import posunion as U
from poslib import *
from posstub import mask

spec = json.load(open(sys.argv[1], encoding="utf-8"))
tag = sys.argv[2] if len(sys.argv) > 2 else "lit"
mans = Manifests(Source(None))
mans.build_index()
crates = U.prepare(Source(None), mans, spec["crates"], False)


def balanced(t, i, o="{", c="}"):
    d = 0
    n = len(t)
    while i < n:
        ch = t[i]
        if ch == o:
            d += 1
        elif ch == c:
            d -= 1
            if d == 0:
                return i
        i += 1
    return -1


def split_top(body):
    parts, cur, d, i = [], [], 0, 0
    n = len(body)
    while i < n:
        ch = body[i]
        if ch == "-" and i + 1 < n and body[i + 1] == ">":
            cur.append("->")
            i += 2
            continue
        if ch in "([{<":
            d += 1
        elif ch in ")]}>":
            d -= 1
        if ch == "," and d == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(ch)
        i += 1
    parts.append("".join(cur))
    return [p.strip() for p in parts if p.strip()]


texts = {}
parent = {}
SELFTEST = os.environ.get("LIT_SELFTEST") == "1"
fid = {}
for c in crates:
    root = c.lib_rel
    for rel in c.code:
        tx = c.files[rel].decode("utf-8", "replace")
        if SELFTEST and rel == root and c is crates[0]:
            tx += "\nmod zz_a { pub struct ZzProbe { f: u8, pub g: u8, pub(super) h: u8 } }\nmod zz_b { fn t() { let _ = ZzProbe { f: 1, g: 2, h: 3 }; } }\nmod zz_a2 { use super::zz_a::*; fn t() { let _ = ZzProbe { f: 1, g: 2, h: 3 }; } }\nmod zz_c { pub struct ZzOk { f: u8 } mod zz_d { fn t() { let _ = super::ZzOk { f: 1 }; } } }\n"
        texts[(c.pkg, rel)] = (tx, mask(tx))
    stack = [root]
    seen = set()
    fid[(c.pkg, root)] = root
    parent[(c.pkg, root)] = None
    while stack:
        rel = stack.pop()
        if rel in seen or rel not in c.code:
            continue
        seen.add(rel)
        tx, mt = texts[(c.pkg, rel)]
        d = os.path.dirname(rel)
        stem = os.path.splitext(os.path.basename(rel))[0]
        me = fid[(c.pkg, rel)]
        for m in U.INC_CODE_RX.finditer(tx):
            tgt = U.norm(os.path.join(d, m.group(1)))
            if tgt in c.code and (c.pkg, tgt) not in fid:
                fid[(c.pkg, tgt)] = me
                stack.append(tgt)
        for kind, comps, name, pth in U.module_children(tx, mt):
            sub = os.path.join(*comps) if comps else ""
            cands = []
            if pth is not None:
                cands.append(U.norm(os.path.join(d, sub, pth)))
            else:
                for b_ in ([sub] if comps else [sub, os.path.join(sub, stem)]):
                    for suffix in (name + ".rs", os.path.join(name, "mod.rs")):
                        cands.append(U.norm(os.path.join(d, b_, suffix)))
            for cn in cands:
                if cn in c.files and cn in c.code and (c.pkg, cn) not in fid:
                    fid[(c.pkg, cn)] = cn
                    parent[(c.pkg, cn)] = me
                    stack.append(cn)
                    break
        for m in U.PATH_RX.finditer(tx):
            tgt = U.norm(os.path.join(d, m.group(1)))
            if tgt in c.code and (c.pkg, tgt) not in fid:
                fid[(c.pkg, tgt)] = tgt
                parent[(c.pkg, tgt)] = me
                stack.append(tgt)


def chain(pkg, rel):
    f = fid.get((pkg, rel), rel)
    out = [f]
    while parent.get((pkg, f)):
        f = parent[(pkg, f)]
        out.append(f)
    return out


INLINE_RX = re.compile(r"\bmod\s+([A-Za-z_]\w*)\s*\{")
nest_cache = {}


def nest_at(pkg, rel, off):
    k = (pkg, rel)
    if k not in nest_cache:
        mt = texts[k][1]
        spans = []
        for m in INLINE_RX.finditer(mt):
            e = balanced(mt, m.end() - 1)
            if e > 0:
                spans.append((m.end() - 1, e, m.group(1)))
        nest_cache[k] = spans
    return tuple(n for s, e, n in sorted(nest_cache[k]) if s <= off <= e)


DEF_RX = re.compile(r"\bstruct\s+([A-Za-z_]\w*)")
defs = collections.defaultdict(list)
for (pkg, rel), (tx, mt) in texts.items():
    for m in DEF_RX.finditer(mt):
        j, d = m.end(), 0
        n = len(mt)
        kind = None
        while j < n:
            ch = mt[j]
            if ch == "-" and mt[j + 1:j + 2] == ">":
                j += 2
                continue
            if ch == "<":
                d += 1
            elif ch == ">":
                d -= 1
            elif d == 0 and ch in "{(;":
                kind = ch
                break
            j += 1
        if kind != "{":
            continue
        e = balanced(mt, j)
        if e < 0:
            continue
        fields = {}
        for p in split_top(mt[j + 1:e]):
            attrs_txt = p[:600]
            gated = 'feature="testkit"' in re.sub(r"\s+", "", attrs_txt)
            neg = "cfg(not(any(test" in re.sub(r"\s+", "", attrs_txt)
            p = re.sub(r"#\s*\[(?:[^\[\]]|\[[^\]]*\])*\]\s*", "", p).strip()
            fm = re.match(r"^(pub(?:\s*\([^)]*\))?\s+)?([A-Za-z_]\w*)\s*:(?!:)", p)
            if not fm:
                continue
            if neg:
                fields.setdefault(fm.group(2), (fm.group(1) or "").strip() or "private")
                continue
            vis = (fm.group(1) or "").strip() or "private"
            vis = re.sub(r"\s+", "", vis)
            fields[fm.group(2)] = "testkit" if (gated and vis == "pub") else vis
        line = tx.count("\n", 0, m.start()) + 1
        defs[m.group(1)].append(dict(pkg=pkg, rel=rel, line=line, fields=fields, nest=nest_at(pkg, rel, m.start())))


def visible(vis, d, pkg, rel, off):
    if vis in ("pub", "testkit"):
        return True
    if d["pkg"] != pkg:
        return False
    if vis.startswith("pub(crate)") or vis.startswith("pub(in"):
        return True
    ch = chain(pkg, rel)
    df = fid.get((d["pkg"], d["rel"]), d["rel"])
    if vis.startswith("pub(super)"):
        par = parent.get((d["pkg"], df))
        if d["nest"]:
            return df in ch
        return par is None or par in ch
    if df not in ch:
        return False
    if ch[0] == df:
        un = nest_at(pkg, rel, off)
        return un[:len(d["nest"])] == d["nest"]
    return True


LIT_RX = re.compile(r"(?<![\w$.])((?:[A-Za-z_]\w*::)*)([A-Z]\w*)\s*(?:::\s*<[^<>{};]*>)?\s*\{")
SKIP_PREV = re.compile(r"(?:\bstruct|\benum|\bunion|\btrait|\bimpl|\bfor|\bmod|\btype|\bdyn|\bwhere|\belse|\bunsafe|\basync|\bmove|->|=>)\s*$")
hits = []
for (pkg, rel), (tx, mt) in texts.items():
    for m in LIT_RX.finditer(mt):
        name = m.group(2)
        if name not in defs or name == "Self":
            continue
        pre = mt[max(0, m.start() - 24):m.start()]
        if SKIP_PREV.search(pre):
            continue
        s = m.end() - 1
        e = balanced(mt, s)
        if e < 0:
            continue
        body = mt[s + 1:e]
        if not re.match(r"^\s*(\}|$|\.\.|[A-Za-z_]\w*\s*(:(?!:)|,|$))", body):
            continue
        used, base = [], False
        for p in split_top(body):
            if p.startswith(".."):
                base = True
                continue
            fm = re.match(r"^([A-Za-z_]\w*)\s*(:(?!:)|$)", p)
            if fm:
                used.append(fm.group(1))
        if not used and not base:
            continue
        results = []
        for d in defs[name]:
            if used and not all(f in d["fields"] for f in used):
                continue
            need = list(used)
            if base:
                need += [f for f in d["fields"] if f not in used]
            bad = [f for f in need if not visible(d["fields"][f], d, pkg, rel, m.start())]
            results.append((d, bad))
        if results and all(b for d, b in results):
            line = tx.count("\n", 0, m.start()) + 1
            hits.append(dict(user=pkg, file=rel, line=line, name=name, used=used, base=base, defs=[(d["pkg"], d["rel"], d["line"], b, [d["fields"][f] for f in b]) for d, b in results]))
json.dump(hits, open(f"{GEN}/{PREFIX}-{tag}-e0451.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
S = lambda s: s.replace("semio-framework-os-renderer-wgpu", "~")
print(len(hits), "candidate E0451 sites;", len(defs), "brace structs;", sum(len(v) for v in defs.values()), "definitions;", len(fid), "files in the module tree")
by = collections.Counter((S(h["user"]), S(h["defs"][0][0]), h["name"]) for h in hits)
for k, n in by.most_common(80):
    print(f"  {n:3} {k[0]} -> {k[1]} {k[2]}")
