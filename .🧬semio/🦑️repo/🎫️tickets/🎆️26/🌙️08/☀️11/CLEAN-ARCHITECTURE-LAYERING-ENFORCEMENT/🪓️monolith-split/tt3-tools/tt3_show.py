"""🔎 Print the diagnostics of files whose path contains a needle, each with a source excerpt around the column."""
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from tt3_blocks import blocks

def main():
    path, needle = sys.argv[1], sys.argv[2]
    width = int(sys.argv[3]) if len(sys.argv) > 3 else 90
    skip = sys.argv[4] if len(sys.argv) > 4 else "NODE_ENV"
    cache = {}
    for b in blocks(path):
        if needle not in b["file"] or (skip and skip in b["text"]): continue
        lines = cache.setdefault(b["file"], open(b["file"], encoding="utf8").read().split("\n"))
        src = lines[b["line"] - 1] if b["line"] <= len(lines) else ""
        c = b["col"] - 1
        print(f'{b["line"]}:{b["col"]} {b["code"]} {b["text"][b["text"].index(":", b["text"].index("error")+8)+2:][:260]}'.replace("\n", "\n      "))
        print("      >>", src[max(0, c - width): c + width].strip())
main()
