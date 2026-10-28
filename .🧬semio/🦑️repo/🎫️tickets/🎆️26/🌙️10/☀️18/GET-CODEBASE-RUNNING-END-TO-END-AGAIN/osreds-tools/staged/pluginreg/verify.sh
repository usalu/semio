#!/bin/sh
# 🟢 pluginreg verify on a root (mirror or live after apply): `sh verify.sh <root>`. Law: the plugin-registry host-activation and catalog-complete suites pass under the official envelope (base: host-activation `ERR_VM_DYNAMIC_IMPORT_CALLBACK_MISSING`, then 0 component outputs; catalog-complete needs the runner batch `nodecast` for its two Node strip-types failures).
set -u
ROOT="$1"; C=$(cd "$(dirname "$0")/../../.." && pwd); ART="$C/🗑️generated/osreds/v"; mkdir -p "$ART" "$ROOT/.osr/tmp"
cd "$ROOT" && TMPDIR="$ROOT/.osr/tmp" OSR_ROOT="$ROOT" OSR_ART="$ART" bun "$C/osreds-tools/vt-env.ts" '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry' './🧪️tests/🎚️config/🟦️.ts' './🧪️tests/🎬️host-activation/🟦️.ts' './🧪️tests/✅️catalog-complete/🟦️.ts' > "$ART/pluginreg.txt" 2>&1
grep -E "Test Files|Tests  " "$ART/pluginreg.txt"; grep -cE "Tests  [0-9]+ passed \(" "$ART/pluginreg.txt" | grep -q '^1$' && ! grep -qE "failed" "$ART/pluginreg.txt"; r=$?; rm -f "$ART/pluginreg.txt"; exit $r
