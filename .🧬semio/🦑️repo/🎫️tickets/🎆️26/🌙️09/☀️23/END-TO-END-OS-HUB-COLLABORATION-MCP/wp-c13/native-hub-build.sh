#!/bin/zsh
# 🏗️ C13 (14c): os-hub from the live tree (kernel-db foreign-transition refusal included) under ONE native-lane hold.
export NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-target"
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c13 -- zsh -c 'cd /Users/ueli/Documents/semio && nice -n 15 cargo build -p semio-hub --bin os-hub; echo "BUILD rc=$? $(date "+%F %T")"'
echo "EXIT rc=$? $(date '+%F %T')"
