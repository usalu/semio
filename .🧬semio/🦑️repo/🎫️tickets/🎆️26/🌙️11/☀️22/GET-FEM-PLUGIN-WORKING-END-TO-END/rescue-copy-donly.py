import os, shutil, sys
T = os.path.dirname(os.path.abspath(__file__))
G = T + r"\🗑️generated"
done = skipped = 0
skip_names = {"rerun.txt", "rerun3.txt"}
log = open(G + r"\rescue-copy-donly.log", "w", encoding="utf-8")
for line in open(G + r"\rescue-status-classes.tsv", encoding="utf-8").read().split("\n"):
    if not line.startswith("d-"):
        continue
    k, st, p = line.split("\t")
    o = os.path.join(r"C:\git\semio", p)
    d = os.path.join(r"C:\semio", p)
    if os.path.basename(p) in skip_names:
        log.write(f"SKIP-log\t{p}\n"); skipped += 1; continue
    if len(d) > 250:
        log.write(f"SKIP-pathlen\t{len(d)}\t{p}\n"); skipped += 1; continue
    if os.path.exists(d):
        log.write(f"SKIP-exists\t{p}\n"); skipped += 1; continue
    os.makedirs(os.path.dirname(d), exist_ok=True)
    shutil.copy2(o, d)
    log.write(f"COPIED\t{p}\n"); done += 1
print(done, skipped)
