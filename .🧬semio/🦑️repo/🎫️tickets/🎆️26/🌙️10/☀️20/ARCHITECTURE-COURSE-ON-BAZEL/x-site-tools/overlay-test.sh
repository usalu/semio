#!/bin/sh
# usage: overlay-test.sh [bun test args…]; rebuilds the scratch overlay, applies the staged set to it and runs the deploy tests there
set -e
here="$(cd "$(dirname "$0")" && pwd)"
ov="${X_SITE_OVERLAY:?set X_SITE_OVERLAY to a scratch directory}"
"$here/mkoverlay.sh" "$ov" >/dev/null
X_SITE_ROOT="$ov" python3 "$here/staged/apply.py" >/dev/null
python3 "$here/staged/apply.py" --emit-manifest "$ov/manifest.json"
cd "$ov"
COURSE_MANIFEST="$ov/manifest.json" exec bun test "./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🟦️.ts" "$@"
