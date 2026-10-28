#!/bin/sh
# 🌐 Laws of batch `planglob`: sh verify.sh <root>   (mirror: python3 -I C/mirror-tools/mirror.py run <name> --seal-allow <C-relative 🗑️generated/audit6> -- sh <this file> <mirror-root>)
# Applies the batch, then runs bootreg's planmemo verify (the memo law with real nightly Cargo, now 14 cases incl. `workspace glob member`, + 27 capture/units/testing/launcher/unit-cache suites against the recorded pre-existing failure set).
# RED_ON_BASE=1 (mirror only): first restores the base memo module + schema with the new law and requires the law to fail on `workspace glob member`.
set -u
root=$(cd "${1:?usage: verify.sh <root>}" && pwd -P); here=$(cd "$(dirname "$0")" && pwd); C=$(cd "$here/../../.." && pwd)
A="$C/🗑️generated/audit6/planglob"; mkdir -p "$A"
LAW=./🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🧪️tests/🟦️.ts
python3 -I "$here/stage.py" apply --root "$root" >/dev/null || exit 2
if [ "${RED_ON_BASE:-0}" = 1 ]; then
  for rel in "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🟦️.ts" "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🧬️schema/🔣️.json"; do
    file=$(python3 -I -c 'import json,sys;print(json.load(open(sys.argv[1],encoding="utf8"))[sys.argv[2]]["file"])' "$here/manifest.json" "$rel"); cp "$here/base/$file" "$root/$rel"; done
  out=$(cd "$root" && SEMIO_TEST_ARTIFACT_DIR="$A" bun test --timeout 300000 "$LAW" 2>&1 | grep -v '^\[script\]')
  echo "$out" | grep -q '"workspace glob member",' && echo "$out" | grep -qE "^ *1 fail$" || { echo "VERIFY FAIL: the glob law is not red on base"; exit 1; }
  echo "RED ON BASE: OK"; python3 -I "$here/stage.py" apply --root "$root" >/dev/null 2>&1; for rel in "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🟦️.ts" "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🧬️schema/🔣️.json"; do file=$(python3 -I -c 'import json,sys;print(json.load(open(sys.argv[1],encoding="utf8"))[sys.argv[2]]["file"])' "$here/manifest.json" "$rel"); cp "$here/files/$file" "$root/$rel"; done
fi
SEMIO_TEST_ARTIFACT_DIR="$A" sh "$C/bootreg-tools/staged/planmemo/verify.sh" "$root" 2>&1 | grep -v '^\[script\]' | tail -4
