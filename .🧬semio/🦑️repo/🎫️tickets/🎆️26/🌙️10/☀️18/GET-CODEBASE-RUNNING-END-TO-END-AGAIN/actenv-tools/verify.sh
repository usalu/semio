#!/bin/sh
# 🛡️ verify.sh <root>: runs the actenv laws against the checkout at <root> (a mirror, or the live root once the batch is applied). Never resolves a root by itself.
# Mirror use : python3 -I C/mirror-tools/mirror.py run <name> --peers-ok -- sh C/actenv-tools/verify.sh "$TMPDIR/sm/<name>"
# Live use   : sh C/actenv-tools/verify.sh "$(git rev-parse --show-toplevel)"   (the laws write below $TMPDIR only)
ROOT=${1:?root required}; HERE=$(cd "$(dirname "$0")" && pwd)
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'; BZ="$R/🔧️tools/🏗️bazel"
cd "$ROOT" || exit 2
FAILED=0
law() { name=$1; shift; out=$("$@" 2>&1); code=$?; summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' '); if [ $code -eq 0 ]; then echo "GREEN $name: $summary"; else echo "RED   $name: $summary"; printf '%s\n' "$out" | grep -E "^\(fail\)|^error" | head -8; FAILED=1; fi; }
python3 -I "$HERE/staged/stage.py" check --root "$ROOT" | grep -q drifted && { echo "RED   batch files drifted from the batch (python3 -I $HERE/staged/stage.py check)"; FAILED=1; }
law "action environment channel (allowlist, Python oracle, runtime.json writer, call sites, real child, read inventory)" bun test "./$BZ/🌿️environment/🏃️action/🧪️tests/🟦️.ts"
law "keyed command environment"                                        bun test "./$BZ/🌿️environment/🧪️tests/🟦️.ts"
law "action keys independent of ambient variables (fingerprint)"      bun test "./$BZ/🧪️tests/📥️fingerprint/🟦️.ts"
law "runtime channel readers (actions)"                                bun test "./$BZ/🧪️tests/📥️actions/🟦️.ts"
law "workspace environment"                                            bun test "./$BZ/🧪️tests/📥️workspace/🟦️.ts"
law "plan reuse"                                                       bun test "./$BZ/♻️plan-reuse/🧪️tests/🟦️.ts"
law "plan reuse whole"                                                 bun test "./$BZ/♻️plan-reuse/🧪️tests/🌌️whole/🟦️.ts"
law "world witness"                                                    bun test "./$R/🔭️witness/🧪️tests/🟦️.ts"
law "every test file belongs to one suite"                             bun test "./$R/🗂️suites/🧪️tests/🟦️.ts"
exit $FAILED
