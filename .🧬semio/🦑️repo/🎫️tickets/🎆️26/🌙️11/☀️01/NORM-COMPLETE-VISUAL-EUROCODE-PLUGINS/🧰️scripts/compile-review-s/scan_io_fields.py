"""🔎 Every Vec field of the artifact Snapshot must be written (self.<f>) and read/initialised in the sqlite snapshot io, and appear in the record/text/binary io."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, _, files = index_types(adir)
snap = max(structs.get("En%sSnapshot" % code, [(None, {})]), key=lambda d: len(d[1]))[1]
vec = [f for f, t in snap.items() if t.startswith("Vec<")]
io = adir + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io"
text = {f: s for f, s in files.items() if f.startswith(io + "/🪶️sqlite") and "/🧪️tests/" not in f}
blob = "\n".join(text.values())
for fld in sorted(snap):
    n_write = len(re.findall(r'\bself\.%s\b' % fld, blob))
    n_read = len(re.findall(r'\b%s\s*[:,]|\bsnapshot\.%s\b|\bresult\.%s\b' % (fld, fld, fld), blob))
    if n_write == 0 or n_read == 0:
        print(f"{code}: sqlite io: field `{fld}` ({snap[fld][:40]}) write-mentions {n_write} read-mentions {n_read}")
