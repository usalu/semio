#!/usr/bin/env bash
# 🌐️ Probe one queue-produced guest during release optimization.
set -euo pipefail
ticket="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$ticket/../../../../../../.." && pwd)"
generated="$ticket/🗑️generated"
module="${1:?component id}"; run="${2:?queue run id}"; port="${3:?local port}"
case "${4:-en}" in
  en|de) projects=(--project "${4:-en}") ;;
  both) projects=(--project en --project de) ;;
  *) exit 2 ;;
esac
case "$module" in
  energy) pane=energie ;;
  procedural) pane=generator ;;
  demonstrator) pane='koordinator|aggregator|aussuchen|bearbeiten|verfolgen' ;;
  fem) pane=statik ;;
  *) exit 2 ;;
esac
runs="$root/.cache/b/r/m/components/runs"
intermediate="$runs/$run/w/semio-hub-$module-component/transpiled"
base="semio_hub_${module}_component"
if [[ -z "${PROBE_JS:-}" || -z "${PROBE_CORE:-}" ]]; then
  while [[ ! -d "$runs/$run/w/semio-hub-$module-component/optimized" || ! -s "$intermediate/$base.d.ts" || ! -s "$intermediate/$base.js" || ! -s "$intermediate/$base.core.wasm" ]]; do
    if [[ -f "$runs/$run/r/units.json" && -f "$runs/$run/r/report.md" ]]; then
      intermediate="$(python3 "$ticket/probe-transpile-cache.py" "$runs/$run" "$module")" || exit 1
      break
    fi
    if [[ ! -d "$runs/$run" ]] || grep -q '^\[native-lanes\] step exited [0-9][0-9]*$' "$generated/browser-$module.log" || ! grep -Fq "runs/$run:" "$generated/browser-$module.log"; then echo "Run $run ended without a probe input"; exit 1; fi
    sleep 5
  done
fi
site="$(python3 "$ticket/captured-site.py" "$root" "$generated/native-site.log")"
modules="$root/.cache/b/p/m/browser/release/modules"
stage=( "$modules/"*️"$module" )
folder="$(basename "${stage[0]}")"
probe="$generated/probe-$run-$$/$folder"; mkdir -p "$probe"
node "$ticket/probe-custody.mjs" copy "${PROBE_JS:-$intermediate/$base.js}" "$probe/$base.js"
if [[ -n "${PROBE_CORE:-}" ]]; then
  node "$ticket/probe-custody.mjs" copy "$PROBE_CORE" "$probe/$base.core.wasm"
else
  for file in "$intermediate/"*.wasm; do node "$ticket/probe-custody.mjs" copy "$file" "$probe/$(basename "$file")"; done
fi
rewrite="$root/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts"
runner=(repo tools exec --light bun --)
if [[ -n "${PROBE_BUN:-}" ]]; then runner=("$PROBE_BUN"); fi
material="$root/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization"
if [[ "${PROBE_REWRITTEN:-0}" != 1 ]]; then
  chmod u+w "$probe/$base.js"
  "${runner[@]}" -e 'const [lib,js,vendor]=process.argv.slice(1);const {rewritePreview2ShimImports}=await import(lib);rewritePreview2ShimImports(js,vendor)' "$rewrite" "$probe/$base.js" "$generated/probe-$run-$$/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim"
  chmod a-w "$probe/$base.js"
fi
"${runner[@]}" -e 'const [dir,out,base]=process.argv.slice(1);const {pluginComponentBridgeSource}=await import(dir+"/🌉️component-bridge-source/🟦️.ts");const {hostShimSource}=await import(dir+"/🪢️host-shim-source/🟦️.ts");await Bun.write(out+"/🌉️bridge.js",pluginComponentBridgeSource(base,base+".wasm"));await Bun.write(out+"/🟨️.js",hostShimSource())' "$material" "$probe" "$base"
chmod a-w "$probe/🌉️bridge.js" "$probe/🟨️.js"
node "$ticket/overlay-serve.mjs" "$site" "$modules" "$port" "$folder" "$probe" > "$generated/$run-server.log" 2>&1 & server=$!
trap 'kill "$server" 2>/dev/null || true' EXIT
while ! curl -sf "http://127.0.0.1:$port/" >/dev/null; do kill -0 "$server" || exit 1; sleep 0.25; done
kill -0 "$server" || exit 1
for file in "$probe/$base.js" "$probe/"*.wasm "$probe/🌉️bridge.js" "$probe/🟨️.js"; do
  asset="$(basename "$file")"
  url="$(node -e 'console.log(process.argv[1].split("/").map(encodeURIComponent).join("/"))' "🔌️plugin-modules/$folder/$asset")"
  node "$ticket/probe-custody.mjs" verify "$probe/$asset" "http://127.0.0.1:$port/$url"
done
cd "$root/♻️mit-bestand/🧺️demonstrator"
cli="$(dirname "$(node -p 'require.resolve("@playwright/test/package.json")')")/cli.js"
repo capacity run --memory=4096 -- env DEMONSTRATOR_VERIFY_CDN=1 PLAYWRIGHT_BASE_URL="http://cdn.localhost:$port" PLAYWRIGHT_BROWSERS_PATH="$root/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright" \
  node "$cli" test --config "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts" --workers=1 "${projects[@]}" --grep "($pane) boot$" --output "$generated/probe-$run-$$-pw" --reporter=list
