#!/bin/sh
# 🪧️ Laws of batch `diskbud`: sh verify.sh <root> [before]   (root = the checkout or a mirror root that holds the applied batch; `before` restores the base of the two edited files first, runs only the new laws, which must be RED, and applies the batch again afterwards)
ROOT=${1:?root required}; MODE=${2:-after}; HERE=$(cd "$(dirname "$0")" && pwd)
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'; M="$R/🧭️invocation/🕸️graph/🧠️memo"; J="$R/🏡️resident/📜️journal"
cd "$ROOT" || exit 2
if [ "$MODE" = before ]; then
  python3 -I - "$HERE" "$ROOT" <<'PY' || exit 2
import json, os, sys
here, root = sys.argv[1:3]
for rel, row in json.load(open(os.path.join(here, "manifest.json"), encoding="utf8")).items():
    if row["base"] is not None: open(os.path.join(root, rel), "wb").write(open(os.path.join(here, "base", row["file"]), "rb").read())
PY
else
  python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -q '^base' && { echo "batch not applied in $ROOT"; exit 2; }
fi
FAILED=0
law() { name=$1; shift; out=$("$@" 2>&1); code=$?; summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|skip|fail)$" | tr '\n' ' '); if [ "$MODE" = before ]; then if [ $code -ne 0 ]; then echo "RED (expected) $name: $summary"; else echo "GREEN on base (law proves nothing) $name: $summary"; FAILED=1; fi; else if [ $code -eq 0 ]; then echo "GREEN $name: $summary"; else echo "RED   $name: $summary"; printf '%s\n' "$out" | grep -E "^\(fail\)|^error" | head -8; FAILED=1; fi; fi; }
law "graph memo retention (Python model on 400 seeded states, byte budget, real collection)" bun test "./$M/🧪️tests/🧹️retention/🟦️.ts"
law "abandoned journal state (Python model, real sweep, starting service)"                 bun test "./$J/🧪️tests/🧹️abandoned/🟦️.ts"
if [ "$MODE" = before ]; then python3 -I "$HERE/stage.py" apply --root "$ROOT" >/dev/null; exit $FAILED; fi
law "graph memo (policy, key, model, soundness, lifecycle)"   bun test "./$M/🧪️tests/🟦️.ts"
law "graph memo slice sidecar"                                 bun test "./$M/🧪️tests/🎯️slice/🟦️.ts"
law "graph memo carry"                                         bun test "./$M/🧪️tests/🧳️carry/🟦️.ts"
law "journal lock"                                             bun test "./$J/🧪️tests/🔒️lock/🟦️.ts"
law "journal directory"                                        bun test "./$J/🧪️tests/📍️directory/🟦️.ts"
law "journal world"                                            bun test "./$J/🔭️world/🧪️tests/🟦️.ts"
law "every test file belongs to one suite"                     bun test "./$R/🗂️suites/🧪️tests/🟦️.ts"
exit $FAILED
