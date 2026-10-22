#!/bin/sh
# usage: sh regress-new.sh NEW [BASE]   re-runs the suites in NEW only and compares with the stored baseline rows of BASE (default r1)
HERE=$(cd "$(dirname "$0")" && pwd)
NEW=$1; BASE=${2:-r1}
D="⚡️caching/🚀️bootstrap/📦️dependencies"
rm -f /private/tmp/pm-seam/regall-$NEW.tsv /private/tmp/pm-seam/regall.done
sh "$HERE/regress.sh" /private/tmp/pm-seam/$NEW /private/tmp/pm-seam/regall-$NEW.tsv \
    "🏗️build/🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts" "🏗️build/👁️observation/🧪️tests/⚡️incremental/🟦️.ts" \
    "$D/🧾️incremental/🧪️tests/🟦️.ts" "$D/🧾️generation/🧪️tests/🟦️.ts" "$D/🧾️generation/🧪️tests/🚪️ingress/🟦️.ts" "$D/🧾️generation/🧪️tests/🛡️followup/🟦️.ts" \
    "$D/🚦️concurrent/🧪️tests/🟦️.ts" "$D/🧪️tests/🟦️.ts" \
    "$D/💾️storage/🧪️tests/🟦️.ts" "$D/💾️storage/🧪️tests/📤️diagnostics/🟦️.ts" "$D/💾️storage/🧪️tests/🚧️fence/🟦️.ts" "$D/💾️storage/🧪️tests/📏️paths/🟦️.ts" "$D/💾️storage/🧪️tests/🔭️providers/🟦️.ts" "$D/💾️storage/🧪️tests/👓️reading/🟦️.ts" \
    "$D/💾️storage/🧪️tests/🌐️platform/🟦️.ts" $MORE "🗂️workspaces/🟦️bun/🧪️tests/🟦️.ts" "⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts" "⚡️caching/🕸️graph/✅️verification/🧪️tests/🟦️.ts" "⚡️caching/🧪️tests/📦️dependencies/🟦️.ts" "🏗️build/📥️inputs/💾️storage/🧪️tests/🎛️phase/🟦️.ts" "🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts" "🏗️build/🚀️bootstrap/🧪️tests/🎯️closure/🟦️.ts" "🏗️build/🧪️tests/🎟️phase/🟦️.ts" "🏗️build/🧪️tests/🕸️graph/🟦️.ts" "🧪️tests/🔧️build-surfaces/🟦️.ts"
sort /private/tmp/pm-seam/regall-$NEW.tsv > /private/tmp/pm-seam/regall-$NEW.s
sort /private/tmp/pm-seam/regall-$BASE.tsv > /private/tmp/pm-seam/regall-$BASE.s
diff /private/tmp/pm-seam/regall-$BASE.s /private/tmp/pm-seam/regall-$NEW.s | sed 's/🧰[^\t]*📚️library\///;s#⚡️caching/🚀️bootstrap/📦️dependencies/##' | cut -c1-220 > /private/tmp/pm-seam/regall.diff
echo "REGRESS-DONE $(wc -l < /private/tmp/pm-seam/regall-$NEW.tsv) rows, $(wc -l < /private/tmp/pm-seam/regall.diff) difflines" > /private/tmp/pm-seam/regall.done
