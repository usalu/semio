"""🧪 RI3 scan: `pub` items gated by a bare `#[cfg(test)]` (invisible to every other crate's tests) that another crate names."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from owners import *

ITEM = re.compile(r'^\s*pub(?:\([^)]*\))?\s+(?:unsafe\s+|async\s+|const\s+)*(?:fn|struct|enum|const|static|type|trait)\s+([A-Za-z_]\w*)')


def main():
    tbl, cf, idx = build()
    gated = collections.defaultdict(set)
    texts = {}
    for suffix, files in cf.items():
        for f in files:
            try:
                raw = read(f)
            except Exception:
                continue
            texts[(suffix, f)] = raw
            lines = raw.split("\n")
            for i, ln in enumerate(lines):
                if ln.strip() == "#[cfg(test)]":
                    j = i + 1
                    while j < len(lines) and (lines[j].strip().startswith("#[") or not lines[j].strip()):
                        j += 1
                    if j < len(lines):
                        m = ITEM.match(lines[j])
                        if m and len(m.group(1)) > 5:
                            gated[m.group(1)].add((suffix, f))
    names_by = collections.defaultdict(set)
    for n, defs in gated.items():
        names_by[n] = {s for s, _ in defs}
    out = []
    for (suffix, f), raw in texts.items():
        clean = blank(raw)
        for name, owners_ in names_by.items():
            if suffix in owners_ or name not in clean:
                continue
            for m in re.finditer(r'(?<![\w$])' + re.escape(name) + r'(?!\w)', clean):
                ln = clean.count("\n", 0, m.start()) + 1
                out.append((name, sorted(owners_), suffix, f.replace(REPO + "/", "")[-72:], ln))
                break
    for r in sorted(out):
        print(r)


main()
