#!/bin/zsh
# 🔁️ Session-4 single activation of puzzle 2d for React (6012) and wgpu (6112): one `nx run-many` of both activation targets from the live
# tree; log `🗑️generated/e2e/activate-s4-<n>.log`, exit code in `activate-s4-<n>.exit`, one event line in `activate-retry.events`.
setopt no_bg_nice
root="/Users/ueli/Documents/semio"
dir="${0:A:h}/🗑️generated/e2e"
mkdir -p "$dir"
n="${1:-1}"
log="$dir/activate-s4-$n.log"
(cd "$root" && bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run-many -t activate-puzzle2d-react-dev activate-puzzle2d-wgpu-dev -p @semio-tech/framework-os-dev --parallel=2) > "$log" 2>&1
code=$?
echo "$code" > "$dir/activate-s4-$n.exit"
echo "$(date '+%F %T') s4 attempt=$n activation exit=$code log=${log:t}" >> "$dir/activate-retry.events"
exit $code
