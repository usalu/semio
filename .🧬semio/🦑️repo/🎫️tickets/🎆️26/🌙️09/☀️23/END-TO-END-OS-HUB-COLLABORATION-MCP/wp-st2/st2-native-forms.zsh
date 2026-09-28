#!/bin/zsh
# 🦀️ ST2 native lane job: forms after the xlsx XML-parts export fix + its calamine law and the overnight label-resolver
# path break (panels → `terminology::forms_play_labels`, inspection test → `render(…, &view, …)`).
# usage: zsh st2-native-forms.zsh <capture>   (detached; priority stamp: chain fix)
setopt no_bg_nice
capture="${1:A}"
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-st2/target NX_DAEMON=false
cd /Users/ueli/Documents/semio || exit 1
{
  echo "[st2-native] QUEUED $(date '+%F %T') pid $$"
  FLEET_TICKET_STAMP=20260928000002 zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native st2 -- zsh -c 'echo "[st2-native] HOLD $(date "+%F %T")"; nice -n 15 cargo test -p semio-s-artifact-forms-forms --lib --tests --no-fail-fast --message-format=short; echo "[st2-native] forms test rc=$? $(date "+%F %T")"'
  echo "[st2-native] END rc=$? $(date '+%F %T')"
} > "$capture" 2>&1
