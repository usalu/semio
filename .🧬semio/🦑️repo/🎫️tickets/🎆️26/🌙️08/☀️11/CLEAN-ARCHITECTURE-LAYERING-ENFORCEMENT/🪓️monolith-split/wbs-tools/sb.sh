#!/bin/zsh
# usage: sb.sh <name> <root> <config> [mode] [secs]  (stubbed-wasm vite build)
name=$1; root=$2; cfg=$3; mode=${4:-prototype}; secs=${5:-900}
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cache=/Users/ueli/Documents/semio/.cache/b/p/m/wbs-cache/$name; mkdir -p $cache
log="$F/🗑️generated/wbs-$name.log"
cd "$root" || exit 2
SEMIO_BUILD_MODE=$mode SEMIO_VITE_CACHE_DIRECTORY=$cache nice -n 10 perl -e 'alarm shift; exec @ARGV' -- $secs bun "$F/wbs-tools/stub-build.mjs" $name "$root" "$cfg" $mode > "$log" 2>&1
echo "exit=$?" >> "$log"; tail -3 "$log"
