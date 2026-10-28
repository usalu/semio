#!/bin/sh
# 🕸️ verify.sh <root>: batch `wasmlock` on a mirror (or any root holding the batch): parse check, the bun law file, the independent Python oracle, and the policy call site.
# The real-repo law lists project manifests with `git ls-files` in <root>; a mirror without .git gets a throwaway index (`git init` + `git add` inside the mirror only).
mkdir -p "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/playdemo"; R="${1:?usage: verify.sh <root>}"; C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; T="$C/playdemo-tools"; rc=0
[ -d "$R/.git" ] || [ "$R" = "/Users/ueli/Documents/semio" ] || { (cd "$R" && git init -q && git add -A -- '*📋️project.json' '*Cargo.toml' '*Cargo.lock' >/dev/null 2>&1); }
printf '%s\n' "$L/⚡️caching/🦀️cargo/🕸️wasm-bindgen/🟦️.ts" "$L/⚡️caching/🧪️tests/🕸️wasm-bindgen-locks/🟦️.ts" "$L/🏃️process/🕸️wasm/🔣️policy/🟦️.ts" > "$C/🗑️generated/playdemo/scan.list"
(cd "$R" && bun "$C/gate-g-tools/scan.ts" "$C/🗑️generated/playdemo/scan.list" "$C/🗑️generated/playdemo/scan") || rc=1
mkdir -p "$C/🗑️generated/playdemo/art"
(cd "$R" && SEMIO_TEST_ARTIFACT_DIR="$C/🗑️generated/playdemo/art" bun test "./$L/⚡️caching/🧪️tests/🕸️wasm-bindgen-locks/🟦️.ts" 2>&1 | tail -12) | tee "$C/🗑️generated/playdemo/law.txt"
/usr/bin/grep -q " 0 fail" "$C/🗑️generated/playdemo/law.txt" || rc=1
python3 -I "$T/oracle.py" "$R" > "$C/🗑️generated/playdemo/oracle.txt" || rc=1; tail -3 "$C/🗑️generated/playdemo/oracle.txt"
/usr/bin/grep -q 'requireWasmBindgenSourceLockV1(root, relative(root, resolve(cwd))' "$R/$L/🏃️process/🕸️wasm/🔣️policy/🟦️.ts" || { echo "[DEBUG] policy call site missing"; rc=1; }
echo "[DEBUG] verify wasmlock rc=$rc"; exit $rc
