# Ticket input script: removes idle resource-lease databases (no holder, no waiter, untouched for N hours) from the leases volume. Dry run unless --write.
import os, sqlite3, sys, time
d = ".cache/b/l/m/data"
hours = float(next((a.split("=")[1] for a in sys.argv if a.startswith("--hours=")), "3"))
write = "--write" in sys.argv
now = time.time(); freed = 0; removed = 0; kept = 0
for f in sorted(os.listdir(d)):
    if not f.endswith(".sqlite"): continue
    base = f[:-7]; p = os.path.join(d, f)
    try: st = os.stat(p)
    except OSError: continue
    if now - st.st_mtime < hours * 3600: kept += 1; continue
    q = os.path.join(d, base + ".queue")
    if os.path.exists(q) and os.path.getsize(q) > 0 and now - os.stat(q).st_mtime < hours * 3600: kept += 1; continue
    if os.path.exists(p + "-wal") and os.path.getsize(p + "-wal") > 0: kept += 1; continue
    try:
        db = sqlite3.connect(p, timeout=0, isolation_level=None)
        db.execute("BEGIN EXCLUSIVE")
    except sqlite3.Error:
        kept += 1; continue
    try:
        size = sum(os.path.getsize(x) for x in (p, q, p + "-wal", p + "-shm") if os.path.exists(x))
        if write:
            for x in (p, q, p + "-wal", p + "-shm"):
                try: os.unlink(x)
                except FileNotFoundError: pass
        freed += size; removed += 1
    finally:
        try: db.close()
        except Exception: pass
print(("removed" if write else "would remove"), removed, "kept", kept, "logical", round(freed / 1048576, 2), "MiB")
