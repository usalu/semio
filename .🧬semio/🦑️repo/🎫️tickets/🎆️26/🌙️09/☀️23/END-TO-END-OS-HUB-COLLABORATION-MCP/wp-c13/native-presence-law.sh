#!/bin/zsh
# 🧪️ C13 (14c): the scoped-presence law file alone (roster, ink camera) under ONE native-lane hold.
export NX_DAEMON=false SEMIO_TEST_LEVEL=long
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c13 -- zsh -c '
  cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../../../🧪️tests/👥️scoped-presence/🟦️.tsx"; echo "LAW rc=$? $(date "+%F %T")"
'
echo "EXIT rc=$? $(date '+%F %T')"
