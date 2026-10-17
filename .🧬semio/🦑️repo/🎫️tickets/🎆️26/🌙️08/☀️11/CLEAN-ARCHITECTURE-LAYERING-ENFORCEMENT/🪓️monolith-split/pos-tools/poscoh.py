"""⚖️ POS coherence scan. A single-crate union cannot see crate-level coherence (E0116 inherent impl on a foreign type, E0117 orphan trait impl, E0210 uncovered type parameter), the classic failure
of a crate carve. This scan reads the family sources (read-only), maps simple item names to their defining crate and reports `impl` headers whose Self type or trait lives in another crate.
It is heuristic (simple-name resolution, macro-generated items unseen) and reports candidates, never verdicts.
usage: poscoh.py <family.json> [--rev REV]   (default: working tree, the family's `crates`)"""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from poslib import *
from posunion import reach
from posstub import mask

DEF_RX = re.compile(r"\b(struct|enum|union|trait|type)\s+([A-Za-z_]\w*)")
IMPL_RX = re.compile(r"(?:^|(?<=[;}{\]]))\s*(?:unsafe\s+)?impl\b", re.M)
FUND = {"Box", "Pin", "Rc", "Arc"}
STD_ADT = {"Vec", "Option", "Result", "String", "HashMap", "HashSet", "BTreeMap", "BTreeSet", "VecDeque", "Cow", "Cell", "RefCell", "Mutex", "RwLock", "PhantomData", "Duration", "Path", "PathBuf", "Range", "RangeInclusive"}
PRIM = {"u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32", "f64", "bool", "char", "str"}


def split_top(s, sep=","):
    out, cur, d = [], "", 0
    for ch in s:
        if ch in "<([":
            d += 1
        elif ch in ">)]":
            d -= 1
        if ch == sep and d == 0:
            out.append(cur)
            cur = ""
        else:
            cur += ch
    out.append(cur)
    return [x.strip() for x in out if x.strip()]


def header(masked, pos):
    """Return the impl header text after `impl` up to the block `{` or `;` at depth 0 (where clause cut)."""
    i = masked.index("impl", pos) + 4
    d, j = 0, i
    while j < len(masked):
        ch = masked[j]
        if ch in "<([":
            d += 1
        elif ch in ">)]" and not (ch == ">" and masked[j - 1] == "-"):
            d -= 1
        elif ch in "{;" and d <= 0:
            break
        j += 1
    h = masked[i:j]
    k = re.search(r"\bwhere\b", h)
    return (h[:k.start()] if k else h).strip()


def head_of(ty):
    t = ty.strip()
    t = re.sub(r"^(&\s*('\w+\s+)?(mut\s+)?)+", "", t)
    t = re.sub(r"^(dyn|impl)\s+", "", t)
    m = re.match(r"^((?:::)?[A-Za-z_]\w*(?:::[A-Za-z_]\w*)*)\s*(<.*>)?$", t, re.S)
    if not m:
        return None, []
    path = m.group(1).lstrip(":").split("::")
    args = split_top(m.group(2)[1:-1]) if m.group(2) else []
    return path[-1], args


def parse_header(h):
    gen = ""
    if h.startswith("<"):
        d = 0
        for i, ch in enumerate(h):
            if ch == "<":
                d += 1
            elif ch == ">" and h[i - 1] != "-":
                d -= 1
                if d == 0:
                    gen, h = h[1:i], h[i + 1:].strip()
                    break
    params = {re.match(r"^\s*(?:const\s+)?('?\w+)", g).group(1) for g in split_top(gen) if re.match(r"^\s*(?:const\s+)?'?\w+", g)}
    neg = h.startswith("!")
    if neg:
        h = h[1:].strip()
    parts = re.split(r"\bfor\b(?![^<]*>)", h, maxsplit=1)
    if len(parts) == 2:
        return params, parts[0].strip(), parts[1].strip()
    return params, None, h.strip()


def scan(pkgs, rev=None):
    src = Source(rev)
    mans = Manifests(src)
    mans.build_index()
    crates = {}
    for pkg in pkgs:
        man = mans.manifest(pkg, src)
        if not man:
            continue
        files, code, _, _ = reach(src, mans.lib_rel(man), man["dir"])
        crates[pkg] = (files, code)
    defs = collections.defaultdict(dict)
    texts = {}
    for pkg, (files, code) in crates.items():
        for rel in sorted(code):
            t = mask(files[rel].decode("utf-8", "replace"))
            texts[(pkg, rel)] = t
            for m in DEF_RX.finditer(t):
                kind, name = m.group(1), m.group(2)
                defs[name].setdefault(pkg, set()).add(kind)
    out = []
    for (pkg, rel), t in texts.items():
        for m in IMPL_RX.finditer(t):
            try:
                h = header(t, m.start())
            except ValueError:
                continue
            params, tr, selfty = parse_header(h)
            line = t.count("\n", 0, m.start()) + 1
            sname, sargs = head_of(selfty)
            while sname in FUND and sargs:
                sname, sargs = head_of(sargs[0])
            if tr is None:
                if sname and sname in defs and pkg not in defs[sname] and len(defs[sname]) == 1:
                    out.append(dict(code="E0116", pkg=pkg, file=rel, line=line, msg=f"inherent impl on `{sname}` defined in {next(iter(defs[sname]))}", header=h[:140]))
                elif sname in STD_ADT or sname in PRIM:
                    out.append(dict(code="E0116/E0390", pkg=pkg, file=rel, line=line, msg=f"inherent impl on std/primitive `{sname}`", header=h[:140]))
                continue
            tname, targs = head_of(tr)
            if tname is None:
                continue
            tr_local = pkg in defs.get(tname, {})
            tr_foreign_family = tname in defs and pkg not in defs[tname] and len(defs[tname]) == 1
            if tr_local:
                continue
            local_ty = lambda n: n is not None and pkg in defs.get(n, {})
            if sname in params:
                out.append(dict(code="E0210", pkg=pkg, file=rel, line=line, msg=f"uncovered type parameter `{sname}` for foreign trait `{tname}`" + (f" (defined in {next(iter(defs[tname]))})" if tr_foreign_family else " (external/std trait)"), header=h[:140]))
                continue
            if local_ty(sname):
                continue
            targ_local = False
            for a in targs:
                an, aargs = head_of(a)
                while an in FUND and aargs:
                    an, aargs = head_of(aargs[0])
                if local_ty(an):
                    targ_local = True
            if targ_local:
                continue
            self_foreign = sname in defs and pkg not in defs[sname] and len(defs[sname]) == 1
            tr_known = tr_foreign_family or tname in {"Display", "Debug", "From", "Into", "Default", "Clone", "PartialEq", "Eq", "Hash", "PartialOrd", "Ord", "Iterator", "Drop", "Deref", "Add", "Sub", "FromStr", "TryFrom", "AsRef", "Error", "Serialize", "Deserialize", "Send", "Sync"}
            if (self_foreign or sname in STD_ADT or sname in PRIM) and tr_known:
                out.append(dict(code="E0117", pkg=pkg, file=rel, line=line, msg=f"orphan impl of `{tname}` for `{sname}`" + (f" (self in {next(iter(defs[sname]))})" if self_foreign else " (std type)") + (f", trait in {next(iter(defs[tname]))}" if tr_foreign_family else ""), header=h[:140]))
    return out


def main():
    spec = json.load(open(sys.argv[1], encoding="utf-8"))
    rev = None
    if "--rev" in sys.argv:
        rev = sys.argv[sys.argv.index("--rev") + 1]
    res = scan(spec["crates"], rev)
    by = collections.Counter(r["code"] for r in res)
    lines = [f"== coherence candidates for {spec['name']} ({len(res)}): {dict(by)}"]
    for r in res:
        lines.append(f"{r['code']} [{r['pkg']}] {r['file']}:{r['line']}: {r['msg']} | impl{r['header']}")
    txt = "\n".join(lines) + "\n"
    log_write(f"{GEN}/{PREFIX}-{spec['name']}-coherence.txt", txt)
    print(txt[:6000])


if __name__ == "__main__":
    main()
