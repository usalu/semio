#!/bin/zsh
# 🧹 Disk guard: every 5 min; below 130 GiB free prune idle incremental sessions (> 60 min); below 80 GiB prune nx cache entries older than 12 h and build units only when
# cargo holds no lock on them (`.lock` taken exclusively, non-blocking), their newest file is older than 3 h (1 h below 40 GiB; semio-* packages only), and a newer unit of the same package exists.
# Below 40 GiB idle incremental sessions older than 20 min go too.
root="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build"
log="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/coord/disk-guard.txt"
mkdir -p "${log:h}"
free_gib() { df -g /System/Volumes/Data | awk 'NR==2{print $4}' }
while true; do
  before=$(free_gib)
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
        if not pkg.startswith("semio-"): continue
        pdir = os.path.join(profile_build, pkg)
        units = [os.path.join(pdir, u) for u in os.listdir(pdir) if os.path.isdir(os.path.join(pdir, u))]
        if len(units) < 2: continue
        ages = sorted(((newest(u), u) for u in units), reverse=True)
        for m, u in ages[1:]:
            if m > bound: continue
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
    echo "$(date '+%F %T') free ${before} GiB -> $(free_gib) GiB" >> "$log"
  fi
  sleep 300
done
