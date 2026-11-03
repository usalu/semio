import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import check_style as cs

src = (cs.REPO / sys.argv[1]).read_text(encoding="utf-8")
for kind, ln, depth, text in cs.ts_scan(src):
    print(ln, depth, kind, text[:50].replace("\n", " "))
