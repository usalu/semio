#!/bin/zsh
# 🔁️ One hand cycle on the work ops of a person (🗑️generated/silhouette/R2-HA/ops/<slug>.json, applied in memory on the catalog file): with specs, redraws them from the volume (`outline.ts`) into the work ops; then the checks of the scale and the worst clusters of the view: cyc.sh <slug> <scale> <view> [spec…]   (F=0.7 lists clusters above 0.7 of the limit, N=count)
source "${0:A:h}/env.sh"
a="$1"; sc="$2"; v="$3"; shift 3; mkdir -p "$GEN/ops"; o="$GEN/ops/$a.json"; [[ -f "$o" ]] || echo "[]" > "$o"
cd "$S/📦️packages/🟦️typescript"
if (( $# > 0 )); then timeout 600 "$B" "$W/outline.ts" "${SRC:-$a}" "$sc" "$v" "$o" "$@" --base "$o" || exit 1; fi
timeout 600 "$B" "$W/one.ts" "${SRC:-$a}" "$sc" all "$o"
timeout 600 "$B" "$W/clusters.ts" "${SRC:-$a}" "$sc" "$v" "$o" "${N:-6}" | cut -c1-${CUT:-700}
