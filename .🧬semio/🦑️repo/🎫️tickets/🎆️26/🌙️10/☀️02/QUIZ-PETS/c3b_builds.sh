#!/usr/bin/env bash
# 💰️ C3b: release builds of the site, one per variant of 🗑️generated/c3b/variants.json (c3b_budget.ts), each into
# 🗑️generated/c3b/<prefix>-<variant>[-<n>], weighed with site_bundle_weight.ts. The first variant is built again last,
# so a change of the tree between the builds shows as two different weights of the same variant.
# Usage (Git Bash, any directory): bash <TK>/c3b_builds.sh <prefix> <variant> [<variant>…]
set -u
TK="$(cd "$(dirname "$0")" && pwd)"
G="$TK/🗑️generated/c3b"
SITE="$TK/../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript"
VITE="$TK/../../../../../../../node_modules/vite/bin/vite.js"
prefix="$1"
shift
first="$1"
cd "$SITE" || exit 1
for variant in "$@" "$first"; do
  out="$prefix-$variant"
  [ -d "$G/$out" ] && [ "$variant" = "$first" ] && out="$out-again"
  C3B_VARIANT="$variant" NX_PLUGIN_NO_TIMEOUTS=true bun "$VITE" build --config "$TK/c3b_budget.vite.ts" --configLoader bundle --outDir "$G/$out" --emptyOutDir > "$G/$out.log" 2>&1
  code=$?
  echo "== $out (exit $code, $(date +%H:%M:%S))"
  if [ "$code" -eq 0 ]; then bun "$TK/site_bundle_weight.ts" "$G/$out" | grep -v '^lazy .*\.css'; else grep -m 3 -E 'error|Error' "$G/$out.log"; fi
done
