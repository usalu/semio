"""🔗 c4p29 consumer re-points (non-split files that name or read a split file). Each edit is an exact anchored change on text re-read immediately before the write;
a peer-changed anchor aborts that edit.
1. The 1.7 base schema registers the Rust text of the `diff` facet with `include_str!("🔺️diff/🦀️.rs")`: it becomes `concat!` over the root and its six children, the pattern the other split diff facets use.
2. The lib-wiring ratchet pin of the pdf crate lib root is removed: the root now complies with the 150-line budget.
usage: c4pconsumers.py plan|apply|verify|stage <dir>    journal: 🗑️generated/c4p29-consumers.json"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS

L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
SCHEMA = f"{PB}/🧬️schema/🦀️.rs"
RATCHET = f"{L}/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json"
PIN = f"{PDF}/🦀️.rs"
JP = GEN + "/c4p29-consumers.json"
OLD_LEAF = '            rust: include_str!("🔺️diff/🦀️.rs"),\n'
parts = ['include_str!("🔺️diff/🦀️.rs")'] + [f'include_str!("🔺️diff/{c["folder"]}/🦀️.rs")' for c in PLANS["diff"]["children"]]
NEW_LEAF = "            rust: concat!(" + ", ".join(parts) + "),\n"
EDITS = [dict(file=SCHEMA, old=OLD_LEAF, new=NEW_LEAF), dict(file=RATCHET, ratchet=PIN)]


def apply_edit(e, base=ROOT):
    p = base + "/" + e["file"]
    text = read(p)
    if e.get("ratchet"):
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
elif cmd == "stage":
    dest = sys.argv[2]
    for e in EDITS:
        if e.get("ratchet"):
            continue
        p, a, b = apply_edit(e)
        write(dest + "/" + e["file"], b)
        print("staged", e["file"][-60:])
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
        ok = (('"' + e["ratchet"] + '":') not in t) if e.get("ratchet") else (e["new"] in t and e["old"] not in t)
        print("OK" if ok else "MISSING", key[:70])
