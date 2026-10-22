#!/bin/sh
# Re-applies the whole batch to a fresh copy of the pristine tree P and writes the result summary to OUT.
P=/private/tmp/runtime-seam/p3; W=/private/tmp/runtime-seam/w3; OUT="$1"
rm -rf "$W"; cp -cR "$P" "$W"
(cd "$W" && /usr/bin/grep -rlE "Bun|bun:|\"bun\"|'bun'|versions\.bun|import\.meta\.(dir|path|main)" --include='*.ts' --include='*.tsx' --include='*.mjs' --include='*.cjs' --include='*.js' --include='*.mts' 🧰️framework 2>/dev/null | sort > /private/tmp/runtime-seam/candidates.txt)
cd "$(dirname "$0")" && RUNTIME_SEAM_ROOT="$P" python3 manual-gen.py > /dev/null && python3 place.py > /dev/null && nice -n 10 bun apply.ts --root "$W" --candidates /private/tmp/runtime-seam/candidates.txt --write --no-backup --report "$OUT.json" > "$OUT" 2>&1
echo "EXIT $?" >> "$OUT"
