#!/bin/zsh
# 🔐️ H11: the shutdown-grace law, then an os-hub binary (default features) carrying it for the live SIGTERM measurement.
# usage: h11-native-hold-9.sh <label>
cd /Users/ueli/Documents/semio
export H11_NICE=0
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h11
zsh $W/h11-cargo.sh "$1-grace-law" test -p semio-hub --all-features --bin os-hub --no-fail-fast -- the_hub_process_exits_within_its_shutdown_grace_however_long_abandoned_blocking_work_runs a_peer_beat_reaches credential_optional_routes
zsh $W/h11-cargo.sh "$1-bin" build -p semio-hub --bin os-hub
