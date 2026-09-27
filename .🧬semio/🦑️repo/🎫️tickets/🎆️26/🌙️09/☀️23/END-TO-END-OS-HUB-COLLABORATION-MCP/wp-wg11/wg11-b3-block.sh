#!/bin/zsh
# 🪪️ WG11: materializes catalog B3's exact block component into a durable module root (`s14-wg11-b3-block`) and rewrites its shard
# worker from the current generator (WG9's glue), for a wasm32 block2d serve (cross-shell pairings with the native law's block2d).
set -u
R=/Users/ueli/Documents/semio
cd $R || exit 1
echo "[wg11-block] start $(date '+%F %T')"
nice -n 10 bun $R/.tmp-ticket/wp-wg7/wg7-catalog-module.ts "$R/.🧬semio/🌐hub/s13-w3-catalog-b3" block semio_s_plugin_block s14-wg11-b3-block 2>&1 | tail -4
echo "[wg11-block] materialize rc=$pipestatus[1] $(date '+%F %T')"
nice -n 10 bun $R/.tmp-ticket/wp-wg9/wg9-module-glue.ts s14-wg11-b3-block
echo "[wg11-block] glue rc=$? $(date '+%F %T')"
