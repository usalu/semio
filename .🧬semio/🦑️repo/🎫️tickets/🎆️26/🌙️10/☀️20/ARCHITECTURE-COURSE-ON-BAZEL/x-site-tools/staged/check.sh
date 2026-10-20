#!/bin/sh
# usage: check.sh [live]
# Without argument: proves the staged set on a scratch overlay (anchors, deploy tests, types). With `live`: runs the anchors check and the deploy tests against the live checkout after apply.py.
set -e
here="$(cd "$(dirname "$0")" && pwd)"
tools="$here/.."
root="$(cd "$here/../../../../../../../.." && pwd)"
python3 "$here/apply.py" --check
if [ "$1" = "live" ]; then
  cd "$root"
  bun test "./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🟦️.ts"
  bun "$tools/check-types.ts" "$root" "📦️packages/🟦️typescript/📜️script.ts" "🚀️deploy/🌐️site/🟦️.ts" "🚀️deploy/⚖️readiness/🟦️.ts" "🧪️tests/🧪️deploy/🟦️.ts"
else
  ov="${X_SITE_OVERLAY:-${TMPDIR:-/tmp}/x-site-overlay}"
  X_SITE_OVERLAY="$ov" "$tools/overlay-test.sh"
  bun "$tools/check-types.ts" "$ov" "📦️packages/🟦️typescript/📜️script.ts" "🚀️deploy/🌐️site/🟦️.ts" "🚀️deploy/⚖️readiness/🟦️.ts" "🧪️tests/🧪️deploy/🟦️.ts"
fi
