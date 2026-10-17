"""🔓 rw2t: widens the definition heads (items) the moved laws name from private or pub(crate)/pub(super) to pub, one anchored line edit each.
usage: widen.py <list.json> [--apply]    (list: [[name, absolute file], ...])"""
import sys, json, re
from ed import *
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rtb-tools")
import items as itm

HEAD = re.compile(r'^(\s*)(pub\s*\([^)]*\)\s+|pub\s+)?((?:(?:const|async|unsafe|extern\s+"[^"]*")\s+)*(?:fn|struct|enum|const|static|type|trait)\b)')

def plan(pairs):
    byfile = {}
    for name, f in pairs:
        byfile.setdefault(f, set()).add(name)
    out = []
    for f, names in byfile.items():
        text = read(f)
        lines = text.split("\n")
        tab = itm.table(text)
        for name in sorted(names):
            hits = [it for it in tab if it["name"] == name and it["kind"] in ("fn", "struct", "enum", "const", "static", "type", "trait")]
            if len(hits) != 1:
                print("SKIP", name, f[-60:], len(hits))
                continue
            it = hits[0]
            ln = lines[it["head"] - 1]
            m = HEAD.match(ln)
            if not m:
                print("NOHEAD", name, ln[:80])
                continue
            if m.group(2) and m.group(2).strip() == "pub":
                continue
            new = ln[:m.start(2) if m.group(2) else m.start(3)] + "pub " + ln[m.end(2) if m.group(2) else m.start(3):]
            out.append((f, it["head"], ln, new, name))
    return out

if __name__ == "__main__":
    pairs = json.load(open(sys.argv[1]))
    pl = plan(pairs)
    for f, k, old, new, name in pl:
        print(f"{f[-70:]}:{k}  {old.strip()[:70]}  =>  {new.strip()[:70]}")
    if "--apply" in sys.argv:
        byfile = {}
        for f, k, old, new, name in pl:
            byfile.setdefault(f, []).append((k, old, new))
        for f, rows in byfile.items():
            text = read(f)
            lines = text.split("\n")
            for k, old, new in rows:
                assert lines[k - 1] == old, (f, k)
                lines[k - 1] = new
            backup(f)
            assert read(f) == text
            with open(f, "w", encoding="utf8") as h:
                h.write("\n".join(lines))
            note(f)
            print("widened", len(rows), f[-60:])
