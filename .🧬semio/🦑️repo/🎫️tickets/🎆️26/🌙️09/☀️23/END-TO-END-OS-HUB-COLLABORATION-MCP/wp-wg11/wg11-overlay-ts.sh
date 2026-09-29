#!/bin/zsh
# 🧪️ WG11 session 14d: the TypeScript halves of WG11's sets, run in WG11's overlay (sets applied by the last overlay dev hold, node_modules
# mirrored from the live install) with the React engine package's own vitest config. Lines prefixed [wg11-ts].
# usage: zsh wg11-overlay-ts.sh > <capture> 2>&1
set -u
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-wg11-overlay"
PKG="$O/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
CONFIG="../../🧪️tests/🎚️config/🟦️.ts"
export NX_DAEMON=false
R=/Users/ueli/Documents/semio
echo "[wg11-ts] mirroring ignored wasm bindings (🕸️bindings) into the overlay $(date '+%F %T')"
( cd "$R" && git ls-files --others --ignored --exclude-standard -- 🧰️framework | /usr/bin/grep "/🕸️bindings/" | while IFS= read -r f; do mkdir -p "$O/$(dirname "$f")" && cp -c "$R/$f" "$O/$f" 2>/dev/null; done )
cd "$PKG" || exit 1
echo "[wg11-ts] display ids + React window laws $(date '+%F %T')"
SEMIO_TEST_LEVEL=standard nice -n 15 bunx vitest run --config "$CONFIG" --reporter=dot "📌️ChromePanels/🧪️tests/🧩️component" "🪟️window-lifecycle-template-drag" 2>&1 | tail -25
echo "[wg11-ts] rc=$? $(date '+%F %T')"
echo "[wg11-ts] engine-contract Display + delivery $(date '+%F %T')"
SEMIO_TEST_LEVEL=standard nice -n 15 bunx vitest run --config "$CONFIG" --reporter=dot "🔬️engine-contract" -t "window kind|world-3d window kind|pre-reverses|drag payload decodes|delivery|lighting fixture" 2>&1 | tail -25
echo "[wg11-ts] rc=$? $(date '+%F %T')"
echo "[wg11-ts] Interpreter tree windows (in-source) $(date '+%F %T')"
SEMIO_TEST_LEVEL=long nice -n 15 bunx vitest run --config "$CONFIG" --reporter=dot "🗣️Interpreter/🟦️.tsx" -t "neutral viewport vectors|a window the guest answers short" 2>&1 | tail -25
echo "[wg11-ts] rc=$? $(date '+%F %T')"
echo "[wg11-ts] text corpus + navbar band + GraphTimeline layout (Chromium) $(date '+%F %T')"
SEMIO_TEST_LEVEL=standard nice -n 15 bunx vitest run --config "$CONFIG" --reporter=dot "🔤️text-advances" "🔝️navbar-centered-band" "🌳️GraphTimelineHost/🧪️tests/🎨️layout" 2>&1 | tail -25
echo "[wg11-ts] rc=$? $(date '+%F %T')"
