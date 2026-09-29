#!/bin/zsh
# 🧪️ S18 §15 (C13 P1 actor sections): os laws (worker harness, visible-surface contract, actor recovery) then renderer laws
# (ShellHelpers browser-actor patch corpus incl. sections), one native-lane hold.
OS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript"
RE="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
echo "[s18] HELD $(date '+%F %T')"
cd "$OS" && SEMIO_TEST_LEVEL=long NX_DAEMON=false nice -n 15 bun "$OS/📜️script.ts" test "../../🔨️modules/🏪️store/👷️worker/🟦️.ts" "../../🧪️tests/🪟️visible-surfaces/🟦️.ts" "../../🧪️tests/🚑️actor-recovery/🟦️.ts"
echo "[s18] os rc=$? $(date '+%F %T')"
cd "$RE" && SEMIO_TEST_LEVEL=long NX_DAEMON=false nice -n 15 bun "$RE/📜️script.ts" test "../../../../🧱️elements/🛠️ShellHelpers/🧪️tests/🎭️browser-actor-panels/🟦️.ts"
echo "[s18] renderer rc=$? $(date '+%F %T')"
