#!/bin/zsh
# usage: vb.sh <name> <bundleRoot(abs)> <config(abs)> [mode] [seconds]
name=$1; root=$2; cfg=$3; mode=${4:-prototype}; secs=${5:-900}
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
out=/Users/ueli/.cache/wbs/out/$name; cache=/Users/ueli/Documents/semio/.cache/b/p/m/wbs-cache/$name
mkdir -p $cache /Users/ueli/.cache/wbs/out
log="$F/🗑️generated/wbs-$name.log"
cd "$root" || exit 2
SEMIO_BUILD_MODE=$mode SEMIO_VITE_CACHE_DIRECTORY=$cache nice -n 10 perl -e 'alarm shift; exec @ARGV' -- $secs \
  bun /Users/ueli/Documents/semio/node_modules/vite/bin/vite.js build --mode $mode --minify false --config "$cfg" --configLoader ${WBS_LOADER:-runner} --outDir $out --emptyOutDir > "$log" 2>&1
echo "exit=$?" >> "$log"; tail -3 "$log"
