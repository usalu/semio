#!/bin/sh
# 🧪 FOR gate-g: setup-g batch verify (rebase-g). `sh C/setup-g-tools/staged/verify.sh [short-root]` builds a short-root MIRROR (never the live tree: path limit 256), runs `stage.py check` + `apply` into it (SEMIO_STAGE_ROOT), then the batch's single-file laws there, one `pass/fail` line per file; `ROUNDTRIP` prints the state after `revert` (must be all base/absent). Exit 1 on a failed check/apply or a failing law.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$C/setup-g-tools/staged"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"
OV="${1:-/private/tmp/claude-501/rb/setup}"
sh "$C/rebase-g-tools/mirror.sh" "$OV" "$B/🧰️tool-setup" "$B/🩺️preflight" "$B/📥️source/🛗️zero-touch" "$L/🏃️process/🦀️cargo" >/dev/null || exit 1
export SEMIO_STAGE_ROOT="$OV"
python3 -I "$T/stage.py" check >/dev/null || { python3 -I "$T/stage.py" check | head; echo "check FAILED"; exit 1; }
python3 -I "$T/stage.py" apply >/dev/null || { echo "apply FAILED"; exit 1; }
printf 'mirror applied: '; python3 -I "$T/stage.py" status | awk '{print $1}' | sort | uniq -c | tr -s ' \n' ' '; echo
fail=0
for f in "$B/🧰️tool-setup/🧪️tests/🟦️.ts" "$B/🧰️tool-setup/🧪️tests/🪜️steps/🟦️.ts" "$B/🧰️tool-setup/🧪️tests/📮️dispatch/🟦️.ts" "$B/🧰️tool-setup/⌨️command/🧪️tests/🟦️.ts" \
  "$B/🩺️preflight/🧪️tests/🟦️.ts" "$B/🩺️preflight/🧪️tests/🧰️toolchain/🟦️.ts" "$B/🩺️preflight/🧪️tests/🧮️matrix/🟦️.ts" "$B/🩺️preflight/💊️remedies/🧪️tests/🟦️.ts" "$B/📥️source/🛗️zero-touch/🧪️tests/🟦️.ts"; do
  out=$( cd "$OV/$(dirname "$f")" && nice -n 10 bun test "./$(basename "$f")" --timeout 120000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" | tr -s ' \n' ' ' )
  printf '%s  %s\n' "${f#$L/}" "$out"; case "$out" in *" 0 fail"*) ;; *) fail=1;; esac
done
python3 -I "$T/stage.py" revert >/dev/null; printf 'mirror reverted: '; python3 -I "$T/stage.py" status | awk '{print $1}' | sort | uniq -c | tr -s ' \n' ' '; echo
rm -rf "$OV"; exit $fail
