#!/bin/zsh
# 🧹 Disk guard: every 5 min; below 130 GiB free prune idle incremental sessions (> 60 min); below 80 GiB prune nx cache entries older than 12 h and build units only when
# cargo holds no lock on them (`.lock` taken exclusively, non-blocking), their newest file is older than 3 h (1 h below 40 GiB; semio-* packages only), and a newer unit of the same package exists.
# Below 40 GiB idle incremental sessions older than 20 min go too. Build units are pruned ONLY in a disk emergency (< 3 GiB free): semio-* keep their newest
# unit (older ones idle > 6 h go), third-party packages their 4 newest (older ones idle > 24 h): cargo reuses or fingerprint-checks a unit without taking its
# `.lock` or touching its mtime, so any prune can race a running build (vello_shaders OUT_DIR 03:28, stdio-zip/docx invoked.timestamp 03:40/03:55, 10-04).
# Every sweep deletes codegen objects (`*.rcgu.o`) older than 60 min — leftovers of killed builds (3.9 GiB in one stale unit, 10-04).
# Last resort below 3 GiB free (any unit prune races live builds; at 10 GiB it killed checks at 11:37) (disk hit 1 GiB at 06:31 with every unit recent): `🧹️s4-one-off-prune.py --emergency` keeps only the newest unit per package.
root="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build"
log="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/coord/disk-guard.txt"
mkdir -p "${log:h}"
free_gib() { df -g /System/Volumes/Data | awk 'NR==2{print $4}' }
while true; do
  before=$(free_gib)
  find "$root" -name '*.rcgu.o' -mmin +60 -delete 2>/dev/null
  if [ "$before" -lt 130 ]; then
    find "$root" -type d -name incremental -prune 2>/dev/null | while read -r inc; do
      find "$inc" -mindepth 2 -maxdepth 2 -type d -mmin +$([ "$(free_gib)" -lt 40 ] && echo 20 || echo 60) -exec rm -rf {} + 2>/dev/null
    done
    if [ "$(free_gib)" -lt 80 ]; then
      python3 - "$root" "$([ "$(free_gib)" -lt 40 ] && echo 1 || echo 3)" >> "$log" 2>&1 <<'PY'
import fcntl, os, shutil, sys, time
root = sys.argv[1]
bound = time.time() - int(sys.argv[2]) * 3600
def newest(path):
    m = os.stat(path).st_mtime
    for base, dirs, files in os.walk(path):
        for f in files:
            try: m = max(m, os.lstat(os.path.join(base, f)).st_mtime)
            except OSError: pass
    return m
removed = 0
for profile_build in [os.path.join(root, "debug", "build")] + [os.path.join(root, t, p, "build") for t in ("wasm32-wasip2", "wasm32-unknown-unknown") for p in (os.listdir(os.path.join(root, t)) if os.path.isdir(os.path.join(root, t)) else [])]:
    if not os.path.isdir(profile_build): continue
    for pkg in os.listdir(profile_build):
        own = pkg.startswith("semio-")
        if shutil.disk_usage(root).free >= 3 * 2**30: continue
        keep, limit = (1, time.time() - 6 * 3600) if own else (4, time.time() - 24 * 3600)
        pdir = os.path.join(profile_build, pkg)
        units = [os.path.join(pdir, u) for u in os.listdir(pdir) if os.path.isdir(os.path.join(pdir, u))]
        if len(units) <= keep: continue
        ages = sorted(((newest(u), u) for u in units), reverse=True)
        for m, u in ages[keep:]:
            if m > limit: continue
            lock = os.path.join(u, ".lock")
            try:
                fd = os.open(lock, os.O_RDWR | os.O_CREAT)
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except OSError:
                continue
            try:
                shutil.rmtree(u, ignore_errors=True); removed += 1
            finally:
                os.close(fd)
print(time.strftime("%F %T"), "unit prune removed", removed)
PY
    fi
    if [ "$(free_gib)" -lt 80 ]; then
      nxc="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/nx"
      n=$(find "$nxc" -maxdepth 1 -mindepth 1 -name '[0-9]*' -mmin +720 2>/dev/null | wc -l | tr -d ' ')
      find "$nxc" -maxdepth 1 -mindepth 1 -name '[0-9]*' -mmin +720 -exec rm -rf {} + 2>/dev/null
      echo "$(date '+%F %T') nx cache prune removed $n entries older than 12 h" >> "$log"
    fi
    if [ "$(free_gib)" -lt 3 ]; then
      python3 "${0:A:h}/🧹️s4-one-off-prune.py" --emergency >> "$log" 2>&1
    fi
    echo "$(date '+%F %T') free ${before} GiB -> $(free_gib) GiB" >> "$log"
  fi
  sleep 300
done
