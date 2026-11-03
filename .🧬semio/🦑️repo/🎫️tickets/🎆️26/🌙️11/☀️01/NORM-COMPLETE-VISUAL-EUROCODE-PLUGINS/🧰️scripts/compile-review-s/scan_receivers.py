"""🔎 `<expr>.<field>.<method>(` where <field> has a unique non-primitive named type T and no impl of T (nor a repo trait impl for T / blanket impl) defines <method>."""
import re, sys, os, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

PRIM = set("f64 f32 u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize bool char String str Vec Option Result HashMap BTreeMap HashSet BTreeSet Box Arc Rc".split())
STD = set("clone to_string to_owned into try_into eq ne cmp partial_cmp hash fmt as_ref as_mut borrow default to_value from_value iter into_iter len is_empty map unwrap expect max min clamp cloned copied is_some is_none ok_or ok_or_else and_then".split())
cache = os.path.join(os.path.dirname(__file__), "..", "..", "🗑️generated", "compile-review-s", "impl-index.pkl")
if os.path.exists(cache): METH, BLANKET = pickle.load(open(cache, "rb"))
else:
    METH, BLANKET = {}, set()
    IMPL = re.compile(r'\bimpl\b\s*(?:<[^{;]*?>)?\s*(?:([\w:]+)(?:<[^{;]*?>)?\s+for\s+)?(&?\s*[\w:]+)(?:<[^{;]*?>)?\s*(?:where[^{;]*)?\{')
    TRAIT = re.compile(r'\btrait\s+(\w+)[^{;]*\{')
    traitfns = {}
    for r in ["/home/deck/semio/🧰️framework", NORM]:
        for f in rs_files(r):
            try: s = strip(open(f, encoding="utf-8").read())
            except Exception: continue
            for m in TRAIT.finditer(s):
                e = match_close(s, m.end() - 1)
                if e > 0: traitfns.setdefault(m.group(1), set()).update(re.findall(r'\bfn\s+(\w+)', s[m.end():e]))
            for m in IMPL.finditer(s):
                e = match_close(s, m.end() - 1)
                if e < 0: continue
                tr, ty = m.group(1), m.group(2).replace("&", "").strip().split("::")[-1]
                fns = set(re.findall(r'\bfn\s+(\w+)', s[m.end():e]))
                if tr: fns |= traitfns.get(tr.split("::")[-1], set())
                if len(ty) == 1 or ty in ("T", "U"): BLANKET |= fns
                else: METH.setdefault(ty, set()).update(fns)
    # trait default methods apply to implementors: second pass by trait name
    pickle.dump((METH, BLANKET), open(cache, "wb"))
code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, enums, files = index_types(adir)
ftype = {}
for s, defs in structs.items():
    for _, fl in defs:
        for f, t in fl.items():
            ftype.setdefault(f, set()).add(t)
uniq = {}
for f, ts in ftype.items():
    if len(ts) != 1: continue
    t = next(iter(ts)); b = re.sub(r'^(?:Option|Box)<(.*)>$', r'\1', t).split("::")[-1]
    if re.fullmatch(r'[A-Z]\w+', b) and b not in PRIM and (b in enums or b in structs or b in METH): uniq[f] = b
for f, src in files.items():
    for m in re.finditer(r'\.(\w+)\.(\w+)\s*(?:::<[^>]*>)?\(', src):
        fld, meth = m.group(1), m.group(2)
        if fld not in uniq or meth in STD or meth in BLANKET: continue
        T = uniq[fld]
        if meth in METH.get(T, set()): continue
        if T in structs and meth in structs[T][0][1]: continue
        print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: `.{fld}.{meth}()` field type {T} has no method {meth}")
