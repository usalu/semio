"""🔎 Duplicate definitions: items twice in one module scope (E0428), repeated struct fields/enum variants, same inherent method twice per type in a crate (E0592), same name imported twice in one scope (E0252)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import scan_resolve as S
from rs import *
S.load_crates()
code = sys.argv[1]
adir = ART + "/" + CODE[code]
ITEM = re.compile(r'(?:^|(?<=[;{}\]\s]))(?:pub(?:\([^)]*\))?\s+)?(?:async\s+|unsafe\s+|const\s+)*(fn|struct|enum|trait|type|union|const|static|mod)\s+(\w+)')
def toplevel(text):
    """Return text with all nested brace groups blanked (depth-0 view)."""
    out, depth = [], 0
    for ch in text:
        if ch == "{": depth += 1; out.append(" " if depth > 1 else ch)
        elif ch == "}": depth -= 1; out.append(" " if depth >= 1 else ch)
        else: out.append(ch if depth == 0 else (" " if ch != "\n" else "\n"))
    return "".join(out)
for lib, c in sorted(S.CRATES.items()):
    if not c["dir"].startswith(adir): continue
    files, parent = S.tree(c["root"])
    inherent = {}
    for f, src in files.items():
        for si, (own, par, full) in enumerate(S.scopes(f, files)):
            top = toplevel(own)
            seen = {}
            for m in ITEM.finditer(top):
                kind, name = m.group(1), m.group(2)
                if "macro_rules" in top[max(0, m.start() - 12):m.start()]: continue
                pre = own[max(0, m.start() - 160):m.start()]
                if re.search(r'#\[cfg\(', pre.split(";")[-1].split("}")[-1]): continue
                ns = "v" if kind in ("fn", "const", "static") else "t"
                key = (ns, name)
                if kind == "mod" and (name in seen): pass
                if key in seen:
                    print(f"{rel(f)}:{own.count(chr(10), 0, m.start()) + 1}: duplicate {kind} `{name}` in one module scope ({c['name']})")
                seen[key] = kind
            imported = {}
            for um in S.USEP.finditer(top):
                names, _ = S.use_names(um.group(2))
                pre = own[max(0, um.start() - 60):um.start() + 1]
                if re.search(r'#\[cfg\([^\]]*\)\]\s*$', own[max(0, um.start() - 80):um.start() + 1].replace("\n", " ")): continue
                for n in names:
                    if n in imported and n not in ("self",):
                        print(f"{rel(f)}:{own.count(chr(10), 0, um.start()) + 1}: `{n}` imported twice in one scope ({c['name']})")
                    imported[n] = 1
                    if (("t", n) in seen or ("v", n) in seen) and n[0].isupper() and False:
                        pass
        for m in re.finditer(r'\bstruct\s+(\w+)\s*(?:<[^{;(]*>)?\s*\{', src):
            e = match_close(src, m.end() - 1)
            if e < 0: continue
            names = [re.sub(r'.*?(\w+)\s*:.*', r'\1', re.sub(r'#\[[^\]]*\]', '', p).strip().replace("pub ", "").replace("pub(crate) ", "").replace("pub(super) ", ""), flags=re.S) for p in split_top(src[m.end():e])]
            names = [n for n in names if re.fullmatch(r'\w+', n)]
            for n in {x for x in names if names.count(x) > 1}:
                print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: struct {m.group(1)} repeats field `{n}`")
        for m in re.finditer(r'\benum\s+(\w+)\s*(?:<[^{]*>)?\s*\{', src):
            e = match_close(src, m.end() - 1)
            if e < 0: continue
            vs = variants_of(src[m.end():e])
            for n in {x for x in vs if vs.count(x) > 1}:
                print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: enum {m.group(1)} repeats variant `{n}`")
        for m in re.finditer(r'\bimpl\b(?:\s*<[^{;]*?>)?\s+([A-Z]\w*)(?:\s*<[^{;]*?>)?\s*(?:where[^{;]*)?\{', src):
            e = match_close(src, m.end() - 1)
            if e < 0: continue
            top = toplevel(src[m.end() - 1:e + 1])
            for fm in re.finditer(r'\bfn\s+(\w+)', top):
                pre = src[max(0, m.end() + fm.start() - 100):m.end() + fm.start()]
                if re.search(r'#\[cfg\(', pre.split(";")[-1].split("}")[-1]): continue
                inherent.setdefault((m.group(1), fm.group(1)), []).append((f, src.count(chr(10), 0, m.end() + fm.start()) + 1))
    for (t, n), locs in sorted(inherent.items()):
        if len(locs) > 1:
            print(f"{rel(locs[0][0])}:{locs[0][1]}: method `{t}::{n}` defined {len(locs)}x in inherent impls: " + ", ".join(f"{rel(a).split('/')[-2]}:{b}" for a, b in locs[:4]))
