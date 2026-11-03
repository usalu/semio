"""🔎 E0499/E0502 class: `x.m(.., x.n(..))` where `m` and `n` are both `&mut self` methods (repo definitions plus common std mutators), or `x.m(.., x.field_or_ref)` with `m` mutable and the argument a reference/call result that borrows x (`x.iter()`, `x.first()`, `&x.f`)."""
import re, sys, os, pickle
S = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/compile-review-s"
sys.path.insert(0, S)
from rs import *
code = sys.argv[1]
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🗑️generated/compile-review-t/mutself-index.pkl"
FN = re.compile(r'\bfn\s+(\w+)\s*(?:<[^(]*?>)?\s*\(\s*&\s*(?:\'\w+\s+)?(mut\s+)?self\b')
if os.path.exists(T): MUT, SHARED = pickle.load(open(T, "rb"))
else:
    MUT, SHARED = set(), set()
    for f in rs_files(NORM):
        try: s = strip(open(f, encoding="utf-8").read())
        except Exception: continue
        for m in FN.finditer(s): (MUT if m.group(2) else SHARED).add(m.group(1))
    pickle.dump((MUT, SHARED), open(T, "wb"))
STDMUT = set("push pop insert remove push_str extend clear entry get_mut iter_mut drain retain sort sort_by truncate append push_back pop_front next take dedup swap_remove or_insert".split())
STDSHARED = set("len iter first last get contains is_empty keys values as_str clone to_string".split())
MUT = (MUT | STDMUT) - (SHARED - STDMUT)
MUT -= {"new", "next"}
root = ART + "/" + CODE[code]
for f in rs_files(root):
    try: s = strip(open(f, encoding="utf-8").read())
    except Exception: continue
    for m in re.finditer(r'(?<![\w.])([a-z_]\w*)\.([a-z_]\w*)\s*(?:::<[^>]*>)?\(', s):
        recv, meth = m.groups()
        if meth not in MUT or recv in ("self",) and False: continue
        o = m.end() - 1
        e = match_close(s, o)
        if e < 0: continue
        body = s[o + 1:e]
        for n in re.finditer(r'(?<![\w.])' + re.escape(recv) + r'\.([a-z_]\w*)\s*\(', body):
            if n.group(1) in MUT and n.group(1) not in ("pop", "take", "next") or n.group(1) in ("pop", "take", "next"):
                print(f"mut-mut {rel(f)}:{s.count(chr(10), 0, m.start()) + 1}: {recv}.{meth}( .. {recv}.{n.group(1)}(")
                break
        else:
            pre = body
            if re.search(r'(?<![\w.])&?' + re.escape(recv) + r'\.(?:iter|first|last|get|as_str|keys|values|as_slice|as_ref)\s*\(', pre) and not re.search(r'\.(cloned|copied|clone|to_vec|to_string|to_owned|collect|len|count|sum|is_some|is_none)\s*\(', pre) and meth in ("push", "insert", "extend", "push_str", "retain", "append"):
                print(f"mut-shared {rel(f)}:{s.count(chr(10), 0, m.start()) + 1}: {recv}.{meth}( .. {pre.strip()[:70]}")
