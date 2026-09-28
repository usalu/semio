#!/bin/zsh
# 🧪️ C13 (14c): P1's language-agnostic half in the overlay — the replication package's in-source laws (the TS fold twin replaying
# the durable-collaborative-redo fixture P1 extends) under ONE native-lane hold; no cargo (rule 23: P1 = 6 files, Rust proof = window 3).
# Mutant: the live tree's pre-P1 twin against P1's fixture must fail (the fixture's foreign-transition steps catch the defect).
OV="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-overlay"
export NX_DAEMON=false
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c13 -- zsh -c '
  TWIN="🧰️framework/🔨️modules/📡️replication/🧪️tests/🗄️durable-collaborative-redo/🟦️.ts"
  cd "$0/🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript" || exit 1
  nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" --reporter=verbose; echo "TS rc=$? $(date "+%F %T")"
  cp "$0/$TWIN" "$0/$TWIN.p1" && cp "/Users/ueli/Documents/semio/$TWIN" "$0/$TWIN"
  nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" --reporter=verbose; echo "MUTANT pre-P1 twin rc=$? $(date "+%F %T")"
  mv "$0/$TWIN.p1" "$0/$TWIN"
' "$OV"
echo "EXIT rc=$? $(date '+%F %T')"
