#!/bin/zsh
# 🧪️ Re-runs the kept post-round-3a renderer wgpu lib binary (the one whose suite hung 36 min in r6-base) as a whole suite, once
# with 4 threads (as the suites run it) and once serially (deterministic order: a hang names its test and its predecessors), each
# bounded by a 420 s alarm. Capture `.🧬semio/🌐hub/s14-u6-logs/renderer-hang-repro.txt` (+ one raw file per run).
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
B=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-target/r6-live-renderer/semio_framework_os_renderer_wgpu-4b607c95eb950a60
cd /Users/ueli/Documents/semio || exit 2
export RUST_MIN_STACK=67108864
for threads in 4 1; do
  raw=$L/renderer-hang-repro-t$threads.txt
  start=$(date +%s)
  perl -e 'alarm 420; exec @ARGV' "$B" --test-threads $threads > $raw 2>&1
  rc=$?
  echo "RUN threads=$threads rc=$rc secs=$(( $(date +%s) - start )) ok=$(/usr/bin/grep -c ' \.\.\. ok$' $raw) failed=$(/usr/bin/grep -c ' \.\.\. FAILED$' $raw) result=$(/usr/bin/grep '^test result' $raw | tr -d '\n')"
  if [ $rc != 0 ]; then
    "$B" --list --format terse 2>/dev/null | sed -n 's/: test$//p' | sort > $raw.all
    sed -nE 's/^test (.*) \.\.\. (ok|FAILED|ignored)$/\1/p' $raw | sort > $raw.done
    comm -23 $raw.all $raw.done | sed 's/^/UNFINISHED /'
    rm -f $raw.all $raw.done
  fi
done
echo "REPRO-DONE $(date +%T)"
