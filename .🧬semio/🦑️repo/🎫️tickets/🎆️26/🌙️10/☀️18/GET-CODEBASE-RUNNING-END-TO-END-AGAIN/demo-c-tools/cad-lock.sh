#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- update --manifest-path "✏️s/🧑‍💻dev/📐️cad/Cargo.toml" --offline -p wasm-bindgen -p js-sys -p web-sys -p wasm-bindgen-futures
echo rc=$?
