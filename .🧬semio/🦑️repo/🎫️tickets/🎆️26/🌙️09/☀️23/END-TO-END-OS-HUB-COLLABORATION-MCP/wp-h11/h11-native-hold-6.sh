#!/bin/zsh
# 🔐️ H11: re-run the two stdio open-kind bin laws after the `s.stdio.json` id fix. usage: h11-native-hold-6.sh <label>
cd /Users/ueli/Documents/semio
export H11_NICE=0
zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-h11/h11-cargo.sh "$1-stdio-open-kind" test -p semio-hub --all-features --bin os-hub --no-fail-fast -- launcher_close_cancels_the_startup_catalog_load_instead_of_retrying_it native_openable_stdio_provider_is_the_only_atomic_readiness_transition
