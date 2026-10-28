#!/bin/sh
# 🪪 [cmdid] verifies batch `identity` on <root> (a mirror, or live after apply): the command law file beneath a genuine native build caller parent (expect 5/0) and bare (5/0 when the file declares its library birth, else 2/4 = only the envelope-free laws run), the loader-identity law 3/0, the generation-protocol pin-guard law, and the stage tool mode law. usage: sh verify.sh <root>
R="${1:?root}"; here="$(cd "$(dirname "$0")" && pwd)"; C="$(cd "$here/../../.." && pwd)"; rc=0; B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
CMD="$B/⌨️command/🧪️tests/🟦️.ts"; ART="$C/🗑️generated/cmdid/art"; mkdir -p "$ART"
count() { printf '%s\n' "$1" | grep -E '^ *[0-9]+ pass$' | tail -1 | tr -dc 0-9; printf /; printf '%s\n' "$1" | grep -E '^ *[0-9]+ fail$' | tail -1 | tr -dc 0-9; }
law() { want="$1"; name="$2"; shift 2; out=$(cd "$R" && "$@" 2>&1 </dev/null); got=$(count "$out")
  if [ "$got" = "$want" ]; then echo "OK    $got $name"; else echo "FAIL  got $got want $want $name"; printf '%s\n' "$out" | grep -E '^\(fail\)|^error' | head -8 | cut -c1-240; rc=1; fi; }
wire=$(cd "$R" && bun "$here/../../wire.ts")
[ -n "$wire" ] || { echo "FAIL no parent envelope"; exit 1; }
law 5/0 "command laws beneath a genuine parent" env SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" bun test --timeout 170000 "./$CMD"
if grep -q declareLibraryBuildTestEnvelopeV1 "$R/$CMD"; then bare=5/0; else bare=2/4; fi
law "$bare" "command laws bare" env -u SEMIO_SCRIPT_PROCESS_INVOCATION -u SEMIO_TEST_ARTIFACT_DIR bun test --timeout 170000 "./$CMD"
law 3/0 "loader-identity law" env -u SEMIO_SCRIPT_PROCESS_INVOCATION bun test "./$B/📥️source/🔏️closure/🚪️command/🧪️tests/🏷️loader-identity/🟦️.ts"
out=$(cd "$R" && env -u SEMIO_SCRIPT_PROCESS_INVOCATION bun test "./$B/🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts" -t "every self-check of a loaded implementation lets a pinned invocation finish" 2>&1 </dev/null); got=$(count "$out")
[ "$got" = "1/0" ] && echo "OK    $got generation-protocol pin-guard law" || { echo "FAIL  got $got generation-protocol pin-guard law"; printf '%s\n' "$out" | grep -E '^\(fail\)|^error|offender|Expected|Received' | head -8 | cut -c1-240; rc=1; }
python3 -I "$C/xplat5-tools/tests/test_stage_mode.py" "$here/stage.py" >/dev/null 2>&1 && echo "OK    stage.py mode law" || { echo "FAIL  stage.py mode law"; rc=1; }
[ $rc -eq 0 ] && echo "VERIFY OK identity" || echo "VERIFY FAILED identity"; exit $rc
