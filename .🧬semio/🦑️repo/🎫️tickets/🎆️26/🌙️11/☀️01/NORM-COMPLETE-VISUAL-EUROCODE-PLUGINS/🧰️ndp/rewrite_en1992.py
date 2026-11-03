import sys, json, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from rewrite_lib import *
ids = json.load(open(pathlib.Path(__file__).parent / "en1992-ids.json"))
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992")
changed = 0
for f in ROOT.rglob("*.rs"):
    if f.name != "🦀️.rs" or "ndp-tables" in str(f): continue
    s = f.read_text(); o = s
    s = rewrite_calls(s, r'(?<![\w])(?:c\.)?ndp\(', ids)
    s = rewrite_calls(s, r'entry_of\(', ids, arg_index=6)
    if s != o:
        f.write_text(s); changed += 1; print(f.relative_to(ROOT).as_posix()[-70:])
print(changed)
