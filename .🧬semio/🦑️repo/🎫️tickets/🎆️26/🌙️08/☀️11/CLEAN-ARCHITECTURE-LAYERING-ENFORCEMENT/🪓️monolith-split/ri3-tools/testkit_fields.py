"""🧪 RI3 scan: struct fields (declarations) and initialisers (`name: expr,`) carrying a `#[cfg(...testkit...)]` attribute, grouped by crate, to find a field defined in one crate and initialised under another crate's feature."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from owners import *

ATTR = re.compile(r'#\[cfg\(([^\n]*)\)\]\s*$')
FIELD = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?([a-z_][a-z0-9_]*)\s*:(?!:)\s*(.*)$')


def main():
    tbl, cf, idx = build()
    decl = collections.defaultdict(list)
    init = collections.defaultdict(list)
    for suffix, files in cf.items():
        for f in files:
            try:
                lines = blank(read(f)).split("\n")
                raw = read(f).split("\n")
            except Exception:
                continue
            for i, ln in enumerate(lines):
                m = ATTR.search(ln.strip()) if ln.strip().startswith("#[cfg(") else None
                if not m or "testkit" not in m.group(1):
                    continue
                j = i + 1
                while j < len(lines) and (lines[j].strip().startswith("#[") or not lines[j].strip()):
                    j += 1
                if j >= len(lines):
                    continue
                fm = FIELD.match(lines[j])
                if not fm:
                    continue
                name = fm.group(1)
                if name in ("self", "pub", "let", "use", "fn", "mod", "const", "static", "type"):
                    continue
                is_decl = bool(re.match(r'^\s*pub', lines[j])) or fm.group(2).rstrip(",").strip().startswith(("Option<", "Vec<", "bool", "u", "i", "f", "String", "HashMap", "BTreeMap"))
                (decl if re.match(r'^\s*(pub|[a-z_]+\s*:\s*[A-Z][A-Za-z<>]*,?\s*$)', lines[j]) and not re.search(r':\s*(None|Vec::new|false|true|0|String::new|HashMap::new|BTreeMap::new|Default::default)', lines[j]) else init)[name].append((suffix, f.replace(REPO + "/", "")[-60:], j + 1, m.group(1)[:70]))
    for name in sorted(set(decl) | set(init)):
        ds = {s for s, *_ in decl.get(name, [])}
        ins = {s for s, *_ in init.get(name, [])}
        cross = ins - ds
        if cross and ds:
            print("CROSS", name, "declared in", sorted(ds), "initialised in", sorted(ins))
            for r in decl[name] + init[name]:
                print("    ", r)
        elif cross and not ds:
            print("INIT-ONLY", name, sorted(ins))
main()
