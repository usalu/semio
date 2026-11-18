#!/bin/sh
# usage: pw-pets.sh <control-port> playwright args
R=/Users/ueli/Documents/semio
C=$1; shift; CFG=${CFG:-"$R/🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts"}
cd $R
PLAYWRIGHT_BROWSERS_PATH=$HOME/Library/Caches/ms-playwright PLAYWRIGHT_BASE_URL=http://127.0.0.1:6065 TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY=dev TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR=http://127.0.0.1:8796 TEACHING_ARCHITECTURE_COURSE_E2E_CONTROL=http://127.0.0.1:$C \
 $HOME/.local/share/semio/node/v24.15.0/bin/node $R/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/cli.js test --config "$CFG" --output "$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/ARCHITECTURE-COURSE-REDEPLOY-READINESS/🗑️generated/grapple2/pw-output" "$@"
