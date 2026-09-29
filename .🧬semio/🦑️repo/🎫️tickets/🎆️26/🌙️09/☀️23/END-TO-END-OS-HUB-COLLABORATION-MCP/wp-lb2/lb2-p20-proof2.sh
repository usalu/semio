#!/bin/zsh
# 🌐️ LB2 p20 re-proof after the p20-s1 reds (two count pins p20 had missed) and the publisher/fixture pins: revert p20 in the
# scratch, write the grown set, rerun the stdio receipts + registry targets, the pin oracle (counts equal the tree + the surface
# fixture validates against its owning schema), the shipped fleet and the hub provider tests.
setopt no_bg_nice
C="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-captures"
W="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2"
tag="$1"
step() {
  local name="$1"; shift
  echo "STEP $name START $(date '+%T')"
  "$@" > "$C/$tag-$name.out" 2>&1
  echo "STEP $name rc=$? END $(date '+%T')"
  /usr/bin/grep -E "^test result|^error(\[|:)|^PROBLEM|files, |^PIN-CHECK|^FAIL|FAILED|restored" "$C/$tag-$name.out" | head -40
}
step revert python3 "$W/lb2-p20-hub-openable.py" --revert --root "$PWD"
step write python3 "$W/lb2-p20-hub-openable.py" --write --root "$PWD"
step pins node "$W/lb2-p20-pin-check.mjs" "$PWD"
step receipts cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test native_openable_provider
step registry cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --lib
step fleet cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet
step hub cargo test --offline --no-fail-fast -p semio-hub --lib native_openable
mkdir -p "$C/p22-probe"
step p22probe python3 "$W/lb2-p22-probe.py" "$PWD"
step p22run env LB2_PROBE_OUT="$C/p22-probe" cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet -- --ignored --exact --nocapture lb2_probe_round_trips
/usr/bin/grep "LB2-PROBE" "$C/$tag-p22run.out"
step p22unprobe python3 "$W/lb2-p22-probe.py" "$PWD" --remove
echo "PROOF END $(date '+%T')"
