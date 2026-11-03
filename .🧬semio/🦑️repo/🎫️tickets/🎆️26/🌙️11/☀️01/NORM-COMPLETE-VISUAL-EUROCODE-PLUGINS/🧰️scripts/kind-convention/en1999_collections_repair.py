"""🩹 Re-adds the 14 collection insert/remove kinds to the en1999 root enum, KINDS and binary protocol after a concurrent owner write dropped them (idempotent)."""
import os, re, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "leaf-kinds"))
from common import *

A = NORM + "/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any"
NAMES = ["material", "section", "connection", "fire-scenario", "fatigue-detail", "cold-formed-sheet", "shell"]
KINDS = [v + "-" + n for n in NAMES for v in ("insert", "remove")]
p = A + "/🧬️schema/🧬️mutations/🦀️.rs"
s = rd(p)
todo = [k for k in KINDS if '    "%s",\n' % k not in s]
if todo:
    uses = "".join("use semio_s_artifact_norm_en1999_mut_collection_rows::%s;\n" % k.replace("-", "_") for k in todo)
    i = s.index("use semio_s_artifact_norm_en1999_mut_detail_rows::insert_weld_run;\n")
    s = s[:i] + uses + s[i:]
    i = s.index("\n}\n\npub const KINDS")
    s = s[:i] + "\n" + "".join("    %s(%s::%s),\n" % (pascal(k), k.replace("-", "_"), pascal(k)) for k in todo).rstrip("\n") + s[i:]
    j = s.index("\n];", s.index("pub const KINDS"))
    s = s[:j] + "\n" + "".join('    "%s",\n' % k for k in todo).rstrip("\n") + s[j:]
    open(p, "w", encoding="utf-8").write(s)
kinds = re.findall(r'^    "([a-z0-9-]+)",$', s, re.M)
q = A + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
t = rd(q)
open(q, "w", encoding="utf-8").write(t[: t.index("record ")] + "".join("record %s tag=%d\nfield payload bytes\n" % (k, i) for i, k in enumerate(kinds)))
print("added", len(todo), "kinds total", len(kinds))
