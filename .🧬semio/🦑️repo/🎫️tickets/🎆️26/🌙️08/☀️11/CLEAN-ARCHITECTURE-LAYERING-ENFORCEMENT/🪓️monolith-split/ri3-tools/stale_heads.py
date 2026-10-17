"""🕵️ RI3 stale-head scanner: lists lower-case path heads (`head::`) in crate files that resolve to no module, import, macro or dependency name of the owning crate."""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from owners import *

PRIM = set("u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 str char bool std core alloc crate self super vec".split())
HEAD = re.compile(r'(?<![\w$:.])([a-z_][a-z0-9_]*)::(?=[A-Za-z_{(<*])')


def manifest_names(folder):
    mf = os.path.join(T, folder, "📦️packages/🦀️rust/Cargo.toml")
    text = read(mf)
    names = set()
    for m in re.finditer(r'(?m)^([A-Za-z0-9_\-]+)\s*=', text):
        names.add(m.group(1).replace("-", "_"))
    for m in re.finditer(r'package\s*=\s*"([^"]+)"', text):
        names.add(m.group(1).replace("-", "_"))
    return names


def main():
    tbl, cf, idx = build()
    out = []
    for suffix, files in cf.items():
        folder = tbl[suffix]
        known = set(PRIM) | manifest_names(folder)
        texts = {}
        for f in files:
            try:
                t = blank(read(f))
            except Exception:
                continue
            texts[f] = t
            for m in re.finditer(r'\bmod\s+([a-z_]\w*)', t):
                known.add(m.group(1))
            for m in re.finditer(r'\bextern\s+crate\s+\w+\s+as\s+(\w+)', t):
                known.add(m.group(1))
            for m in re.finditer(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);', t):
                body = m.group(1)
                for a in re.finditer(r'\bas\s+(\w+)', body):
                    known.add(a.group(1))
                for a in re.finditer(r'(\w+)\s*(?:,|\}|$)', body):
                    known.add(a.group(1))
                for a in re.finditer(r'(\w+)::\{', body):
                    pass
            for m in re.finditer(r'\b(?:fn|let|const|static|struct|enum|type|trait)\s+(?:mut\s+)?([a-z_]\w*)', t):
                known.add(m.group(1))
            for m in re.finditer(r'macro_rules!\s*(\w+)', t):
                known.add(m.group(1))
        for f, t in texts.items():
            for m in HEAD.finditer(t):
                h = m.group(1)
                if h in known:
                    continue
                ln = t.count("\n", 0, m.start()) + 1
                out.append((suffix, h, f.replace(REPO + "/", ""), ln))
    agg = collections.defaultdict(list)
    for s, h, f, ln in out:
        agg[(s, h)].append((f, ln))
    for (s, h), lst in sorted(agg.items()):
        print(f"{s} :: {h}  x{len(lst)}  e.g. {lst[0][0][-70:]}:{lst[0][1]}")
    json.dump([[s, h, f, ln] for s, h, f, ln in out], open(GEN + "/ri3-stale-heads.json", "w"), ensure_ascii=False)


main()
