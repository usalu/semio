#!/bin/zsh
# 🧪️ The other suites of objects, one after the other with a name filter (`🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog` per asset; `✅️asset-validation` and `⚡️asset-power` by their manifest and catalog-wide cases plus the asset): others.sh <log-prefix> <id>…
source "${0:A:h}/env.sh"
p="$1"; shift
"$W/vt.sh" "$p-three-views" 🧊️three-views "$@"
"$W/vt.sh" "$p-catalog-physics" 🌡️catalog-physics "$@"
"$W/vt.sh" "$p-breathing-catalog" 🫁️breathing-catalog "$@"
ids="(^|[ >])(${(j:|:)@})([ :]|$)"
PAT="$ids|manifest" "$W/vt.sh" "$p-asset-validation" ✅️asset-validation
PAT="$ids|manifest|never negative|nameplate|every rating|engines|sorts the elements" "$W/vt.sh" "$p-asset-power" ⚡️asset-power
