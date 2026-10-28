#!/bin/sh
# legs.sh : the pure bun-test legs of narrowed targets, one at a time, each under the Seatbelt read oracle with exactly the declared set.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$C/keys2-g-tools"; F="🧰️framework"; D="$F/🛍️products/🦑️repo/🔨️modules/🎛️dashboard"
BUN=$(bun -e 'console.log(process.execPath)')
FW="$T/rules/framework-rs-tests.json"; FPJ="$F/📦️packages/🦀️rust/📋️project.json"; DR="$T/rules/dashboard-cargo-tests.json"; DPJ="$D/📦️packages/🦀️rust/📋️project.json"
fw() { tag="$1"; target="$2"; shift 2; rule="$FW"; [ -f "$T/rules/framework-rs-tests-$tag.json" ] && rule="$T/rules/framework-rs-tests-$tag.json"; sh "$T/oracle-leg.sh" "leg-$tag" "$rule" @semio-tech/framework-rs "$FPJ" "$target" -- "$BUN" test "$@"; }
fw source-projection test-source-projection "./$F/🔨️modules/📁️filesystem/📷️snapshot/🧪️tests/🟦️.ts"
fw directory-discovery test-directory-discovery "./$F/🔨️modules/📁️filesystem/🔎️discovery/🧪️tests/🟦️.ts"
fw host-effect test-host-effect-invocation "./$F/🔨️modules/🛂️manifest/🧪️tests/🔁️host-effect-invocation/🟦️.ts"
fw fixture-ownership test-fixture-ownership-source "./$F/🧪️tests/🧱️fixture-ownership/🟦️.ts"
fw artifact-kind test-artifact-kind-source "./$F/🔨️modules/🚪️io/🧬️schema/🧪️tests/🪪️artifact-kind/🟦️.ts"
fw sqlite-source test-snapshot-sqlite-source "./$F/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🟦️.ts"
sh "$T/oracle-leg.sh" leg-dash-windows "$DR" @semio-tech/repo-dashboard-rs "$DPJ" test-windows -- "$BUN" test "./$D/🧪️tests/🪟️windows/🟦️.ts" "./$D/🧪️tests/⌨️controls/🟦️.ts" "./$F/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🖱️pointer-routing/🟦️.ts"
echo done > "$C/🗑️generated/keys2-g/legs.done"
