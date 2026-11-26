#!/bin/zsh
# [DEBUG] polls the mtime of the store test-support sources; when they settle it checks that crate for wasm32-wasip2 under the guard and, when the check passes, runs the guarded stdio then stdio-office rebuild; retries until a build exits 0
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; L=$T/🗑️generated/claude-split; mkdir -p $L
SRC="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support"
stamp() { find "$SRC" -name '*.rs' -exec stat -f '%m' {} + | sort -n | tail -1; }
attempt=0
while [ $attempt -lt 12 ]; do
  s1=$(stamp); sleep 120; s2=$(stamp)
  if [ "$s1" != "$s2" ]; then echo "[DEBUG] $(date +%T) sources still changing"; continue; fi
  attempt=$((attempt+1))
  echo "[DEBUG] $(date +%T) attempt $attempt: checking test-support"
  MIN_DISK_GIB=20 $T/claude-resource-guard.sh $T/claude-testsupport-check.sh > $L/ts-check-$attempt.log 2>&1; c=$?
  if [ $c -ne 0 ]; then echo "[DEBUG] check failed ($c); waiting for a source change"; until [ "$(stamp)" != "$s2" ]; do sleep 60; done; continue; fi
  echo "[DEBUG] $(date +%T) check passed; rebuilding"
  MIN_DISK_GIB=20 $T/claude-resource-guard.sh $T/claude-pane-cargo-logged.sh > $L/rebuild-$attempt.out 2>&1; c=$?
  echo "[DEBUG] rebuild exit $c"
  if [ $c -eq 0 ]; then echo "BUILD-OK attempt $attempt"; exit 0; fi
  until [ "$(stamp)" != "$s2" ]; do sleep 60; done
done
echo "GAVE-UP"
