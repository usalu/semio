#!/bin/zsh
# 📦️ W2 (session 12): ONE wasm hold for describe demonstrator → descriptor preflight (seconds) → `--packages all` publish, so no other
# hold lands between the describe the preflight reads and the publish. Logs and data root under .🧬semio/🌐hub (outside every sweep).
# usage: zsh w2-publish-all.sh [describe-plugin …]
setopt no_bg_nice
set -u
cd /Users/ueli/Documents/semio || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-logs"
DATA="/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-catalog-all"
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
DESCRIBE=("$@")
mkdir -p "$DATA"; chmod 700 "$DATA"
step='for p in '"${DESCRIBE[*]}"'; do echo "[w2-all] START describe $p $(date "+%F %T")"; bun nx run @semio-tech/$p-plugin:describe --outputStyle=stream > "'"$OUT"'/describe-$p.txt" 2>&1 || { echo "[w2-all] describe $p FAILED"; exit 1; }; echo "[w2-all] END describe $p $(date "+%F %T")"; done
echo "[w2-all] START preflight $(date "+%F %T")"; bun /Users/ueli/Documents/semio/.tmp-ticket/wp-w2/w2-preflight-probe.ts all || exit 1
echo "[w2-all] START publish w2-catalog-all $(date "+%F %T")"; s=$(date +%s)
OS_HUB_DATA="'"$DATA"'" bun nx run os-hub:trusted-catalog-bootstrap --packages all --outputStyle=stream > "'"$OUT"'/publish-w2-catalog-all-2.txt" 2>&1; rc=$?
echo "[w2-all] END publish w2-catalog-all rc=$rc wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; exit $rc'
zsh "$MUTEX[1]" wasm w2 -- zsh -c "$step"
echo "[w2-all] DONE rc=$? $(date '+%F %T')"
