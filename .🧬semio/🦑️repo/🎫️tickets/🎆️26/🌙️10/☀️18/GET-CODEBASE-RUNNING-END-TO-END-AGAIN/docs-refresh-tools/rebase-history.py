# 📚 Rebases the applied-text record of the docs tool onto the README as peers edited it live (drifted identifiers only).
import json, sys
C, RECORD, README = sys.argv[1], sys.argv[2], sys.argv[3]
rec = json.load(open(RECORD, encoding="utf8"))
hist = rec["history"]
cur = open(README, encoding="utf8").read()
changed = []
for ident, applied in hist.items():
    if cur.count(applied) == 1:
        continue
    if "\n" in applied:
        first = applied.split("\n")[0]
        last = [l for l in applied.split("\n") if l.strip()][-1]
        assert cur.count(first) == 1, (ident, "first line not unique", first[:60])
        start = cur.index(first)
        assert cur.count(last) == 1, (ident, "last line not unique", last[:60])
        end = cur.index(last, start) + len(last)
        new = cur[start:end]
    else:
        key = applied[:50]
        lines = [l for l in cur.split("\n") if l.startswith(key)]
        assert len(lines) == 1, (ident, "line prefix matches", len(lines))
        new = lines[0]
    assert cur.count(new) == 1, (ident, "rebased text not unique")
    hist[ident] = new
    changed.append((ident, len(applied), len(new)))
json.dump(rec, open(RECORD, "w", encoding="utf8"), ensure_ascii=False)
print("rebased", changed)
