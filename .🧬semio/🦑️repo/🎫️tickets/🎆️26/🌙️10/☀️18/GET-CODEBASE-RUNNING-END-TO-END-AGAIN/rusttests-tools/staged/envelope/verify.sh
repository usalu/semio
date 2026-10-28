#!/bin/sh
# 🪪 [rusttests] verifies batch `envelope` on <root> (a mirror, or live after apply): the helper laws (16), the build route-table laws (9, incl. the declaration-vs-route law and the genuine native grant law), and every declared library file under a BARE `bun test ./file` from the root (no parent envelope, no artifact directory; a short in-checkout directory only for checkout-scratch files when the root path is too long for the sandbox root limit, as it is in a mirror). A mismatching file is run once more (the 2 s receiver timing laws of the helper tests jitter at host load 40+). Files with a known open cause print `KNOWN-OPEN` with their exact expected red count (any other count fails). usage: sh verify.sh <root>
R="${1:?root}"; here="$(cd "$(dirname "$0")" && pwd)"; rc=0; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
law() { want="$1"; open="$2"; scratch="$3"; f="$4"; art=""; [ "$open" = "-" ] && open=""
  [ "$scratch" = checkout ] && [ ${#R} -gt 40 ] && { mkdir -p "$R/.cache/test-sandbox/rtv"; art="$R/.cache/test-sandbox/rtv"; }
  for attempt in 1 2; do
  out=$(cd "$R" && env -u SEMIO_SCRIPT_PROCESS_INVOCATION -u SEMIO_TEST_ARTIFACT_DIR ${art:+SEMIO_TEST_ARTIFACT_DIR="$art"} nice -n 10 bun test --timeout 170000 "./$f" 2>&1 </dev/null)
  got="$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ pass$' | tail -1 | tr -dc 0-9)/$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ fail$' | tail -1 | tr -dc 0-9)"
  [ "$got" = "$want" ] && break; done
  if [ "$got" = "$want" ]; then if [ -n "$open" ]; then echo "KNOWN-OPEN $got ($open) $f"; else echo "OK    $got $f"; fi; else echo "FAIL  got $got want $want $f"; printf '%s\n' "$out" | grep -E '^\(fail\)|^error' | head -6 | cut -c1-220; rc=1; fi; }
law "16/0" "" sandbox "🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🧪️tests/🟦️.ts"
law "9/0" "" sandbox "$L/🏗️build/🗂️suites/🧪️tests/🟦️.ts"
while IFS="$(printf '\t')" read -r want open scratch file; do [ -n "$file" ] && law "$want" "$open" "$scratch" "$file"; done < "$here/expect.tsv"
[ $rc -eq 0 ] && echo "VERIFY OK envelope" || echo "VERIFY FAILED envelope"; exit $rc
