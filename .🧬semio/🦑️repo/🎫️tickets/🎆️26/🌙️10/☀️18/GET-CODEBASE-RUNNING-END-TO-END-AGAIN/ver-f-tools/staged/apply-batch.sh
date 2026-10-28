#!/bin/sh
# usage: apply-batch.sh — applies every staged patch to the real tree in ONE batch (run after the 02:45 freeze end, from anywhere)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/ver-f-tools/staged"
P="🧰️framework/🔨️modules/🏃️process"
set -e
python3 "$C/patch-sandbox-test.py" "$P/🧪️testing/🏖️sandbox/🧪️tests/🟦️.ts"
python3 "$C/patch-ledger.py" "$P/🧪️testing/🏖️sandbox/🧪️tests/🧫️fixtures/📒️ledger.json"
python3 "$C/patch-route-toolchains.py" "$P/🧪️testing/🗂️suites/🔣️.json"
python3 "$C/patch-exact-inputs.py" "$P/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/📥️inputs/🟦️.ts"
python3 "$C/patch-leases-drain.py" "$P/🔒️leases/🟦️.ts"
python3 "$C/patch-wait-test.py" "$P/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/⏳️wait/🧪️tests/🟦️.ts"
python3 "$C/patch-silence-test.py" "$P/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤫️silence/🧪️tests/🟦️.ts"
python3 "$C/patch-image-scratch.py"
python3 "$C/patch-storage-backends.py"
python3 "$C/patch-runtime-bootstrap-test.py" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts"
python3 "$C/patch-dependency-storage-test.py" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts"
echo applied
