"""🔬️ Unmasked row diff of the saved oracle error files (tc4-oracle-base.err vs tc4-oracle-cur-*.err): rows are (error line, primary span file without line numbers). usage: oracle-diff.py"""
import collections, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.realpath(HERE + "/../🗑️generated") + "/"
REL = "✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/"
def rows(path, dst):
    out = collections.Counter(); lines = open(path, encoding="utf8").read().splitlines()
    for i, l in enumerate(lines):
        if l.startswith("error") and "aborting due to" not in l:
            span = ""
            for j in range(i + 1, min(i + 4, len(lines))):
                m = re.match(r"\s*--> (.*?):\d+:\d+", lines[j])
                if m: span = os.path.normpath(m.group(1)).replace(dst + "/", "").replace(REL, ""); break
            out[(l, span)] += 1
    return out
base = rows(GEN + "tc4-oracle-base.err", "/Users/ueli/.cache/tc4/sb/base")
cur = collections.Counter()
roots = ["🔭️viewer-laws", "🪟️editor-mode-laws", "🧭️editor-structure-laws", "🎮️editor-command-laws", "🧮️editor-unit-laws", "🕹️editor-surface-laws"]
for r in roots:
    for k, v in rows(GEN + f"tc4-oracle-cur-{r}.err", "/Users/ueli/.cache/tc4/sb/cur").items(): cur[k] = max(cur[k], v)
print("base", len(base), "cur", len(cur))
new = sorted(k for k in cur if k not in base)
base_messages = {l for l, _ in base}
moved_files = ("📥️editor-imports/🦀️.rs", "🧰️shared-helpers/🦀️.rs", "🦀️.rs")
relocated = [k for k in new if k[0] in base_messages and (k[1].endswith(moved_files) or re.search(r"-laws/🦀️\.rs$", k[1]))]
rest = [k for k in new if k not in relocated]
print("new total", len(new), "relocated into imports/root files", len(relocated), "other", len(rest))
for l, s in rest: print("  NEW", l[:170], "|", s[-70:])
