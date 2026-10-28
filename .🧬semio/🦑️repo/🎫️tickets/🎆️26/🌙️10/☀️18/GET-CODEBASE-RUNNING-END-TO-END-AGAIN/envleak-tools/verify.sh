#!/bin/sh
# verify.sh <root>: runs the envleak laws against the checkout at <root> (a mirror, or the live root once the batch is applied). Never resolves a root by itself.
# Mirror use : python3 -I C/mirror-tools/mirror.py run <name> --peers-ok -- sh C/envleak-tools/verify.sh "$TMPDIR/sm/<name>"
# Live use   : sh C/envleak-tools/verify.sh "$(git rev-parse --show-toplevel)"   (the laws write below $TMPDIR only)
ROOT=${1:?root required}; HERE=$(cd "$(dirname "$0")" && pwd)
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'; BZ="$R/🔧️tools/🏗️bazel"
cd "$ROOT" || exit 2
FAILED=0
law() { name=$1; shift; out=$("$@" 2>&1); code=$?; summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' '); if [ $code -eq 0 ]; then echo "GREEN $name: $summary"; else echo "RED   $name: $summary"; printf '%s\n' "$out" | grep -E "^\(fail\)|^error" | head -8; FAILED=1; fi; }
law "keyed command environment (module law, Python oracle, real children)" bun test "./$BZ/🌿️environment/🧪️tests/🟦️.ts"
law "action keys independent of ambient variables (fingerprint harness)"   bun test "./$BZ/🧪️tests/📥️fingerprint/🟦️.ts"
law "world witness (ambient variables leave no row)"                         bun test "./$R/🔭️witness/🧪️tests/🟦️.ts"
law "plan reuse"                                                             bun test "./$BZ/♻️plan-reuse/🧪️tests/🟦️.ts"
law "plan reuse ignore sources"                                              bun test "./$BZ/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts"
law "every test file belongs to one suite"                                   bun test "./$R/🗂️suites/🧪️tests/🟦️.ts"
if git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1; then
 L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws'
 law "direct tool declarations (needs git)"     bun test "./$L/🧰️direct-tool-declarations/🟦️.ts"
 law "language catalogs"                        bun test "./$L/🗣️language-catalogs/🟦️.ts"
 law "raw process spawn"                        bun test "./$L/🚷️raw-process-spawn/🟦️.ts"
fi
echo "--- real-process oracle: digests of the real runtime input 'repo tools exec --light bun -- --version' (merged = base behaviour, keyed = fixed)"
out=$(bun "$HERE/oracle-runtime.ts" "$ROOT" 2>&1); echo "$out" | tail -9
echo "$out" | grep -q '"keyedDistinct":1' || { echo "RED   keyed digest is not unique"; FAILED=1; }
exit $FAILED
