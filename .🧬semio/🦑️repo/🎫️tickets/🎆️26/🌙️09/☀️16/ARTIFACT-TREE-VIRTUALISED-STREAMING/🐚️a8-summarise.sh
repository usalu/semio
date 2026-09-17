#!/bin/zsh
# 🧾️ Reads every per-crate log 🐚️a8-verify.sh has written so far and prints one line per crate: the
# window-law verdict (the only assertions this packet owns), the full-suite verdict, and the wasm-target
# check. `cap` in the suite column means the run hit the watchdog, not that it failed.
set -u
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ARTIFACT-TREE-VIRTUALISED-STREAMING/🗑️generated/a8"
verdict() {
  local f="$1"
  [ -e "$f" ] || { echo '-'; return }
  local r
  r=$(grep -hoE '^test result: (ok|FAILED)' "$f" 2>/dev/null | tail -1 | sed 's/test result: //')
  if [ -n "$r" ]; then echo "$r"; return; fi
  grep -q '^    Finished' "$f" && { echo ok; return }
  grep -qE '^error(\[|: could not compile)' "$f" && { echo BUILD-ERR; return }
  echo running
}
printf '%-38s %-10s %-9s %-10s %s\n' CRATE LAWS 'LAWS n' SUITE WASM
for f in "$G"/*.laws.txt; do
  [ -e "$f" ] || continue
  crate="${f:t:r:r}"
  ok=$(grep -hcE '^test .* \.\.\. ok$' "$f" 2>/dev/null | tr -d ' ')
  bad=$(grep -hcE '^test .* \.\.\. FAILED$' "$f" 2>/dev/null | tr -d ' ')
  suite=$(verdict "$G/$crate.test.txt")
  grep -q "$crate test TIMEOUT" "$G/progress.txt" 2>/dev/null && suite="$suite(cap)"
  printf '%-38s %-10s %-9s %-10s %s\n' "$crate" "$(verdict "$f")" "$ok ok/$bad bad" "$suite" "$(verdict "$G/$crate.wasm.txt")"
done
echo
tail -3 "$G/progress.txt" 2>/dev/null
