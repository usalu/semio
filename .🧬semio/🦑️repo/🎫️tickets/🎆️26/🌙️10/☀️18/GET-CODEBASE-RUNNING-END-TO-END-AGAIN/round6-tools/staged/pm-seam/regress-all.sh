#!/bin/sh
# usage: sh regress-all.sh   the existing suites that touch the patched files, in the baseline mirror and in the mirror with the set, rows compared at the end
HERE=$(cd "$(dirname "$0")" && pwd)
D="⚡️caching/🚀️bootstrap/📦️dependencies"
rm -f /private/tmp/pm-seam/regall-*.tsv
BASE=${1:-base}; NEW=${2:-m}; MORE=${3:-}
for m in $BASE $NEW; do
  sh "$HERE/regress.sh" /private/tmp/pm-seam/$m /private/tmp/pm-seam/regall-$m.tsv \
    "🏗️build/🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts" "🏗️build/👁️observation/🧪️tests/⚡️incremental/🟦️.ts" \
    "$D/🧾️incremental/🧪️tests/🟦️.ts" "$D/🧾️generation/🧪️tests/🟦️.ts" "$D/🧾️generation/🧪️tests/🚪️ingress/🟦️.ts" "$D/🧾️generation/🧪️tests/🛡️followup/🟦️.ts" \
    "$D/🚦️concurrent/🧪️tests/🟦️.ts" "$D/🧪️tests/🟦️.ts" \
    "$D/💾️storage/🧪️tests/🟦️.ts" "$D/💾️storage/🧪️tests/📤️diagnostics/🟦️.ts" "$D/💾️storage/🧪️tests/🚧️fence/🟦️.ts" "$D/💾️storage/🧪️tests/📏️paths/🟦️.ts" "$D/💾️storage/🧪️tests/🔭️providers/🟦️.ts" "$D/💾️storage/🧪️tests/👓️reading/🟦️.ts" \
    $MORE "🗂️workspaces/🟦️bun/🧪️tests/🟦️.ts" "⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts" "⚡️caching/🕸️graph/✅️verification/🧪️tests/🟦️.ts" "⚡️caching/🧪️tests/📦️dependencies/🟦️.ts" "🏗️build/📥️inputs/💾️storage/🧪️tests/🎛️phase/🟦️.ts" "🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts" "🏗️build/🚀️bootstrap/🧪️tests/🎯️closure/🟦️.ts" "🏗️build/🧪️tests/🎟️phase/🟦️.ts" "🏗️build/🧪️tests/🕸️graph/🟦️.ts" "🧪️tests/🔧️build-surfaces/🟦️.ts"
done
sort /private/tmp/pm-seam/regall-$BASE.tsv > /private/tmp/pm-seam/regall-$BASE.s; sort /private/tmp/pm-seam/regall-$NEW.tsv > /private/tmp/pm-seam/regall-$NEW.s
diff /private/tmp/pm-seam/regall-$BASE.s /private/tmp/pm-seam/regall-$NEW.s > /private/tmp/pm-seam/regall.diff
echo "REGRESS-DONE rows base=$(wc -l < /private/tmp/pm-seam/regall-$BASE.tsv) m=$(wc -l < /private/tmp/pm-seam/regall-$NEW.tsv) difflines=$(wc -l < /private/tmp/pm-seam/regall.diff)" > /private/tmp/pm-seam/regall.done
