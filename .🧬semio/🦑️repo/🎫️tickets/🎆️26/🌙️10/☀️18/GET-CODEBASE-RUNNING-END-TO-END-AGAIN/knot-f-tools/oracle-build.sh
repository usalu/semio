#!/bin/sh
# builds the package-body Rust oracle through the issued cargo entry exactly as the test does; prints the compiler-artifact lines
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔮️oracles/📦️package-boundary-classification/📦️packages/🦀️rust/Cargo.toml"
s=$(date +%s)
SEMIO_BUILD_MODE=prototype nice -n 10 bun repo cargo --store=shared --configuration=prototype -- build --quiet --message-format=json --manifest-path "$M" > "$C/🗑️generated/knot-f/oracle-build.out" 2> "$C/🗑️generated/knot-f/oracle-build.err"
echo "oracle-build rc=$? $(( $(date +%s)-s ))s" >> "$C/🗑️generated/knot-f/verdicts.txt"
