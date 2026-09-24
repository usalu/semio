#!/bin/zsh
# 🧩️ S15: publish a stdio+gis+note trusted catalog (schema v3, with plugin modules) into S15's own hub data root (hub 8040), one wasm hold.
cd /Users/ueli/Documents/semio || exit 1
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
DATA=$(cd .tmp-ticket/wp-s15/hub-8040 && pwd -P)
echo "[s15-catalog] START $(date '+%T') data=$DATA"
zsh .tmp-ticket/📜️fleet-mutex.sh wasm s15 -- env OS_HUB_DATA="$DATA" bun nx run os-hub:trusted-catalog-bootstrap --packages stdio,gis,note --outputStyle=stream
rc=$?
echo "[s15-catalog] END rc=$rc $(date '+%T')"
exit $rc
