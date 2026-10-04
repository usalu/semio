#!/bin/zsh
# 🔁️ Session-4 puzzle-only describe + activation (design §21.4: the dev registry offers re-described plugins only): snapshot the committed
# `.vscode/launch.json`, run `describe` + `materialize-dev` of the puzzle composition, then the React (6012) + wgpu (6112) activation targets;
# logs `🗑️generated/e2e/describe-activate-s4-<n>.log`, exit code in `.exit`, one event line in `activate-retry.events`.
setopt no_bg_nice
root="/Users/ueli/Documents/semio"
dir="${0:A:h}/🗑️generated/e2e"
mkdir -p "$dir"
n="${1:-1}"
log="$dir/describe-activate-s4-$n.log"
nx=(bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx)
cp "$root/.vscode/launch.json" "$dir/launch-before-s4-$n.json"
(
  echo "== describe $(date '+%F %T')"
  (cd "$root" && $nx run-many -t describe materialize-dev -p @semio-tech/puzzle-plugin --parallel=1) || { echo "describe failed"; exit 11; }
  echo "== activate $(date '+%F %T')"
  (cd "$root" && $nx run-many -t activate-puzzle2d-react-dev activate-puzzle2d-wgpu-dev -p @semio-tech/framework-os-dev --parallel=2) || { echo "activation failed"; exit 12; }
  echo "== done $(date '+%F %T')"
) > "$log" 2>&1
code=$?
echo "$code" > "$dir/describe-activate-s4-$n.exit"
echo "$(date '+%F %T') s4 describe+activate attempt=$n exit=$code log=${log:t}" >> "$dir/activate-retry.events"
exit $code
