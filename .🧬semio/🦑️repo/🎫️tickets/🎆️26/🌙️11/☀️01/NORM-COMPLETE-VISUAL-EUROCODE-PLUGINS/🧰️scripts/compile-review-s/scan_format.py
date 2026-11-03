"""🔎 `{}` formatting / `.to_string()` of enum-typed fields whose enum has no Display impl (typed-enums lost the implicit String)."""
import re, sys, os, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, enums, _ = index_types(adir)
cache = os.path.join(os.path.dirname(__file__), "..", "..", "🗑️generated", "compile-review-s", "norm-enums.pkl")
for k, v in pickle.load(open(cache, "rb")).items(): enums.setdefault(k, [("", v)])
raw_files = {}
for f in rs_files(adir):
    raw_files[f] = strip(open(f, encoding="utf-8").read(), keep_strings=True)
disp = set()
for src in raw_files.values():
    disp |= set(re.findall(r'impl(?:<[^>]*>)?\s+(?:std::fmt::|core::fmt::|fmt::)?Display\s+for\s+(\w+)', src))
def base(t):
    for _ in range(3): t = re.sub(r'^(?:Option|Vec|Box)<(.*)>$', r'\1', t)
    return t.split("::")[-1]
kinds = {}
for s, defs in structs.items():
    for _, fl in defs:
        for f, t in fl.items():
            b = base(t)
            kinds.setdefault(f, set()).add(("enum", b) if b in enums and b not in structs else ("other", b))
nodisp = {f for f, k in kinds.items() if len(k) == 1 and next(iter(k))[0] == "enum" and next(iter(k))[1] not in disp}
MAC = re.compile(r'\b(format|write|writeln|println|eprintln|print|eprint|panic|assert|assert_eq|assert_ne|debug_assert|unreachable|todo|format_args)!\s*\(')
for f, src in raw_files.items():
    for m in MAC.finditer(src):
        e = match_close(src, m.end() - 1)
        if e < 0: continue
        args = split_top(src[m.end():e], angles=False)
        fi = next((i for i, a in enumerate(args) if a.strip().startswith('"')), None)
        if fi is None: continue
        fmt = args[fi].strip()[1:-1]
        ph = re.findall(r'\{([^{}:]*)(:[^{}]*)?\}', fmt.replace("{{", "").replace("}}", ""))
        pos = [p for p in ph if p[0] == "" or p[0].isdigit()]
        rest = [a.strip() for a in args[fi + 1:]]
        for i, (nm, spec) in enumerate(pos):
            if "?" in (spec or ""): continue
            idx = int(nm) if nm.isdigit() else i
            if idx < len(rest):
                am = re.fullmatch(r'&?\s*\w+(?:\.\w+)*?\.(\w+)', rest[idx])
                if am and am.group(1) in nodisp:
                    print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: `{{}}` formats enum field `{rest[idx]}` without Display")
        for a in rest:
            am = re.fullmatch(r'(\w+)\s*=\s*&?\w+(?:\.\w+)*\.(\w+)', a)
            if am and am.group(2) in nodisp: print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: named arg `{a}` formats enum without Display")
    for m in re.finditer(r'\.(\w+)\.to_string\(\)', src):
        if m.group(1) in nodisp: print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: `.{m.group(1)}.to_string()` enum without Display")
