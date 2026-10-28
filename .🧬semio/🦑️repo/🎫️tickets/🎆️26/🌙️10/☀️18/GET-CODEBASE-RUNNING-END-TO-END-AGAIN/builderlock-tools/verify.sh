#!/bin/sh
# 👯 Laws of batch `multiwriter` after apply: sh verify.sh <repository or mirror root>  (exit 0 = VERIFY OK). Needs `bun` and `python3` on PATH.
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
B=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
G=$B/🕸️graph/🕰️generations
size=$(wc -c < "$G/🟨️.mjs" | tr -d ' '); [ "$size" -lt 65536 ] || { echo "generations/🟨️.mjs $size B >= 65536 consumer cap"; exit 1; }
echo "generations/🟨️.mjs $size B < 65536"
scratch=$(mktemp -d); list="$scratch/list"
python3 -I -c 'import json,sys;[print(r) for r,v in json.load(open(sys.argv[1],encoding="utf8")).items() if r.endswith((".ts",".mjs"))]' "$here/manifest.json" > "$list"
bun "$here/../../../gate-g-tools/scan.ts" "$list" "$scratch/scan" | tail -1
rm -rf "$scratch"
bun test "$G/🔐️custody/🧪️tests/🟦️.ts" "$G/🔀️merge/🧪️tests/🟦️.ts" "$G/🧪️tests/👯️multiwriter/🟦️.ts" "$G/🔐️custody/🧪️tests/🔑️kernel/🟦️.ts" "$G/🧩️claims/🧪️tests/🟦️.ts" "$G/🧩️claims/🧪️tests/🏃️processes/🟦️.ts"
bun test "$G/🧪️tests/🟦️.ts" "$G/🔌️load/🧪️tests/🟦️.ts" "$G/🔌️load/🧪️tests/🪑idle-builder/🟦️.ts" "$G/🔌️load/🧪️tests/🕊️early-release/🟦️.ts" "$G/🧩️claims/🧪️tests/💤️idle/🟦️.ts" "$G/🧪️tests/🪞️plan-once/🟦️.ts" "$B/🕸️graph/🧪️tests/🔼️upgrade-before-custody/🟦️.ts" "$B/🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts"
bun test "$B/📥️inputs/💾️storage/🧪️tests/🟦️.ts" -t "normal Graph state uses the same full resource"
bun test "$G/🔌️load/🧪️tests/🚧️withheld-dependency/🟦️.ts" -t "the full graph"
echo "VERIFY OK"
