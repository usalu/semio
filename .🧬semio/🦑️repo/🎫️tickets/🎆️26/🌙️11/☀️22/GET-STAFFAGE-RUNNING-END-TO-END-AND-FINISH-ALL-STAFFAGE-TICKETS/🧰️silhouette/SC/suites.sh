#!/bin/zsh
# 🧾️ The other suites of the proof for components, each with a name filter and one after the other: suites.sh <log-prefix> <id>…  (three-views, catalog-physics, breathing-catalog and asset-power by asset id; asset-validation and the manifest tests of asset-power by their describe names)
source "${0:A:h}/env.sh"
p="$1"; shift
export SUFFIX='( |:|$)'
"$W/vt.sh" "$p-three-views" "🧊️three-views" "$@"
"$W/vt.sh" "$p-catalog-physics" "🌡️catalog-physics" "$@"
"$W/vt.sh" "$p-breathing-catalog" "🫁️breathing-catalog" "$@"
"$W/vt.sh" "$p-asset-power" "⚡️asset-power" "$@"
SUFFIX='' "$W/vt.sh" "$p-asset-validation" "✅️asset-validation" "asset validation" "flat elevation shape"
SUFFIX='' "$W/vt.sh" "$p-asset-power-all" "⚡️asset-power" "asset power"
