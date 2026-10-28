#!/bin/sh
# 🕰 Laws of batch `freshmtime`: sh verify.sh <repository or mirror root> [red]
# 1. the freshness law (real Cargo + rustc, three lanes, Cargo's own -v verdict and the Python mtime oracle as independent oracles);
# 2. every unit-cache suite + the compiler lane suite: no failure. Suites run WITHOUT SEMIO_TEST_ARTIFACT_DIR (scratch workspaces must stay outside every checkout).
# With `red` (mirror only): restores the base store module first and expects the law to FAIL, then re-applies the batch.
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}; out="$here/../../../🗑️generated/edit30"; mkdir -p "$out"; unset SEMIO_TEST_ARTIFACT_DIR
law=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🕰️freshness/🟦️.ts
store=🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
if [ "${2:-}" = red ]; then cp "$here/base/00.ts" "$store"; if bun test "$law" > "$out/fm-red.out" 2>&1; then echo "RED CHECK FAILED: law passes on base"; cp "$here/files/00.ts" "$store"; exit 1; fi; echo "red on base: OK"; cp "$here/files/00.ts" "$store"; fi
bun test "$law"
bun test --timeout 300000 --reporter=junit --reporter-outfile="$out/fm-suites.xml" $(tr '\n' ' ' < "$here/suites.txt") > "$out/fm-suites.out" 2>&1 || true
python3 -I "$here/junit-set.py" "$out/fm-suites.xml" | tee "$out/fm-suites.set" | head -5
if grep -q '^FAIL' "$out/fm-suites.set"; then echo "suites: failures"; exit 1; fi
echo "VERIFY OK"
