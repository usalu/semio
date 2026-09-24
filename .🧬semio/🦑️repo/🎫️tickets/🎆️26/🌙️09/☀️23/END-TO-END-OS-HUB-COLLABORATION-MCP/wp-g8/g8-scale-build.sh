#!/bin/zsh
# g8: check then build the registered scale fixture component in one wasm mutex hold.
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm g8 -- zsh -c 'CARGO_INCREMENTAL=0 cargo check --manifest-path Cargo.toml -p semio-framework-os-scale-fixture --lib --target wasm32-wasip2 --profile wasm-dev --features component-guest && CARGO_INCREMENTAL=0 bun 📜️script.ts build-wasm; echo "G8-EXIT $?"'
