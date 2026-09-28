#!/bin/zsh
# 🧪️ C13: host laws on the live tree under ONE native-lane hold (rule 21a): the store worker's in-source laws + the pairing-rule
# law (os package), and the renderer's document-opening law (`document-opening-scope-check`).
export NX_DAEMON=false
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c13 -- zsh -c '
  cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "👷️worker" "surface-opens-kind"; echo "OS rc=$? $(date "+%F %T")"
  cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" && nice -n 15 bun ./📜️script.ts document-opening-scope-check; echo "OPENING rc=$? $(date "+%F %T")"
'
echo "EXIT rc=$? $(date '+%F %T')"
