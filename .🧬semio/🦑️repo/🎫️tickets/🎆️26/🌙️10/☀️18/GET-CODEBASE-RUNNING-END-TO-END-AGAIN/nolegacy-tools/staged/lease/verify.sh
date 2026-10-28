#!/bin/sh
# 🔒️ Staged batch `nl-lease` (lane nolegacy): the lease holder state `legacy` and its 4 h reclaim are removed; a registry entry without a standing is not read, a holder without a readable standing is `unknown` and never stopped; schema, corpus, Python oracle, status, fleet status, queue notices and docs EN+DE follow.
# usage: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (stage state, Bun parse-check via gate-g-tools/scan.ts, docstring/emoji/[DEBUG] censuses and the language law on the batch files via compliance-tools/verify_batch.py, then the batch's laws)
set -e
here=$(cd "$(dirname "$0")" && pwd)
c=$(cd "$here/../../.." && pwd)
root=${VERIFY_ROOT:?VERIFY_ROOT required}
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$c/compliance-tools/verify_batch.py" "$here" "$root"
cd "$root"
bun test './🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🟦️.ts' './🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/📋️status/🟦️.ts' './🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🕸️wait-for/🟦️.ts' './🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/🧪️tests/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🧪️tests/🔎️probes/🟦️.ts' './🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions/🧪️tests/🔗️adapters/🟦️.ts'
