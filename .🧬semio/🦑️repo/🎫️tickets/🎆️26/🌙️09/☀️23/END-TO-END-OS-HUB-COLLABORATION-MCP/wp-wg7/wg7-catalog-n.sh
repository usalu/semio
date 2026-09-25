#!/bin/zsh
# 🗒️ WG7 note catalog chain: builds os-hub (hub mutex), publishes a stdio,gis,note trusted catalog from the current tree (wasm mutex),
# materializes the catalog's own note for the ticket serve (wasm mutex), and boots hub 8050 on a fresh copy of that generation with
# both human credentials (`wg7-hub-8050.sh`). Durable paths only (`.🧬semio/🌐hub/s11-wg7-*`, preamble rule 15).
# usage: zsh wg7-catalog-n.sh   (detached; progress lines in s11-wg7-logs/n-chain.txt)
set -u
R=/Users/ueli/Documents/semio
H=$R/.🧬semio/🌐hub
L=$H/s11-wg7-logs
CAT=$H/s11-wg7-catalog-n
MUTEX=($R/.tmp-ticket/*fleet-mutex.sh)
HUB_RUST=($R/🌎️hub/📦️packages/🦀️rust)
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
cd $R || exit 1
step() { echo "[wg7-n] $1 $(date '+%F %T')"; }
fail() { step "FAIL $1"; tail -30 "$2"; exit 1; }

step "START hub-build"
zsh $MUTEX[1] hub WG7 -- zsh -c "cd '$HUB_RUST[1]' && cargo build --manifest-path Cargo.toml --bin os-hub" > $L/n-hub-build.txt 2>&1 || fail hub-build $L/n-hub-build.txt

step "START publish"
[ -e $CAT/trusted-catalog/current.json ] && { step "FAIL catalog root $CAT already published"; exit 1; }
mkdir -p $CAT && chmod 700 $CAT
zsh $MUTEX[1] wasm WG7 -- env OS_HUB_DATA=$CAT bun nx run os-hub:trusted-catalog-bootstrap --packages stdio,gis,note --outputStyle=stream > $L/n-publish.txt 2>&1 || fail publish $L/n-publish.txt

step "START served-note"
zsh $MUTEX[1] wasm WG7 -- bun $R/.tmp-ticket/wp-wg7/wg7-catalog-module.ts $CAT note semio_s_plugin_note > $L/n-catalog-module.txt 2>&1 || fail served-note $L/n-catalog-module.txt

zsh $R/.tmp-ticket/wp-wg7/wg7-hub-8050.sh
