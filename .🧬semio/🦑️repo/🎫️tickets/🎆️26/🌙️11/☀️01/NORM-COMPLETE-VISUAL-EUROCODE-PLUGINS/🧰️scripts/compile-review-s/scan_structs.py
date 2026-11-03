"""🔎 Struct literals of artifact types that miss fields (E0063) or name unknown fields (E0560); catches new collections absent from `Snapshot { .. }` initializers."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, enums, files = index_types(adir)
out = []
IMPL = re.compile(r'\bimpl\b(?:\s*<[^{;]*?>)?\s*(?:[\w:<>, ]+?\s+for\s+)?([A-Z]\w*)(?:\s*<[^{;]*?>)?\s*(?:where[^{;]*)?\{')
for f, src in files.items():
    if "/node_modules/" in f: continue
    impls = []
    for im in IMPL.finditer(src):
        ie = match_close(src, im.end() - 1)
        if ie > 0: impls.append((im.end(), ie, im.group(1)))
    for m in re.finditer(r'(?<![\w:$])([A-Z]\w*)\s*\{', src):
        name = m.group(1)
        if name == "Self":
            hit = [n for a, b, n in impls if a <= m.start() <= b]
            name = hit[-1] if hit else ("En%sSnapshot" % code if "sqlite" in f and "snapshot" in f.lower() else name)
        if name not in structs: continue
        pre = src[max(0, m.start() - 24):m.start()]
        if re.search(r'(struct|enum|impl|trait|for|mod|dyn|where|->|\bis|\blet|\bmatch|\bif|\bwhile|\|)\s*$', pre): continue
        e = match_close(src, m.end() - 1)
        if e < 0: continue
        post = src[e + 1:e + 6].lstrip()
        if post.startswith(("=>", "|")) or re.match(r'=(?!=)', post) or post.startswith((":", "if ")): continue
        body = src[m.end():e]
        if "$" in body or re.search(r'(^|[,{\s])\.\.', body) and not re.search(r'\.\.\s*(Default|[\w:]+::default|\w+)', body): continue
        base = bool(re.search(r'(^|[,\s])\.\.\s*\S', body))
        given = set()
        for part in split_top(body, angles=False):
            part = part.strip()
            if not part or part.startswith(".."): continue
            fm = re.match(r'(\w+)\s*(?::|$)', part)
            if fm: given.add(fm.group(1))
        cands = [d for _, d in structs[name]]
        best = min(cands, key=lambda d: (len(given - set(d)), len(set(d) - given)))
        extra = given - set(best)
        missing = set(best) - given
        line = src.count("\n", 0, m.start()) + 1
        if extra: out.append((f, line, f"literal {name}: unknown field(s) {sorted(extra)}"))
        elif missing and not base and len(given) >= 1: out.append((f, line, f"literal {name}: missing field(s) {sorted(missing)}"))
for f, l, msg in sorted(set(out)):
    print(f"{rel(f)}:{l}: {msg}")
print(f"# {len(set(out))} findings", file=sys.stderr)
