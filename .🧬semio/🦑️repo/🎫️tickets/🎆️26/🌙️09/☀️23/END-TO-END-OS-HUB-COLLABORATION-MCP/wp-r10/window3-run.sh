#!/bin/zsh
# 🪟️ R10 window-3 runner — one step per call, strictly serial (preamble 14 item 1): apply one kernel-derive input with
# `window3-apply.ts`, then prove it: taxonomy load probe, one `serve s react dev` boot to Home (`serve-boot-probe.ts`),
# and after `discovery` / `render` the registry launch law blocks (targets + seed make launch.json stale until render). Every output lands in the durable log directory.
# Preamble rule 25: the chain registry check and the serve boot wait until every build lane is idle and the 1-min load < 32
# (`--probe-only` skips both: taxonomy-load probe + static diff only; `verify` runs them later). State: `.🧬semio/🌐hub/s14-r10-state`.
# usage: zsh window3-run.sh <taxonomy|discovery|targets|seed|render|plan|st2-r10> [--dry-run|--probe-only]
#        zsh window3-run.sh refresh   (read-only: re-probes the scopes against the current candidate, for directories created since)
#        zsh window3-run.sh verify    (registry check + launch laws + boot, after a --probe-only landing)
step="$1"; mode="--apply"; [ "$2" = "--dry-run" ] && mode=""; probe_only=""; [ "$2" = "--probe-only" ] && probe_only=1
if [ "$step" = "refresh" ]; then
  cd /Users/ueli/Documents/semio/.tmp-ticket/wp-r10 || exit 2
  state="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-r10-state"
  bun window3-apply.ts taxonomy > "$state/window3-refresh-candidate.txt" 2>&1 || exit 1
  bun taxonomy-kinds.ts --taxonomy "$state/taxonomy.window3.json" --json "$state/tax-kinds-window3.json" "🌎️hub" "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp" "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry" "🧰️framework/🔨️modules/🗜️deflate" "🧰️framework/🛍️products/💻️os/🖥️host" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests" "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel"
  exit $?
fi
here="/Users/ueli/Documents/semio/.tmp-ticket/wp-r10"
state="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-r10-state"
logs="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-r10-logs"
registry="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"
stamp="$(date '+%H%M%S')"
log="$logs/window3-$step-$stamp.txt"
mkdir -p "$logs"
stdio_scopes=("✏️s/🔌️plugins/🗄️stdio/🧩️extensions" "✏️s/🔌️plugins/🗄️stdio/🧩️composition" "✏️s/🔌️plugins/🗄️stdio/🏘️composition")
lanes_idle() {
  while [ -d /tmp/semio-native-build.lock ] || [ -d /tmp/semio-wasm-build.lock ] || [ -d /tmp/semio-overlay-build.lock ] || [ "$(sysctl -n vm.loadavg | awk '{print int($2)}')" -ge 32 ]; do sleep 30; done
  echo "[window3-run] lanes idle, load $(sysctl -n vm.loadavg) $(date '+%H:%M:%S')"
}
registry_check() {
  (cd /Users/ueli/Documents/semio && NX_DAEMON=false bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run @semio-tech/plugin-registry:check --skip-nx-cache --outputStyle=stream)
}
launch_laws() {
  (cd "$registry" && bun /Users/ueli/Documents/semio/node_modules/vitest/vitest.mjs run --config "🧪️tests/🎚️config/🟦️.ts" --testTimeout 900000 --hookTimeout 900000 "🧪️tests/🚀️launch/🟦️.ts" -t "launch configuration identity|declared project targets")
}
{
  if [ "$step" = "verify" ]; then
    lanes_idle
    registry_check || { echo "[window3-run] REGISTRY CHECK RED"; exit 4; }
    launch_laws || echo "[window3-run] LAUNCH LAWS RED"
    bun "$here/serve-boot-probe.ts" --port 6620 || { echo "[window3-run] SERVE BOOT FAILED"; exit 3; }
    echo "[window3-run] verify DONE $(date '+%H:%M:%S')"
    exit 0
  fi
  if [ "$step" = "st2-r10" ] && [ -n "$mode" ]; then
    (cd "$here" && bun taxonomy-kinds.ts --json "$state/probe-st2-r10-before.json" "${stdio_scopes[@]}") | /usr/bin/grep "scope="
  fi
  echo "[window3-run] $step $mode $(date '+%H:%M:%S')"
  bun "$here/window3-apply.ts" "$step" $mode || { echo "[window3-run] APPLY FAILED"; exit 1; }
  [ -z "$mode" ] && { echo "[window3-run] dry run only"; exit 0; }
  echo "[window3-run] taxonomy load probe"
  (cd "$here/../wp-coord" && bun taxonomy-load-probe.ts) || { echo "[window3-run] TAXONOMY INVALID — revert this step"; exit 2; }
  if [ "$step" = "taxonomy" ]; then
    echo "[window3-run] static candidate diff"
    python3 "$here/taxonomy-diff-check.py" "$state/window3-backups/$(ls "$state/window3-backups" | /usr/bin/grep '^taxonomy-' | sort | tail -1)/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json" "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json" || { echo "[window3-run] TAXONOMY DIFF NOT ADDITIVE — revert this step"; exit 2; }
  fi
  if [ "$step" = "st2-r10" ]; then
    echo "[window3-run] new unresolved directories of the set → taxonomy, then launch.json from the generator"
    (cd "$here" && bun taxonomy-kinds.ts --json "$state/probe-st2-r10-after.json" "${stdio_scopes[@]}") | /usr/bin/grep "scope="
    python3 -c "import json,sys; before={r['path'] for r in json.load(open(sys.argv[1]))}; rows=[r for r in json.load(open(sys.argv[2])) if r['path'] not in before]; json.dump(rows, open(sys.argv[3],'w'), ensure_ascii=False, indent=1); print('[window3-run] st2-r10 new unresolved:', len(rows)); [print('  ', r['path'], r['parentKind']) for r in rows]" "$state/probe-st2-r10-before.json" "$state/probe-st2-r10-after.json" "$state/tax-kinds-st2-r10.json"
    bun "$here/window3-apply.ts" taxonomy --apply || { echo "[window3-run] TAXONOMY FAILED — revert st2-r10 + taxonomy"; exit 2; }
    (cd "$here/../wp-coord" && bun taxonomy-load-probe.ts) || { echo "[window3-run] TAXONOMY INVALID — revert st2-r10 + taxonomy"; exit 2; }
    bun "$here/window3-apply.ts" render --apply || { echo "[window3-run] RENDER FAILED — revert st2-r10 + taxonomy + render"; exit 1; }
  fi
  [ -n "$probe_only" ] && { echo "[window3-run] $step DONE (probe only; run verify when the lanes are idle) $(date '+%H:%M:%S')"; exit 0; }
  if [ "$step" = "taxonomy" ] || [ "$step" = "st2-r10" ]; then
    lanes_idle
    echo "[window3-run] chain registry check"
    registry_check || { echo "[window3-run] REGISTRY CHECK RED — revert this step"; exit 4; }
  fi
  if [ "$step" = "discovery" ] || [ "$step" = "render" ] || [ "$step" = "st2-r10" ]; then
    echo "[window3-run] registry launch laws"
    launch_laws || echo "[window3-run] LAUNCH LAWS RED"
  fi
  if [ "$step" != "plan" ]; then
    lanes_idle
    echo "[window3-run] serve boot to Home"
    bun "$here/serve-boot-probe.ts" --port 6620 || { echo "[window3-run] SERVE BOOT FAILED — revert this step"; exit 3; }
  fi
  echo "[window3-run] $step DONE $(date '+%H:%M:%S')"
} > "$log" 2>&1
rc=$?
echo "$log rc=$rc"
exit $rc
