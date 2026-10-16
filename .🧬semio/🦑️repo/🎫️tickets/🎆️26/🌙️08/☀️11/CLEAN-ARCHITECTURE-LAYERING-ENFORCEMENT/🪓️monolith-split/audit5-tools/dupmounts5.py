"""🔗️ Read-only scan: every .rs file that is the target of a #[path] mount from more than one mounting file (source trees only)."""
import os, re, sys, collections
ROOT = "/Users/ueli/Documents/semio"
GEN = sys.argv[1]
PATH_RX = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
rows = []
for src in ["audit5r-path-tracked.txt", "audit5r-path-untracked.txt"]:
    for line in open(os.path.join(GEN, src), encoding="utf-8", errors="replace"):
        m = re.match(r"^(.*?\.rs):(\d+):(.*)$", line.rstrip("\n"))
        if not m:
            continue
        file, lineno, content = m.group(1), int(m.group(2)), m.group(3)
        if file.startswith(".🧬semio/"):
            continue
        pm = PATH_RX.search(content)
        if not pm:
            continue
        target = os.path.normpath(os.path.join(os.path.dirname(os.path.join(ROOT, file)), pm.group(1)))
        rows.append((file, lineno, pm.group(1), target))
by_target = collections.defaultdict(list)
for file, lineno, lit, target in rows:
    by_target[target].append((file, lineno, lit))
print("source-tree #[path] mounts:", len(rows), "distinct targets:", len(by_target))
missing = [t for t in by_target if not os.path.isfile(t)]
print("targets not on disk:", len(missing))
for t in missing[:30]:
    print("  MISSING", os.path.relpath(t, ROOT), by_target[t][0][0], by_target[t][0][1])
multi_file = []
same_file = []
for t, ms in by_target.items():
    files = set(f for f, _, _ in ms)
    if len(files) > 1:
        multi_file.append((t, ms))
    elif len(ms) > 1:
        same_file.append((t, ms))
print("targets mounted from >1 mounting file:", len(multi_file))
print("targets mounted >1 times from one file:", len(same_file))
with open(os.path.join(GEN, "audit5r-dupmounts.tsv"), "w", encoding="utf-8") as f:
    for t, ms in sorted(multi_file):
        for file, lineno, lit in ms:
            f.write("MULTIFILE\t" + os.path.relpath(t, ROOT) + "\t" + file + ":" + str(lineno) + "\t" + lit + "\n")
    for t, ms in sorted(same_file):
        for file, lineno, lit in ms:
            f.write("SAMEFILE\t" + os.path.relpath(t, ROOT) + "\t" + file + ":" + str(lineno) + "\t" + lit + "\n")
for t, ms in sorted(multi_file)[:60]:
    print("MULTI", os.path.relpath(t, ROOT))
    for file, lineno, lit in ms:
        print("     <-", file + ":" + str(lineno), lit)
