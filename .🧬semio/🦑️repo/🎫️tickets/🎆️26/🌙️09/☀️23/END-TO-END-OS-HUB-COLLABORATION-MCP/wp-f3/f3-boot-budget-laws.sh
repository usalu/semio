#!/bin/zsh
# 🥾️ F3 — boot-budget harness proof: oracle drift check, tsc of the harness closure, the law vitest (os-dev config). Run
# inside the native lane. usage: zsh f3-boot-budget-laws.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
DEV="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && python3 f3-boot-budget-oracle.py --check > "$OUT/oracle-check-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/oracle-check-$TAG.txt"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 /Users/ueli/Documents/semio/node_modules/.bin/tsc -p tsc/tsconfig-boot-budget.json > "$OUT/tsc-boot-budget-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-boot-budget-$TAG.txt"
cd "$DEV" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🥾️boot-budget" > "$OUT/test-boot-budget-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-boot-budget-$TAG.txt"
