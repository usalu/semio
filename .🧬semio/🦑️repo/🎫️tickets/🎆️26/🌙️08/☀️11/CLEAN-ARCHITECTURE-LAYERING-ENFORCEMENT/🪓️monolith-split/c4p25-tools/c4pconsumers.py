"""🔗 c4p25 consumer re-points (non-Rust files that name or read the split energy editor root). Each edit is an exact anchored one-line change on text re-read immediately before the write;
a peer-changed anchor aborts that edit. usage: c4pconsumers.py plan|apply|verify    journal: 🗑️generated/c4p25-consumers.json"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *

L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
EDITOR = f"{EA}/✏️editor/🦀️.rs"
CHILD = f"{EA}/✏️editor/🖥️editor-impl/🦀️.rs"
JP = GEN + "/c4p25-consumers.json"
EDITS = [
    dict(file=f"{L}/🔍️discovery/📖️source-access/🟦️.ts",
         old='  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",\n',
         new='  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",\n  "' + EDITOR + '",\n'),
    dict(file="🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json",
         old='      "source": "' + EDITOR + '",\n',
         new='      "source": "' + CHILD + '",\n'),
]
for key in (f"{EN}/🌰️kernel/🦀️.rs", f"{EN}/🔋️model/🦀️.rs", f"{EN}/🧠️precompute/🦀️.rs", f"{EN}/🧪️sim/📡️wire/🦀️.rs", EDITOR):
    EDITS.append(dict(file=f"{L}/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json", old=None, ratchet=key))


def apply_edit(e):
    p = ROOT + "/" + e["file"]
    text = read(p)
    if e.get("ratchet"):
        import re
        rx = re.compile(r'^    "' + re.escape(e["ratchet"]) + r'": \d+,\n', re.M)
        hits = rx.findall(text)
        assert len(hits) == 1, (e["ratchet"], len(hits))
        new = rx.sub("", text, count=1)
    else:
        assert text.count(e["old"]) == 1, (e["file"], text.count(e["old"]))
        new = text.replace(e["old"], e["new"], 1)
    return p, text, new


cmd = sys.argv[1] if len(sys.argv) > 1 else "plan"
journal = json.load(open(JP, encoding="utf-8")) if os.path.exists(JP) else {}
if cmd == "plan":
    for e in EDITS:
        p, a, b = apply_edit(e)
        print(os.path.basename(os.path.dirname(p)), len(a), "->", len(b))
elif cmd == "apply":
    for i, e in enumerate(EDITS):
        key = f"{i}:{e.get('ratchet') or e['old'][:40]}"
        if key in journal:
            print("already applied", key[:60])
            continue
        p, a, b = apply_edit(e)
        assert read(p) == a
        write(p, b)
        journal[key] = dict(file=e["file"], base_sha=sha(a), new_sha=sha(b))
        json.dump(journal, open(JP, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print("applied", key[:70])
elif cmd == "verify":
    for i, e in enumerate(EDITS):
        key = f"{i}:{e.get('ratchet') or e['old'][:40]}"
        t = read(ROOT + "/" + e["file"])
        if e.get("ratchet"):
            ok = ('"' + e["ratchet"] + '":') not in t
        else:
            ok = e["new"] in t and e["old"] not in t or (e["new"] in t)
        print("OK" if ok else "MISSING", key[:70])
