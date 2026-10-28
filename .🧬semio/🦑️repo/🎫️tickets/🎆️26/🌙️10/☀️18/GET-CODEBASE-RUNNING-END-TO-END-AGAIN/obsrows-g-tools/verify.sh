#!/bin/sh
# usage: verify.sh [check|selftest|suites]   (obsrows-g) check = batch state on the live tree; selftest = apply/revert round trip on a scratch copy; suites = staged single-file suites (Nx env supplied by hand, observation module not mirrored)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$PWD/$C/obsrows-g-tools"; G="$PWD/$C/🗑️generated/obsrows-g"; L="$PWD/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
case "${1:-check}" in
check) python3 -I "$T/stage.py" check;;
selftest)
  R="$G/scratch-root/"; rm -rf "$R"; mkdir -p "$R"
  python3 -I - "$T" "$L" "$R" <<'PY' || exit 1
import json, os, shutil, sys
T, L, R = sys.argv[1:4]
for entry in json.load(open(T + "/batch/batch.json", encoding="utf8"))["files"]:
    os.makedirs(os.path.dirname(R + entry["file"]), exist_ok=True); shutil.copy(L + entry["file"], R + entry["file"])
PY
  python3 -I "$T/stage.py" apply --root "$R" >/dev/null && python3 -I "$T/stage.py" check --root "$R" | awk '{print $1}' | sort | uniq -c && python3 -I "$T/stage.py" revert --root "$R" >/dev/null && echo "round trip ok"; rm -rf "$R";;
suites)
  B=$(ls "$PWD"/.cache/b/t/m/bun/*/bun | head -1); export SEMIO_NX_FIXED_STORAGE="$PWD/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m" NX_CACHE_DIRECTORY="$PWD/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts" SKIP_STAGED="🏗️build/👁️observation/🟨️.mjs"
  mkdir -p "$G/verify"
  for f in "🏗️build/🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts" "🏗️build/🧪️tests/🕸️graph/📐️portable-model/🟦️.ts" "🏗️build/👁️observation/🧪️tests/📥️reuse/🟦️.ts" "🏗️build/🧪️tests/🕸️graph/🛂️admission/🟦️.ts"; do
    n=$(printf '%s' "$f" | md5 -q | cut -c1-8); "$B" "$T/stage-run.mjs" "$f" "$G/verify/$n.log"; echo "$f: $(/usr/bin/grep -a -E '^ *[0-9]+ (pass|fail)$' "$G/verify/$n.log" | tr -s ' \n' ' ')"
  done;;
esac
