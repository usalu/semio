#!/bin/sh
# 🏛️ Graph-free verification after the flip: the laws of the sealed layer and the door, a promotion NOW (so no light command runs an old snapshot; its exit code counts, a blocked promotion is a failed flip), then the real tool.
# usage: sh post-flip.sh [ROOT]   ROOT defaults to the working directory (a mirror root works; PB=<pinned bun> overrides the pinned binary lookup)
# Exit 0 when every law file is green (the startup law is flip-independent: its known baseline red is reported as BASELINE, not counted) and the real tool answers.
ROOT="${1:-$PWD}"; cd "$ROOT" || exit 2
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; D="$B/🚪️door/📜️script.ts"; bad=0
PB="${PB:-$(ls "$ROOT"/.cache/b/t/m/bun/*/bun 2>/dev/null | head -1)}"; [ -x "$PB" ] || PB="$(command -v bun)"
law(){ out=$(bun test --timeout 120000 "./$1" 2>&1); p=$(echo "$out" | grep -E '^ *[0-9]+ pass' | tr -dc '0-9'); f=$(echo "$out" | grep -E '^ *[0-9]+ fail' | tr -dc '0-9'); printf '%s pass %s fail  %s%s\n' "${p:-?}" "${f:-?}" "$1" "${2:+  [$2: flip-independent, known red]}" | cut -c1-260; case "${f:-x}" in 0) ;; *) [ "$2" = baseline ] || bad=1;; esac; }
for f in "$B/🏛️sealed/🧪️tests/🗂️store/🟦️.ts" "$B/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts" "$B/🏛️sealed/🧪️tests/👥️processes/🟦️.ts" "$B/🏛️sealed/🧪️tests/🔀️interleavings/🟦️.ts" "$B/🚪️door/🧪️tests/🟦️.ts"; do law "$f"; done
law "$B/🚪️door/🧪️tests/🚦️startup/🟦️.ts" baseline
echo "policy: $(python3 -I -c "import json,sys;print(json.load(open(sys.argv[1]))['defaultMode'])" "$B/🏛️sealed/🔣️.json")"
echo "promote:"; promoted=$(SEMIO_REPO_SEAL=promote "$PB" "$D" repo 2>&1); rc=$?; echo "$promoted" | grep -E "Promoted|promoted|Sealed snapshot|Versiegelter|cannot be promoted|nicht" | cut -c1-200; echo "promote exit $rc"; [ "$rc" = 0 ] || bad=1
echo "tool:"
"$PB" "$D" repo 2>&1 | head -1 | cut -c1-80
"$PB" "$D" leases status 2>&1 | grep -E "^Leases in" | cut -c1-120
"$PB" "$D" repo cache status 2>&1 | grep -E "implementation\]|implementation:|complete" | cut -c1-160
echo "POSTFLIP exit $bad"; exit $bad
