#!/bin/sh
# 🧲️ [incfacts] verifies batch `libfacts` on a root (a mirror, or live after landing): parse scan of the touched TS/MJS, the unit laws of the touched components under the pinned bun (failure SET compared, never counts), then the `⚡️incremental` observation law beneath a genuine `setup` birth with the root's Nx SDK identity (`.nx/installation`) and an artifact dir inside the root (the dataset schema refuses a source root outside the workspace). usage: sh verify.sh <root> [--no-birth]; outputs in C/🗑️generated/incfacts/
R="$(cd "$1" && pwd -P)"; shift; C="$(cd "$(dirname "$0")/../../.." && pwd)"; G="$C/🗑️generated/incfacts"; mkdir -p "$G"
want=$(sed -n 's/.*"packageManager": "bun@\([^"]*\)".*/\1/p' "$R/package.json"); B="${BUN:-}"; [ -n "$B" ] || for b in "$HOME"/Library/Caches/semio/runtimes/bun/*/bun "$HOME"/.cache/semio/runtimes/bun/*/bun; do [ -x "$b" ] && [ "$("$b" --version)" = "$want" ] && B="$b" && break; done; [ -n "$B" ] || { echo "pinned bun $want not found (set BUN)"; exit 2; }
I="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference"; fail=0
P="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference"; printf '%s\n' "$P/🧱️project-assembly/🟨️.mjs" "$P/🧱️project-assembly/🧪️tests/🟦️.ts" "$P/🧭️script-inputs/🟨️.mjs" "$P/🧬️source-analyses/🟨️.mjs" "$P/♻️revision/🟨️.mjs" > "$G/scan-list.txt"
(cd "$R" && "$B" "$C/gate-g-tools/scan.ts" "$G/scan-list.txt" "$G/scan") > "$G/scan.txt" 2>&1 || { echo "SCAN FAIL"; head "$G/scan.txt"; fail=1; }; printf 'scan: %s\n' "$(tail -1 "$G/scan.txt")"
for t in "🧱️project-assembly/🧪️tests" "🔖️stamps/🧪️tests" "🧪️tests/🔖️output-stamps" "🧪️tests/🙈️presentation-resources" "♻️revision/🧪️tests" "../../🧪️tests/🕸️graph/🎯️source-demand"; do
  slug=$(printf '%s' "$t" | shasum | cut -c1-8); (cd "$I/$t" && SEMIO_TEST_WORKSPACE_ROOT="$R" nice -n 10 "$B" test --timeout 170000 ./🟦️.ts) > "$G/law-$slug.txt" 2>&1
  printf '%s: %s\n' "$t" "$(grep -E '^ *[0-9]+ (pass|fail)$' "$G/law-$slug.txt" | tr -s ' \n' ' ')"; grep -E '^\(fail\)' "$G/law-$slug.txt" | cut -c1-200
done
[ "${1:-}" = "--no-birth" ] && exit $fail
O="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/⚡️incremental/🟦️.ts"
mkdir -p "$R/.tmp/if"; (cd "$R" && SEMIO_TEST_ARTIFACT_DIR="$R/.tmp/if" nice -n 10 "$B" "$C/td-observe-tools/sdkdrive.ts" "$R" "$O" setup) > "$G/incremental.txt" 2>&1
printf 'incremental (setup birth): %s\n' "$(grep -E '^ *[0-9]+ (pass|fail)$' "$G/incremental.txt" | tr -s ' \n' ' ')"; grep -E '^\(fail\)|foreignLibraryFacts|Expected|Received' "$G/incremental.txt" | cut -c1-200 | head -12
grep -qE '^ *0 fail$' "$G/incremental.txt" || fail=1
exit $fail
