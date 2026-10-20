#!/bin/sh
# Verifies the combined series. Never writes into the checkout in `mirror` mode; in `live` mode only reads.
#   check-all.sh mirror <dir> [--no-baseline] [--tests a,b]   byte-copy the course closure to <dir>/base, copy it to <dir>/series, apply the whole series (cleanup included), prove idempotency and the
#                                                            peer-not-applied variant, run the semantic checks and every set's tests on base and series, compare
#   check-all.sh live [--tests a,b]                          after apply-all.py --live: anchors all applied, semantic checks and the tests against the checkout
# Only explicit test files, at most 2 at once (see check-tests.py). Output: <dir>/logs/*.json and a summary on stdout; exit 1 on any failure.
set -u
TOOLS=$(cd "$(dirname "$0")" && pwd)
TICKET=$(cd "$TOOLS/.." && pwd)
LIVE=$(cd "$TOOLS/../../../../../../../.." && pwd)
MODE=${1:-}
status=0
note() { printf '%s\n' "== $*"; }
fail() { printf 'FAIL %s\n' "$*"; status=1; }
tests_arg=""
for arg in "$@"; do case "$arg" in --tests) ;; esac; done
while [ $# -gt 0 ]; do case "$1" in --tests) tests_arg="--groups $2"; shift 2 ;; *) shift ;; esac; done
semantic() {
  root=$1; logs=$2
  note "manifest invariants"; python3 "$TOOLS/check-manifests.py" "$root" || fail "manifest invariants"
  note "cached site build key completeness (independent TypeScript resolver walk vs declared + derived closure)"; python3 "$TOOLS/check-build-closure.py" "$root" || fail "build closure gap"
  note "type diagnostics of the touched files"; bun "$TOOLS/check-types.ts" "$root" "$logs/types-$(basename "$root").json" || fail "type check run"
}
case "$MODE" in
  mirror)
    DIR=${2:?mirror directory required}
    mkdir -p "$DIR/logs"
    LOGS=$DIR/logs
    if [ ! -d "$DIR/base" ]; then note "byte-copy mirror (no links, no clones)"; python3 "$TICKET/x-inputs-tools/mkmirror-copy.py" "$DIR/base" || exit 1; fi
    rm -rf "$DIR/series" "$DIR/peerless"; rsync -a "$DIR/base/" "$DIR/series/"
    note "apply the whole series to series/ (--live on the mirror)"; python3 "$TOOLS/apply-all.py" --live --root "$DIR/series" --with-cleanup || fail "series apply"
    note "second run must find everything applied"; python3 "$TOOLS/apply-all.py" --check --root "$DIR/series" --with-cleanup | tee "$LOGS/second-run.txt" | tail -4
    if grep -Eq '^(pending|refused)' "$LOGS/second-run.txt"; then fail "series is not idempotent"; fi
    note "variant: peer's uncached-owners rows NOT applied (HEAD manifests) must give byte-identical manifests"
    rsync -a "$DIR/base/" "$DIR/peerless/"
    for rel in "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json" "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json"; do git -C "$LIVE" show "HEAD:$rel" > "$DIR/peerless/$rel"; done
    python3 "$TOOLS/apply-all.py" --live --root "$DIR/peerless" --with-cleanup >/dev/null || fail "peerless apply"
    for rel in "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json" "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json"; do cmp "$DIR/series/$rel" "$DIR/peerless/$rel" || fail "manifest differs without the peer rows: $rel"; done
    rm -rf "$DIR/peerless"
    semantic "$DIR/series" "$LOGS"
    base_types=$LOGS/types-base.json; bun "$TOOLS/check-types.ts" "$DIR/base" "$base_types" >/dev/null
    python3 - "$base_types" "$LOGS/types-series.json" <<'PY' || fail "new type diagnostics"
import json, sys
base, series = (set(json.load(open(path, encoding="utf8"))["diagnostics"]) for path in sys.argv[1:3])
new = sorted(series - base)
print(f"type diagnostics: base {len(base)}, series {len(series)}, new {len(new)}")
for row in new: print("   NEW", row)
sys.exit(1 if new else 0)
PY
    note "tests on the series tree"; python3 "$TOOLS/check-tests.py" "$DIR/series" "$LOGS/tests-series.json" $tests_arg || fail "series tests"
    if [ "${2:-}" != "--no-baseline" ] && ! printf '%s' "$*" | grep -q -- '--no-baseline'; then
      note "tests on the baseline tree"; python3 "$TOOLS/check-tests.py" "$DIR/base" "$LOGS/tests-base.json" $tests_arg || true
      note "compare"; python3 "$TOOLS/check-compare.py" "$LOGS/tests-base.json" "$LOGS/tests-series.json" || fail "new test failures"
    fi
    ;;
  live)
    LOGS=${TMPDIR:-/tmp}/x-integrate-live; mkdir -p "$LOGS"
    note "every step applied"; python3 "$TOOLS/apply-all.py" --check | tee "$LOGS/live-check.txt" | tail -14
    if grep -Eq '^(pending|refused)' "$LOGS/live-check.txt"; then fail "series not fully applied on the checkout"; fi
    semantic "$LIVE" "$LOGS"
    note "tests on the checkout"; python3 "$TOOLS/check-tests.py" "$LIVE" "$LOGS/tests-live.json" $tests_arg || fail "live tests"
    python3 - "$LOGS/tests-live.json" <<'PY' || fail "failing tests on the checkout"
import json, sys
rows = json.load(open(sys.argv[1], encoding="utf8"))["groups"]
bad = {name: row["failing"] for name, row in rows.items() if row["failing"]}
print(json.dumps(bad, ensure_ascii=False, indent=1)[:3000]); sys.exit(1 if bad else 0)
PY
    ;;
  *) echo "usage: check-all.sh mirror <dir> [--no-baseline] [--tests a,b] | live [--tests a,b]" >&2; exit 2 ;;
esac
note "check-all: $([ "$status" = 0 ] && echo OK || echo FAILED)"
exit $status
