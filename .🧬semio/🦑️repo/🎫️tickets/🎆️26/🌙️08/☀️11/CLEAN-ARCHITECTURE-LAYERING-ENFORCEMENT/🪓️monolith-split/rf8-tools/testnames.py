"""Test-name multiset proof: the `#[test]`/`async_test` fn names of the original target file versus the union of the live target file and its installed children."""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *

RX = re.compile(r"#\[\s*(?:[\w:]*::)?(?:test|async_test|tokio::test)\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)")


def names(text):
    return sorted(m.group(1) for m in RX.finditer(text))


ok = True
for name, t in TARGETS.items():
    mk = f"{W}/{name}/installed.json"
    if not os.path.exists(mk):
        continue
    a = names(read(snap_dir(name) + "/" + t["file"]))
    files = json.loads(read(mk))["files"]
    b = names("\n".join(read(ROOT + "/" + f) for f in files))
    same = a == b
    ok &= same
    print(f"{name:9} test fns before={len(a):3} after={len(b):3} identical={same}")
print("TESTNAMES", "OK" if ok else "DIFFERENT")
