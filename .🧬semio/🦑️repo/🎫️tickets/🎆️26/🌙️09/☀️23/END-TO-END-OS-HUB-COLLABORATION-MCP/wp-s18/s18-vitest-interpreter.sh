#!/bin/zsh
# 🧪️ S18: Interpreter in-source laws (tree windows incl. served windows, table), input-ledger (identity gate), + extra files, in the native lane.
P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd "$P" && echo "[s18] HELD $(date '+%F %T')" && SEMIO_TEST_LEVEL=long NX_DAEMON=false nice -n 15 bun "$P/📜️script.ts" test "../../../../🧱️elements/🗣️Interpreter/🟦️.tsx" "../../../../🧪️tests/🎯️input-ledger/🟦️.ts" "../../../../🧱️elements/🔐️HubSignIn/🧪️tests/🧩️component/🟦️.tsx" "$@"; echo "[s18] rc=$? $(date '+%F %T')"
