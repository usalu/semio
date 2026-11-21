"""🧾 Reconstructs an exact reviewed Cargo lock from the live and tracked package rows."""

import hashlib
import itertools
import json
from pathlib import Path
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = Path.cwd()
generated = ticket / "🗑️generated"
lock = Path("🌎️hub/Cargo.lock")
expected = json.loads((generated / "hub-assembly/reviewed-hub-assembly.json").read_text())[str(lock)]["postimage"]
before = subprocess.check_output(["repo","tools","exec","git","--light","--","show","HEAD:"+str(lock)])
live = lock.read_bytes()
assert before.split(b"[[package]]",1)[0] == live.split(b"[[package]]",1)[0]

def package_rows(body):
    pieces = body.split(b"[[package]]")
    rows = {}
    for piece in pieces[1:]:
        key = tuple(x.decode() for x in re.findall(rb'^(?:name|version|source) = "([^"]+)"$',piece,re.M))
        assert len(key) in {2,3} and key not in rows
        rows[key] = b"[[package]]" + piece
    return pieces[0], rows

header, a = package_rows(before)
_, b = package_rows(live)
dictionary = sorted(a.keys() | b.keys(), key=lambda key: (key[0], tuple(int(n) if n.isdigit() else n for n in re.split(r'[.\-+]',key[1])), key[2:]))
for original, rows in [(before,a), (live,b)]:
    assert header+b"".join(rows[key] for key in dictionary if key in rows) == original
deltas = sorted(key for key in dictionary if a.get(key)!=b.get(key))
groups = [header]
for key in dictionary:
    if key in deltas:
        groups += [key, b""]
    else:
        groups[-1] += b[key]
assert isinstance(groups[0],bytes)
for count in range(min(len(deltas),int(sys.argv[1]))+1):
    for reverted in itertools.combinations(deltas,count):
        prefix = set(reverted)
        candidate = b"".join((a if item in prefix else b).get(item,b"") if isinstance(item,tuple) else item for item in groups)
        if hashlib.sha256(candidate).hexdigest() == expected:
            assert hashlib.sha256(live).hexdigest() == hashlib.sha256(lock.read_bytes()).hexdigest()
            (generated / "hub-assembly/captured-hub-Cargo.lock").write_bytes(candidate)
            print("[DEBUG] Reverted exact captured lock delta:",reverted)
            break
    else:
        continue
    break
else:
    print("[DEBUG] No qualifying exact SHA reconstruction within",sys.argv[1],"changed package rows of",len(deltas))
    raise SystemExit(1)
