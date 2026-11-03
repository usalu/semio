"""🔎 Lowercase identifiers used as values inside a fn body that are bound nowhere in the fn (params, let, for, closure, match/if-let patterns) and defined/imported nowhere in scope (E0425)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import scan_resolve as S
from rs import *

S.load_crates()
code = sys.argv[1]
adir = ART + "/" + CODE[code]
KW = set("u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool char str as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self static struct super trait true type unsafe use where while async await dyn box yield try union macro_rules".split())
BUILTIN = set("drop panic assert assert_eq vec format println print eprintln write writeln matches unreachable todo unimplemented dbg cfg env include_str include concat stringify line file column debug_assert debug_assert_eq assert_ne some none ok err".split())
IDENT = re.compile(r'[a-z_][A-Za-z0-9_]*')
FNSIG = re.compile(r'\bfn\s+(\w+)\s*(<[^{;()]*>)?\s*\(')
def idents(t): return set(IDENT.findall(t))
for lib, c in sorted(S.CRATES.items()):
    if not c["dir"].startswith(adir): continue
    files, parent = S.tree(c["root"])
    for f, src in files.items():
        scs = S.scopes(f, files)
        vis_all = set()
        for si in range(len(scs)):
            v, o = S.vis_scope(f, si, files, parent, lib)
            vis_all |= v
            if o: vis_all |= S.crate_names_all()
        for m in FNSIG.finditer(src):
            pe = match_close(src, m.end() - 1)
            if pe < 0: continue
            rest = src[pe + 1:pe + 400]
            bm = re.match(r'[^{;]*?(\{)', rest, re.S)
            if not bm or ";" in rest[:bm.start(1)]: continue
            bs = pe + 1 + bm.start(1)
            be = match_close(src, bs)
            if be < 0: continue
            sig, body = src[m.end():pe], src[bs + 1:be]
            if "$" in body: continue
            bound = idents(re.sub(r':\s*[^,]*', '', sig)) | {"self"}
            for lm in re.finditer(r'\blet\s+([^=;]*?)(?<![=!<>])=(?!=)', body): bound |= idents(re.sub(r':[^=]*$', '', lm.group(1)))
            for lm in re.finditer(r'\blet\s+([^=;]*?);', body): bound |= idents(lm.group(1))
            for lm in re.finditer(r'\bfor\s*(.*?)\s*\bin\b', body): bound |= idents(lm.group(1))
            for lm in re.finditer(r'\|([^|]*)\|', body): bound |= idents(re.sub(r':[^,|]*', '', lm.group(1)))
            for lm in re.finditer(r'(?:^|[,{};(\s])([^=>{};]*?)=>', body): bound |= idents(re.sub(r'\bif\b.*$', '', lm.group(1)))
            for lm in re.finditer(r'\b(?:if|while)\s+let\s+(.*?)\s*=(?!=)', body): bound |= idents(lm.group(1))
            for lm in re.finditer(r'\b(?:fn|const|static)\s+(\w+)', body): bound.add(lm.group(1))
            for lm in re.finditer(r'\bfn\s+\w+\s*\(([^)]*)\)', body): bound |= idents(re.sub(r':\s*[^,]*', '', lm.group(1)))
            seen = set()
            for um in re.finditer(r'(?<![\w.:$\'#])([a-z_]\w*)\b(?!\s*(?:::|!|:(?!:)|\())', body):
                n = um.group(1)
                if n in KW or n in BUILTIN or n in bound or n in vis_all or n in seen or n.startswith("_") and len(n) == 1: continue
                if re.match(r'\d', n): continue
                seen.add(n)
                line = src.count(chr(10), 0, bs + 1 + um.start()) + 1
                print(f"{rel(f)}:{line}: in fn {m.group(1)}: unbound value `{n}`")
