import glob, json, re, sys
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
S = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
slug, pat = sys.argv[1], re.compile(sys.argv[2])
text = open(glob.glob(str(S / "*" / slug / "🔣️.json"))[0], encoding="utf8").read()
for n, line in enumerate(text.splitlines(), 1):
    if pat.search(line):
        print(n, line.strip()[:int(sys.argv[3]) if len(sys.argv) > 3 else 200])
