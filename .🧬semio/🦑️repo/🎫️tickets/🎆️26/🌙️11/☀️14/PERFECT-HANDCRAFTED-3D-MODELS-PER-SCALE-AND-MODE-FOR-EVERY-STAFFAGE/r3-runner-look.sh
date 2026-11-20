#!/bin/sh
# 👀 Renders the round-3 look sheets of humans/runner-with-blade-prosthesis with r3-sheet.ts: every list at every scale, the standing skeleton, the recovery pose and two clip frames.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for a in "--scale 50 --list structure --pose standing-ready --no-rig" "--scale 100 --list structure --no-rig" "--scale 200 --list structure --no-rig" "--scale 500 --list structure --no-rig" "--scale 100 --list thermal --no-rig" "--scale 200 --list thermal --no-rig" "--scale 500 --list thermal --no-rig" "--scale 50 --list thermal --no-rig" "--scale 50 --list thermal --cut --no-rig" "--scale 50 --no-rig" "--scale 100 --no-rig" "--scale 200 --no-rig" "--scale 500 --no-rig" "--scale 50 --pose hands-on-hips --no-rig" "--scale 50 --clip run-cycle --t 0.1 --no-rig" "--scale 100 --clip run-cycle --t 0.3 --list structure" "--scale 50 --list structure --no-labels"; do
  # shellcheck disable=SC2086
  "$BUN" r3-sheet.ts humans/runner-with-blade-prosthesis $a 2>&1 | tail -1 | sed 's|.*/r3-sheet/||'
done
