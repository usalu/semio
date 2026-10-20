#!/bin/sh
# After GO and apply: runs every verified suite live, one process at a time (explicit file paths only).
# usage: sh live.sh   (results on stdout; logs in /private/tmp/testfix-real/logs)
T="$(cd "$(dirname "$0")" && pwd)"
cd /Users/ueli/Documents/semio || exit 1
for f in \
  '🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🚧️boundary/🟦️.ts' \
  '🧰️framework/🛍️products/🧪️tests/🧭️fixture-placement/🟦️.ts' \
  '✏️s/🧪️tests/🗂️native-ownership/🟦️.ts' \
  '🧰️framework/🔨️modules/📡️replication/🧪️tests/🏛️crate-partition/🟦️.ts' \
  '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/🟦️.ts' \
  '🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️mutation-attrs/🟦️.ts' \
  '🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧳️retained/🧪️tests/🟦️.ts' ; do
  echo "## $f"; LIVE=1 python3 "$T/one.py" "$f" | tail -4
done
echo "## replication oracle"; bun -e 'import {testRetainedMutationOracle} from "./🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🧪️tests/🟦️.ts"; testRetainedMutationOracle(); console.log("ORACLE OK")' 2>&1 | tail -2
