#!/bin/sh
# compiles semio-framework-job twice into two private target dirs WITHOUT any wrapper/cache and prints the sha256 of the metadata/rlib outputs
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/cap-c/determinism"
cd /Users/ueli/Documents/semio || exit 1
for n in 1 2; do
  nice -n 10 bun repo tools exec cargo -- build --locked --offline -p semio-framework-job --target wasm32-wasip2 --manifest-path Cargo.toml --target-dir "$C/t$n" 2>&1 | tail -40
done
for n in 1 2; do
  find "$C/t$n/wasm32-wasip2" -path '*deps*' \( -name 'libsemio_framework_job-*.rmeta' -o -name 'libsemio_framework_job-*.rlib' -o -name 'libsemio_framework_value-*.rmeta' \) -exec shasum -a 256 {} \; | sed "s#$C/##"
done
echo "rc=$?"
