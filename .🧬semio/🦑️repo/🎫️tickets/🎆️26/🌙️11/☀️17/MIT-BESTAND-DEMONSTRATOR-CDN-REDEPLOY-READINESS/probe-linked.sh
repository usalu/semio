#!/usr/bin/env bash
# 🌐️ Probe one queue-produced guest during release optimization.
set -euo pipefail
ticket="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$ticket/../../../../../../.." && pwd)"
generated="$ticket/🗑️generated"
module="${1:?component id}"; run="${2:?queue run id}"; port="${3:?local port}"
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
while [[ ! -s "$intermediate/$base.d.ts" || ! -s "$intermediate/$base.js" || ! -s "$intermediate/$base.core.wasm" ]]; do
  if [[ ! -d "$runs/$run" || -f "$runs/$run/r/report.md" ]] || ! grep -Fq "runs/$run:" "$generated/browser-$module.log"; then echo "Run $run ended without a probe input"; exit 1; fi
  sleep 5
done
site="$(python3 "$ticket/captured-site.py" "$root" "$generated/native-site.log")"
modules="$root/.cache/b/p/m/browser/release/modules"
stage=( "$modules/"*️"$module" )
folder="$(basename "${stage[0]}")"
probe="$generated/$run/$folder"; mkdir -p "$probe"
cp "$intermediate/$base.js" "$probe/"
for file in "$intermediate/"*.wasm; do ln -sf "$file" "$probe/$(basename "$file")" || cp "$file" "$probe/"; done
rewrite="$root/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts"
repo tools exec bun -- -e 'const [lib,js,vendor]=process.argv.slice(1);const {rewritePreview2ShimImports}=await import(lib);rewritePreview2ShimImports(js,vendor)' "$rewrite" "$probe/$base.js" "$generated/$run/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim"
node "$ticket/overlay-serve.mjs" "$site" "$modules" "$port" "$folder" "$probe" > "$generated/$run-server.log" 2>&1 & server=$!
trap 'kill "$server" 2>/dev/null || true' EXIT
while ! curl -sf "http://127.0.0.1:$port/" >/dev/null; do kill -0 "$server" || exit 1; sleep 0.25; done
kill -0 "$server" || exit 1
cd "$root/♻️mit-bestand/🧺️demonstrator"
cli="$(dirname "$(node -p 'require.resolve("@playwright/test/package.json")')")/cli.js"
DEMONSTRATOR_VERIFY_CDN=1 PLAYWRIGHT_BASE_URL="http://cdn.localhost:$port" PLAYWRIGHT_BROWSERS_PATH="$root/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright" \
  node "$cli" test --config "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts" --workers=1 --project en --grep "($pane) boot$" --output "$generated/$run-pw" --reporter=list
