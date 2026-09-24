#!/bin/zsh
# 🔄️ W2: restage the `s` lane from the current tree. Describe-all in ONE wasm hold (Nx re-describes only components whose inputs
# changed, 2 at a time on the shared build-dir), plugin-registry generate (wasm hold) + check, activate-s-react-dev (wasm hold,
# re-materializes missing/stale modules), then verify committed == dist == shared == staged for all 60. Logs outside `generated`.
set -u
cd /Users/ueli/Documents/semio || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-logs"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w2
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
tag=${1:-restage}
step() { echo "[w2-restage] START $1 $(date '+%F %T')"; s=$(date +%s); }
done_() { echo "[w2-restage] END $1 rc=$2 wall=$(( $(date +%s) - s ))s $(date '+%F %T')"; [ "$2" -eq 0 ] || { tail -30 "$3" | sed 's/^/    /'; exit "$2"; }; }
step describe-all; zsh "$MUTEX[1]" wasm w2 -- bun nx run-many -t describe --exclude @semio-tech/os-plugin-describe-rs --parallel=2 --outputStyle=stream > "$OUT/$tag-describe.txt" 2>&1; done_ describe-all $? "$OUT/$tag-describe.txt"
step generate; zsh "$MUTEX[1]" wasm w2 -- bun nx run @semio-tech/plugin-registry:generate --outputStyle=stream > "$OUT/$tag-generate.txt" 2>&1; done_ generate $? "$OUT/$tag-generate.txt"
step check; bun nx run @semio-tech/plugin-registry:check --outputStyle=stream > "$OUT/$tag-check.txt" 2>&1; done_ check $? "$OUT/$tag-check.txt"
step activate-s; zsh "$MUTEX[1]" wasm w2 -- bun nx run @semio-tech/framework-os-dev:activate-s-react-dev --outputStyle=stream > "$OUT/$tag-activate.txt" 2>&1; done_ activate-s $? "$OUT/$tag-activate.txt"
step verify; bun "$W/w2-verify-staged.ts" > "$OUT/$tag-verify.txt" 2>&1; done_ verify $? "$OUT/$tag-verify.txt"
echo "[w2-restage] DONE $(date '+%F %T')"
