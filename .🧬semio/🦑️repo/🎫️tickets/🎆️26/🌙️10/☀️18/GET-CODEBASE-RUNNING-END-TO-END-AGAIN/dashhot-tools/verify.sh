#!/bin/sh
# 🧊 Laws of batch `fresh` after apply: sh verify.sh <root> [store]   (in a mirror: python3 -I C/mirror-tools/mirror.py run dhm --seal-allow <C-relative 🗑️generated/dashhot> -- sh verify.sh <mirror-root>)
# Plain legs: intent schema/reader/fixture, bootstrap catalog. Envelope legs (graph-free Script envelope from C/dashhot-tools/env-run.ts): the new compiled-sources laws and the store-free installation laws.
# `store` (live root only: the mirror root is too long for the rustup archive path) adds the four store-backed installation laws that publish and pin real executables.
set -u
root=$(cd "$1" && pwd -P); here=$(cd "$(dirname "$0")" && pwd)
C=$(python3 -I -c 'import os,sys;h=sys.argv[1]
while os.path.basename(h)!="GET-CODEBASE-RUNNING-END-TO-END-AGAIN" and h!="/":h=os.path.dirname(h)
print(h)' "$here")
art="$C/🗑️generated/dashhot/art"; mkdir -p "$art"; fail=0
D=./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution
LIB=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
python3 -I "$here/stage.py" check "$root" | grep -v "^applied" ; python3 -I "$here/stage.py" check "$root" >/dev/null || fail=1
cd "$root"
leg(){ label=$1; shift; out=$("$@" 2>&1); line=$(echo "$out" | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' '); echo "$label: $line"; echo "$out" | grep -E "^\(fail\)" | cut -c1-200; echo "$out" | grep -qE "^ *[1-9][0-9]* fail|^ *0 pass" && fail=1; echo "$out" | grep -qE "^ *[0-9]+ pass" || fail=1; }
leg intent bun test "$LIB/🏗️build/🕸️graph/🦀️rust/📥️intent/🧪️tests/🟦️.ts"
leg catalog bun test "$LIB/⚡️caching/🚀️bootstrap/📖️texts/🧪️tests/🟦️.ts"
envelope(){ bun "$C/dashhot-tools/env-run.ts" "$root" "@semio-tech/repo-dashboard-rs:test-source-freshness" "$art" -- test --timeout 300000 "$@"; }
leg compiled envelope "$D/🧾️compiled/🟦️.ts"
leg installation envelope -t "^(source digest|dashboard controls|dashboard bootstrap)" "$D/🟦️.ts"
if [ "${2:-}" = store ]; then
  for name in "dashboard immutable installation" "installation Source staleness" "installation receipt schema" "installed finite dashboard"; do leg "store: $name" envelope -t "^$name" "$D/🟦️.ts"; done
fi
[ $fail -eq 0 ] && echo "VERIFY fresh: GREEN" || echo "VERIFY fresh: RED"
exit $fail
