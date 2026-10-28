#!/bin/bash
# 🪓 [audit12] on a MIRROR root: reverts every G46 batch (reverse landing order), runs <bun test args> on the pre-G46 tree, then re-applies batch by batch and runs them again after each batch whose manifest matches <file-regex>: bash bisect.sh <mirror-root> <file-regex> <bun test args…>
M=${1:?mirror root}; rx=${2:?file regex}; shift 2; HERE=$(cd "$(dirname "$0")" && pwd); C=$(dirname "$HERE")
[ -e "$M/.git" ] && { echo "refusing a root with .git"; exit 2; }
BUN=${BUN:-$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun}
order=$(python3 -I -c 'import sys;ns={};exec([l for l in open(sys.argv[1],encoding="utf8") if l.startswith("ORDER")][0],ns);print("\n".join(ns["ORDER"]))' "$HERE/compose.py")
run(){ (cd "$M" && env -u SEMIO_TEST_ARTIFACT_DIR -u SEMIO_SCRIPT_PROCESS_INVOCATION "$BUN" test "$@" 2>&1) | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$' | cut -c1-140 | tr '\n' ' '; echo; }
for b in $(echo "$order" | tail -r); do python3 -I "$C/$b/stage.py" revert --root "$M" | grep -q DRIFTED && echo "REVERT REFUSED $b"; done
echo "pre-G46: $(run "$@")"
for b in $order; do python3 -I "$C/$b/stage.py" apply --root "$M" | grep DRIFTED && echo "APPLY REFUSED $b"; grep -qE "$rx" "$C/$b/manifest.json" && echo "+ $b: $(run "$@")"; done
python3 -I -c 'import sys' ; for b in $order; do python3 -I "$C/$b/stage.py" check --root "$M" >/dev/null 2>&1 || true; done
