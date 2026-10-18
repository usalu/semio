#!/bin/zsh
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; R="/Users/ueli/Documents/semio"; C="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript"
d=$R/.cache/b/p/m/wbs-next; mkdir -p $d/next
cat > $d/tsconfig.json <<JSON
{"extends":"$C/tsconfig.json","compilerOptions":{"baseUrl":"$C","strict":true,"incremental":true,"tsBuildInfoFile":"$d/next.tsbuildinfo"},"include":["$C/app/**/*.ts","$C/app/**/*.tsx","$C/lib/**/*.ts","$d/next-env.d.ts","$d/next/types/**/*.ts"],"exclude":["$C/node_modules","$C/📜️script.ts","$C/dist"]}
JSON
cd "$C" || exit 2
rel(){ node -e 'console.log(require("path").relative(process.argv[1],process.argv[2]))' "$C" "$1"; }
SEMIO_BUILD_MODE=prototype SEMIO_NEXT_BUILD_DIRECTORY=$(rel $d/next) SEMIO_NEXT_TYPESCRIPT_PROJECT=$(rel $d/tsconfig.json) NEXT_TELEMETRY_DISABLED=1 nice -n 10 perl -e 'alarm shift; exec @ARGV' -- 1500 node --experimental-transform-types $R/node_modules/next/dist/bin/next build > "$F/🗑️generated/wbs-next-coordinator.log" 2>&1
echo "exit=$?" >> "$F/🗑️generated/wbs-next-coordinator.log"; tail -3 "$F/🗑️generated/wbs-next-coordinator.log"
