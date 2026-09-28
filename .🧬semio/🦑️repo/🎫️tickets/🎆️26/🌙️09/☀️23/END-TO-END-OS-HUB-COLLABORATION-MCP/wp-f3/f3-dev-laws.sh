#!/bin/zsh
# 🧪️ F3 session 14c — os-dev perf laws: boot-budget oracle drift, tsc of the boot/latency harness closure, vitest of the
# boot-budget and interaction-latency reducer laws (schema-validated fixtures). Native lane. usage: zsh f3-dev-laws.sh <tag>
TAG="${1:?tag}"
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated
DEV="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && python3 f3-boot-budget-oracle.py --check > "$OUT/oracle-check-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/oracle-check-$TAG.txt"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-f3 && nice -n 15 /Users/ueli/Documents/semio/node_modules/.bin/tsc -p tsc/tsconfig-dev-laws.json > "$OUT/tsc-dev-laws-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/tsc-dev-laws-$TAG.txt"
cd "$DEV" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🥾️boot-budget" "⏱️interaction-latency" > "$OUT/test-dev-laws-$TAG.txt" 2>&1; echo "rc=$?" >> "$OUT/test-dev-laws-$TAG.txt"
