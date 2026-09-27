#!/bin/zsh
# 🪪️ WG9: materializes catalog B3's exact note, puzzle and draw components into durable module roots
# (`.🧬semio/🌐hub/s13-wg9-b3-<plugin>`) for the wasm32 wgpu serves, one root per plugin (WG7's `wg7-catalog-module.ts`).
# usage: python3 ../wp-w2/w2-detach.py <log> zsh wg9-b3-modules.sh [plugin…]
set -u
R=/Users/ueli/Documents/semio
CAT="$R/.🧬semio/🌐hub/s13-w3-catalog-b3"
cd $R || exit 1
for plugin in "${@:-note puzzle draw}"; do
  for p in ${=plugin}; do
    echo "[wg9-modules] $p start $(date '+%F %T')"
    nice -n 10 bun $R/.tmp-ticket/wp-wg7/wg7-catalog-module.ts "$CAT" "$p" "semio_s_plugin_$p" "s13-wg9-b3-$p" 2>&1 | tail -4
    echo "[wg9-modules] $p rc=$pipestatus[1] $(date '+%F %T')"
  done
done
