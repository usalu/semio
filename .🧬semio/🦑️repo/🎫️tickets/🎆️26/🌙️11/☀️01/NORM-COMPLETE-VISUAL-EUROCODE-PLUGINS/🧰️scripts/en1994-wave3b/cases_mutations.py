"""Installs the seven design-case mutation leaves into en1994 (cloned from the EN 1995 leaves), the family crate `mut-design-cases` with all workspace, lock, script and owner registration, and the root enum, KINDS, json, ts, binary tags and edit rules. Run once: /home/deck/semio/.venv/bin/python -I cases_mutations.py"""
import os, re, sys
os.environ["TH_FORCE"] = "1"
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/catalogue-bridge-v2")
import lib
for k in list(lib.LEAF_DIRS):
    del lib.LEAF_DIRS[k]
lib.LEAF_DIRS.update({"insert-case": "➕️insert-case", "remove-case": "➖️remove-case", "change-case-number": "🔢️change-case-number", "change-case-text": "🔤️change-case-text", "change-case-label-en": "🏷️change-case-label-en", "change-case-label-de": "🏷️change-case-label-de", "change-case-entry-id": "📇️change-case-entry-id"})
lib.KINDS[:] = list(lib.LEAF_DIRS)
lib.RECORD.clear(); lib.RECORD.update({k: "".join(w.capitalize() for w in k.split("-")) for k in lib.KINDS})
lib.MODULE.clear(); lib.MODULE.update({k: k.replace("-", "_") for k in lib.KINDS})
from lib import *
import leaves as L
A = Art("en1994")
CASES = "semio_s_artifact_norm_en1994_checks::case_catalogue::En1994Cases"
if os.path.exists(A.mut + "/" + LEAF_DIRS["insert-case"]):
    sys.exit("leaves exist already")
proto = A.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
top = max(int(x) for x in re.findall(r"tag=(\d+)", rd(proto)))
tags = {k: top + 1 + i for i, k in enumerate(KINDS)}
L.install_leaf_dirs(A, CASES, tags)
cname, consumers = L.family_crate(A, [A.crate + "checks"])
L.root_enum(A, cname)
ext = L.root_json(A)
L.root_ts(A, ext)
wr(proto, rd(proto).rstrip("\n") + "\n" + "".join("record %s tag=%d\nfield payload bytes\n" % (k, tags[k]) for k in KINDS))
p = A.mut + "/🧭️edit-rules/🦀️.rs"
s = rd(p)
assert "cases[]" not in s
SET = """        SetFieldRule { path: "cases[].labelEn", kind: "change-case-label-en", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },
        SetFieldRule { path: "cases[].labelDe", kind: "change-case-label-de", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },
        SetFieldRule { path: "cases[].entryId", kind: "change-case-entry-id", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },
        SetFieldRule { path: "cases[].numbers[].value", kind: "change-case-number", selectors: &[SelectorKey::Id("caseId"), SelectorKey::Id("key")], value: "value" },
        SetFieldRule { path: "cases[].texts[].value", kind: "change-case-text", selectors: &[SelectorKey::Id("caseId"), SelectorKey::Id("key")], value: "value" },
"""
i = s.index("    set_field: &[\n"); j = s.index("    ],\n", i); s = s[:j] + SET + s[j:]
i = s.index("    insert_item: &[\n"); j = s.index("    ],\n", i); s = s[:j] + '        InsertItemRule { path: "cases", kind: "insert-case", selectors: &[], index: "index", item: "case" },\n' + s[j:]
i = s.index("pub const ID_REMOVALS: &[IdRemoval] = &[\n"); j = s.index("\n];", i); s = s[:j] + '\n        IdRemoval { path: "cases", kind: "remove-case", selectors: &[], id: "id" },' + s[j:]
wr(p, s)
print(len(LOG), cname, tags, [os.path.relpath(c, A.root) for c in consumers])
