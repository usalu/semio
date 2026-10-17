"""📋 Compact per-file listing of a tsc output (skips TS2307/TS7016), one block per file, most diagnostics first."""
import collections, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt4_blocks import blocks

def main():
    src, out = sys.argv[1], sys.argv[2]
    skip = set(sys.argv[3].split(",")) if len(sys.argv) > 3 else {"TS2307", "TS7016"}
    items = [b for b in blocks(src) if b["code"] not in skip]
    by = collections.OrderedDict()
    for b in items:
        by.setdefault(b["file"], []).append(b)
    lines, cache = [], {}
    for f, rows in sorted(by.items(), key=lambda kv: (-len(kv[1]), kv[0])):
        text = cache.setdefault(f, open(f, encoding="utf8").read().split("\n") if os.path.exists(f) else [])
        lines.append(f"=== {len(rows)} {f.replace('🧰️framework/', 'FW/')}")
        for b in rows:
            line = text[b["line"] - 1] if b["line"] <= len(text) else ""
            c = b["col"] - 1
            t = b["text"]
            lines.append(f"{b['line']}:{b['col']} {t[t.index(': error ') + 8:].replace(chr(10), ' | ')[:260]}")
            lines.append("   >> " + line[max(0, c - 60): c + 90].strip())
    open(out, "w", encoding="utf8").write("\n".join(lines))
    print(len(items), len(by))

main()
