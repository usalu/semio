#!/bin/sh
# 🛡️ laws of batch `ownerbytes` (lane ownerbytes): sh verify.sh <mirror-root> [before]
#   works on a mirror root or the live root (the process envelope runner is pointed at the given root)
#   after  (default): owner-bytes law, bootstrap cli + arguments tests, revision tests, policy-literals law (0 fail each) and a real plan read of all 8 recovery routes (frontend build/install, maintenance preview/retire, reset graph/nx/rust/products)
#   before : negative controls on the mirror (each restored through force.py): the 10-28 gate state (bootstrap ceiling 65536) and the 10-27 state (revision text ceiling 65536) turn the law and the route read RED
ROOT="$(cd "${1:?mirror root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; MODE="${2:-after}"; FAILED=0
T="$HERE/../.."; C="$(cd "$T/.." && pwd)"; S="$(mktemp -d)"; trap 'rm -rf "$S"' EXIT
sed "s#/Users/ueli/Documents/semio#$ROOT#g" "$C/ver-f-tools/proc-run.ts" > "$S/proc-run.ts"
envrun() { (cd "$ROOT" && VF_WALL=300 bun "$S/proc-run.ts" file "$ROOT/$1" 2>&1); }
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; CLI="$L/🏗️build/🚀️bootstrap/⌨️cli"; REV="$L/🏗️build/📥️inputs/🔏️revision"
python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
suite() { name=$1; file=$2; out=$(envrun "$file"); summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|fail)$" | tr -s ' \n' ' '); case " $summary" in *" 0 fail"*) echo "OK    $name: $summary";; *) echo "FAIL  $name: $summary"; printf '%s\n' "$out" | grep -E "^\(fail\)|^error" | head -5; FAILED=1;; esac; }
routes() { (cd "$ROOT" && bun "$T/routes.ts" "$ROOT") >"$ROOT/.ob-routes" 2>&1; code=$?; echo "$code $(grep -c '"kind"' "$ROOT/.ob-routes")"; rm -f "$ROOT/.ob-routes"; }
edit() { python3 -I - "$ROOT/$1" "$2" "$3" <<'PY'
import sys
path, old, new = sys.argv[1:4]
text = open(path, encoding="utf8").read()
assert text.count(old) == 1, old
open(path, "w", encoding="utf8").write(text.replace(old, new))
PY
}
if [ "$MODE" = before ]; then
  [ -e "$ROOT/.git" ] && { echo "before mode edits files: mirror roots only"; exit 2; }
  edit "$CLI/🧬️schema/🔣️.json" '"maximumOwnerBytes": 262144,' '"maximumOwnerBytes": 65536,'
  out=$(envrun "$CLI/🧪️tests/📏️owner-bytes/🟦️.ts"); printf '%s\n' "$out" | grep -qE "[1-9][0-9]* fail" && echo "OK    law RED at bootstrap ceiling 65536 (gate state 10-28)" || { echo "FAIL  law green at bootstrap ceiling 65536"; FAILED=1; }
  [ "$(routes)" = "1 2" ] && echo "OK    routes RED at 65536: only the 2 frontend routes enter" || { echo "FAIL  routes at 65536: $(routes)"; FAILED=1; }
  python3 -I "$HERE/../../force.py" "$ROOT" >/dev/null
  edit "$REV/🧬️schema/🔣️.json" '"maximumTextBytes": 262144,' '"maximumTextBytes": 65536,'
  out=$(envrun "$CLI/🧪️tests/📏️owner-bytes/🟦️.ts"); printf '%s\n' "$out" | grep -qE "[1-9][0-9]* fail" && echo "OK    law RED at revision text ceiling 65536 under bootstrap ceiling 262144 (state 10-27)" || { echo "FAIL  law green at revision text ceiling 65536"; FAILED=1; }
  python3 -I "$HERE/../../force.py" "$ROOT" >/dev/null
  python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "FAIL  batch not restored"; FAILED=1; }
  [ $FAILED = 0 ] && echo "VERIFY OK ownerbytes (before)" || echo "VERIFY FAIL ownerbytes (before)"; exit $FAILED
fi
suite owner-bytes "$CLI/🧪️tests/📏️owner-bytes/🟦️.ts"
suite bootstrap-cli "$CLI/🧪️tests/🟦️.ts"
suite bootstrap-arguments "$CLI/🧪️tests/🧮️arguments/🟦️.ts"
suite revision "$REV/🧪️tests/🟦️.ts"
suite policy-literals "$L/🏗️build/🧪️tests/🧬️policy-literals/🟦️.ts"
[ "$(routes)" = "0 8" ] && echo "OK    all 8 recovery routes read their plan" || { echo "FAIL  route read: $(routes)"; FAILED=1; }
[ $FAILED = 0 ] && echo "VERIFY OK ownerbytes" || echo "VERIFY FAIL ownerbytes"; exit $FAILED
