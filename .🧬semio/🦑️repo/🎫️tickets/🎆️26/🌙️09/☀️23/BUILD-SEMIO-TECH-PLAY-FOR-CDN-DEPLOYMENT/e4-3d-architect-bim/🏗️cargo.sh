#!/bin/bash
# 🏗️ One bounded E4 cargo call inside a fleet slot with private target and build dirs under `⚡️cache/e4`, plus a 2 s
# rustc RSS sampler that keeps per-crate peaks and the seconds each crate's rustc was seen running (`⚡️cache/e4/peaks/<tag>.tsv`) across resumed slices. The whole cargo
# process group is interrupted after <seconds>, so a foreground call never outlives its deadline; rerunning resumes.
# usage: 🏗️cargo.sh <tag> <seconds> <cargo args...>     log: `⚡️cache/e4/logs/<tag>.log`
TAG=$1; SECS=$2; shift 2
REPO=/Users/ueli/Documents/semio; E4="$REPO/.🧬semio/🦑️repo/⚡️cache/e4"; mkdir -p "$E4/logs" "$E4/peaks"
LOG="$E4/logs/$TAG.log"; PEAKS="$E4/peaks/$TAG.tsv"
cd "$REPO" || exit 2
set -m
python3 - "$PEAKS" <<'PY' &
import subprocess, time, sys, os
path = sys.argv[1]; peaks = {}; first = {}; last = {}
if os.path.exists(path):
    for line in open(path):
        size, name, *span = line.rstrip("\n").split("\t"); peaks[name] = int(float(size[:-1]) * 1048576)
while True:
    now = time.time()
    for line in subprocess.run(["ps", "-Ao", "rss=,command="], capture_output=True, text=True).stdout.splitlines():
        p = line.split(None, 1)
        if len(p) < 2 or "⚡️cache/e4/" not in p[1]: continue
        t = p[1].split()
        if t[0].endswith("/rustc") and "--crate-name" in t:
            n = t[t.index("--crate-name") + 1]
            peaks[n] = max(peaks.get(n, 0), int(p[0])); first.setdefault(n, now); last[n] = now
    with open(path + ".tmp", "w") as f:
        for k, v in sorted(peaks.items(), key=lambda kv: -kv[1]): f.write(f"{v/1048576:.3f}G\t{k}\t{(last[k]-first[k]) if k in first else 0:.0f}s\n")
    os.replace(path + ".tmp", path)
    time.sleep(2)
PY
SAMPLER=$!
env CARGO_TARGET_DIR="$E4/t" CARGO_BUILD_BUILD_DIR="$E4/b" CARGO_TERM_COLOR=never \
  "$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "e4-$TAG" -- cargo "$@" >> "$LOG" 2>&1 &
PID=$!; START=$(date +%s)
while kill -0 $PID 2>/dev/null; do
  if [ $(( $(date +%s) - START )) -ge "$SECS" ]; then kill -INT -- -$PID 2>/dev/null; sleep 5; kill -TERM -- -$PID 2>/dev/null; wait $PID; echo "INTERRUPTED after ${SECS}s"; break; fi
  sleep 2
done
wait $PID 2>/dev/null; RC=$?
kill $SAMPLER 2>/dev/null; wait $SAMPLER 2>/dev/null
echo "rc=$RC wall=$(( $(date +%s) - START ))s errors=$(grep -c '^error' "$LOG") warnings=$(grep -c '^warning' "$LOG")"
grep -q 'Finished' "$LOG" && echo DONE
exit $RC
