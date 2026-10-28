#!/bin/sh
# ts2-g: one cold tsc of the course program under bun and under node, /usr/bin/time -l (CPU, peak RSS).
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript" || exit 1
TSC=/Users/ueli/Documents/semio/node_modules/typescript/bin/tsc
echo "== bun"; /usr/bin/time -l nice -n 10 bun "$TSC" --noEmit -p tsconfig.json 2>&1 | grep -E "real|user|sys|maximum resident|peak memory|error TS" | head -12
echo "== node"; /usr/bin/time -l nice -n 10 node "$TSC" --noEmit -p tsconfig.json 2>&1 | grep -E "real|user|sys|maximum resident|peak memory|error TS" | head -12
