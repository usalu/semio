#!/bin/zsh
# 🧪️ C13 (14c): the document-wide presence roster (row 3.4) — renderer-react `scoped-presence-check` (source oracle + roster law + the
# worker's scope-safe presence law) under ONE native-lane hold.
export NX_DAEMON=false
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c13 -- zsh -c '
  cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" && nice -n 15 bun ./📜️script.ts scoped-presence-check; echo "PRESENCE rc=$? $(date "+%F %T")"
'
echo "EXIT rc=$? $(date '+%F %T')"
