#!/bin/zsh
# 🪪️ WG11 window 3: materializes the ALL catalog's exact note, block and puzzle components into durable module roots
# (`.🧬semio/🌐hub/s14-wg11-all-<plugin>`) and rewrites their shard workers from the current generator (WG9's glue) — the wasm32 wgpu
# serves of the collaboration battery mount the catalog's own bytes, so the hub's execution-target lease admits the documents.
# usage: python3 ../wp-w2/w2-detach.py <log> zsh wg11-all-roots.sh <catalogRoot> [plugin…]
set -u
R=/Users/ueli/Documents/semio
CAT="$1"; shift
[ -d "$CAT" ] || { echo "usage: wg11-all-roots.sh <catalogRoot> [plugin…]"; exit 2; }
cd $R || exit 1
for p in ${@:-note block puzzle}; do
  echo "[wg11-roots] $p start $(date '+%F %T')"
  nice -n 10 bun $R/.tmp-ticket/wp-wg7/wg7-catalog-module.ts "$CAT" "$p" "semio_s_plugin_$p" "s14-wg11-all-$p" 2>&1 | tail -3
  echo "[wg11-roots] $p materialize rc=$pipestatus[1] $(date '+%F %T')"
  nice -n 10 bun $R/.tmp-ticket/wp-wg9/wg9-module-glue.ts "s14-wg11-all-$p"
  echo "[wg11-roots] $p glue rc=$? $(date '+%F %T')"
done
