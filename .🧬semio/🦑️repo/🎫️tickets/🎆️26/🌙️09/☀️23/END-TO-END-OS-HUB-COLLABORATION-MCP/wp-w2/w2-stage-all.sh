#!/bin/zsh
# 🧱️ W2: per component, in ONE fleet wasm-mutex hold: `describe` (links the component-dev unit into the shared target
# and writes the committed descriptor) then `materialize-dev` (its `component-dev` reuses that unit and stages the
# browser module). Shared cargo target, CARGO_INCREMENTAL=0. A failed component waits 10 min and retries once
# (a peer's transient half-edit); a second failure is recorded and the chain continues.
# usage: zsh w2-stage-all.sh <project …>   (Nx project names, e.g. @semio-tech/note-plugin)
set -u
cd /Users/ueli/Documents/semio || exit 1
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-w2/generated
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
echo "[w2-stage] $# projects $(date '+%F %T')"
for p in "$@"; do
  n=${p#@semio-tech/}
  if [ -f "$OUT/stage-$n.ok" ]; then echo "[w2-stage] SKIP $n"; continue; fi
  for attempt in 1 2; do
    echo "[w2-stage] START $n attempt=$attempt $(date '+%F %T')"
    s=$(date +%s)
    zsh "$MUTEX[1]" wasm w2 -- zsh -c "bun nx run '$p:describe' --outputStyle=stream && bun nx run '$p:materialize-dev' --outputStyle=stream" > "$OUT/stage-$n.txt" 2>&1
    rc=$?
    echo "[w2-stage] END $n attempt=$attempt rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
    if [ $rc -eq 0 ]; then touch "$OUT/stage-$n.ok"; break; fi
    grep -E 'error(\[E[0-9]+\])?:' "$OUT/stage-$n.txt" | head -5 | sed 's/^/    /'
    cp "$OUT/stage-$n.txt" "$OUT/stage-$n-attempt$attempt.txt"
    [ $attempt -eq 1 ] && sleep 600
  done
done
echo "[w2-stage] DONE $(date '+%F %T')"
