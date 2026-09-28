#!/bin/zsh
# 🪟️ R10 window-3 runner — one step per call, strictly serial (preamble 14 item 1): apply one kernel-derive input with
# `window3-apply.ts`, then prove it: taxonomy load probe, one `serve s react dev` boot to Home (`serve-boot-probe.ts`),
# and after `discovery` / `render` the registry launch law blocks (targets + seed make launch.json stale until render). Every output lands in the durable log directory.
# usage: zsh window3-run.sh <taxonomy|discovery|targets|seed|render|plan> [--dry-run]
#        zsh window3-run.sh refresh   (read-only: re-probes the scopes against the current candidate, for directories created since)
step="$1"; mode="--apply"; [ "$2" = "--dry-run" ] && mode=""
if [ "$step" = "refresh" ]; then
  cd /Users/ueli/Documents/semio/.tmp-ticket/wp-r10 || exit 2
  bun window3-apply.ts taxonomy > generated/window3-refresh-candidate.txt 2>&1 || exit 1
  candidate="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/END-TO-END-OS-HUB-COLLABORATION-MCP/wp-r10/generated/taxonomy.window3.json"
  bun taxonomy-kinds.ts --taxonomy "$candidate" --json generated/tax-kinds-window3.json "🌎️hub" "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp" "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry" "🧰️framework/🔨️modules/🗜️deflate" "🧰️framework/🛍️products/💻️os/🖥️host" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests" "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel"
  exit $?
fi
here="/Users/ueli/Documents/semio/.tmp-ticket/wp-r10"
logs="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-r10-logs"
registry="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"
stamp="$(date '+%H%M%S')"
log="$logs/window3-$step-$stamp.txt"
mkdir -p "$logs"
{
  echo "[window3-run] $step $mode $(date '+%H:%M:%S')"
  bun "$here/window3-apply.ts" "$step" $mode || { echo "[window3-run] APPLY FAILED"; exit 1; }
  [ -z "$mode" ] && { echo "[window3-run] dry run only"; exit 0; }
  echo "[window3-run] taxonomy load probe"
  (cd "$here/../wp-coord" && bun taxonomy-load-probe.ts) || { echo "[window3-run] TAXONOMY INVALID — revert this step"; exit 2; }
  if [ "$step" = "taxonomy" ]; then
    echo "[window3-run] static candidate diff + chain registry check"
    python3 "$here/taxonomy-diff-check.py" "$here/generated/window3-backups/$(ls "$here/generated/window3-backups" | /usr/bin/grep '^taxonomy-' | sort | tail -1)/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json" "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json" || { echo "[window3-run] TAXONOMY DIFF NOT ADDITIVE — revert this step"; exit 2; }
    (cd /Users/ueli/Documents/semio && NX_DAEMON=false bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run @semio-tech/plugin-registry:check --skip-nx-cache --outputStyle=stream) || { echo "[window3-run] REGISTRY CHECK RED — revert this step"; exit 4; }
  fi
  if [ "$step" = "discovery" ] || [ "$step" = "render" ]; then
    echo "[window3-run] registry launch laws"
    (cd "$registry" && bun /Users/ueli/Documents/semio/node_modules/vitest/vitest.mjs run --config "🧪️tests/🎚️config/🟦️.ts" --testTimeout 900000 --hookTimeout 900000 "🧪️tests/🚀️launch/🟦️.ts" -t "launch configuration identity|declared project targets") || echo "[window3-run] LAUNCH LAWS RED"
  fi
  if [ "$step" != "plan" ]; then
    echo "[window3-run] serve boot to Home"
    bun "$here/serve-boot-probe.ts" --port 6620 || { echo "[window3-run] SERVE BOOT FAILED — revert this step"; exit 3; }
  fi
  echo "[window3-run] $step DONE $(date '+%H:%M:%S')"
} > "$log" 2>&1
rc=$?
echo "$log rc=$rc"
exit $rc
