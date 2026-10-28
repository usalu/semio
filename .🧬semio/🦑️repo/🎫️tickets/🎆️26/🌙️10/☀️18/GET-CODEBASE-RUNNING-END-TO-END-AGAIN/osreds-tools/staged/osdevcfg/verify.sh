#!/bin/sh
# 🟢 osdevcfg verify on a root (mirror or live after apply): `sh verify.sh <root>`. Law: the os-dev config suite passes under the official envelope with the scratch TMPDIR inside the root (base live: 2 failed, `expected 513 to be <= 40` and the browser walk missing ShellHost/theme because workspace `exports` subpaths were not followed). Needs the runner batch `nodecast` only for the contribution suite, which this verify does not run. On a mirror one test (`resolves source roots ... ✏️s`) fails because the mirror has no `✏️s` tree; pass `MIRROR=1` to tolerate exactly that.
set -u
ROOT="$1"; C=$(cd "$(dirname "$0")/../../.." && pwd); ART="$C/🗑️generated/osreds/v"; mkdir -p "$ART" "$ROOT/.osr/tmp"
cd "$ROOT" && TMPDIR="$ROOT/.osr/tmp" OSR_ROOT="$ROOT" OSR_ART="$ART" bun "$C/osreds-tools/vt-env.ts" '🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript' '../../🧪️tests/🎚️config/🟦️.ts' '../../🧪️tests/🧹️config/🟦️.ts' > "$ART/osdevcfg.txt" 2>&1
grep -E "Test Files|Tests  " "$ART/osdevcfg.txt"
if [ "${MIRROR:-0}" = 1 ]; then ! grep -E '^ *×' "$ART/osdevcfg.txt" | grep -qv 'resolves source roots that cover the products'; else ! grep -qE "Tests  .*failed" "$ART/osdevcfg.txt"; fi; r=$?; rm -f "$ART/osdevcfg.txt"; exit $r
