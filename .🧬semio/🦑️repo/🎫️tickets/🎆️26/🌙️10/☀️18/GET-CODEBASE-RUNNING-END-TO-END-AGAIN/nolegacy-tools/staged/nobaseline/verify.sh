#!/bin/sh
# 🗑️ Staged batch `nl-nobaseline` (lane nolegacy): the unwired cutover module `🔒️leases/📦️namespace/📒️baseline` and its tests are deleted with every reference (descriptor law cases, drain branches, statfs kept site, publication type-check list, schema-document index).
# usage: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (stage state, Bun parse-check via gate-g-tools/scan.ts, docstring/emoji/[DEBUG] censuses and the language law on the batch files via compliance-tools/verify_batch.py, then the batch's laws)
set -e
here=$(cd "$(dirname "$0")" && pwd)
c=$(cd "$here/../../.." && pwd)
root=${VERIFY_ROOT:?VERIFY_ROOT required}
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$c/compliance-tools/verify_batch.py" "$here" "$root"
cd "$root"
bun test './🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🧪️tests/📂️descriptors/🟦️.ts' './🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📐️statfs/🧪️tests/🆓️unknown/🟦️.ts'
