#!/bin/sh
# Runs the documentation-law tests against the README with and without the staged edits, inside an OVERLAY (the checkout is never written).
# The overlay is a mirror of the checkout root made of symlinks, except the directories on the way to the library (real directories) and the
# library itself (an APFS/CoW clone or a copy), so every module computes its root from the overlay. One bun process at a time, niced.
# Usage: lawtest.sh [baseline|applied|both]   (default both). Results: /private/tmp/docs1021a/results.txt
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
PRODUCT="🧰️framework/🛍️products/🦑️repo"
LIB="$PRODUCT/🔨️modules/📚️library"
OV=/private/tmp/docs1021a/ov
OUT=/private/tmp/docs1021a/results.txt
MODE="${1:-both}"
mkdir -p /private/tmp/docs1021a
if [ ! -d "$OV/$LIB" ]; then
  python3 -I "$HERE/overlay.py" "$ROOT" "$OV"
  cp "$ROOT/$PRODUCT/README.md" "$OV/$PRODUCT/README.md.pristine"
fi
ART=/private/tmp/docs1021a/art; mkdir -p "$ART"
export SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_WORKSPACE_ROOT="$OV"
L="$LIB/🧪️tests"
run() {
  label="$1"; shift
  echo "== $label $*" >> "$OUT"
  (cd "$OV" && nice -n 10 perl -e 'alarm 590; exec @ARGV' -- bun test --tsconfig-override "$HERE/tsconfig.alias.json" --timeout 240000 "$@" 2>&1) | grep -E "^\(fail\)| pass$| fail$|^error:|^Ran " | head -14 >> "$OUT" || true
}
suite() {
  state="$1"
  for entry in "owner-projection-policy|🏠️owner-projection-policy" "readme-current-source-revision|🔖️readme-current-source-revision" "frozen-markdown-coordinates|❄️frozen-markdown-coordinates" "testing-readme-coordinates|🗺️testing-readme-coordinates" "readme-current-source-activation|🟢️readme-current-source-activation" "readme-reviewed-fixture-inputs|👀️readme-reviewed-fixture-inputs" "readme-move-source-authority|🚚️readme-move-source-authority"; do
    name="${entry%%|*}"; dir="${entry#*|}"
    filter=""
    if [ "$name" = owner-projection-policy ]; then filter="--test-name-pattern=^(?!.*actual.scoped.normalization)"; fi
    if [ "$name" = frozen-markdown-coordinates ]; then filter="--test-name-pattern=^(?!.*a.scoped.transaction.preserves)"; fi
    if [ -z "$ONLY" ] || [ "$ONLY" = "$name" ]; then run "$state $name" $filter "./$L/$dir/🟦️.ts"; fi
  done
}
: > "$OUT"
if [ -n "$ONLY" ]; then OUT="/private/tmp/docs1021a/results-$ONLY.txt"; : > "$OUT"; fi
if [ "$MODE" = baseline ] || [ "$MODE" = both ]; then
  cp "$OV/$PRODUCT/README.md.pristine" "$OV/$PRODUCT/README.md"
  suite baseline
fi
if [ "$MODE" = applied ] || [ "$MODE" = both ]; then
  cp "$OV/$PRODUCT/README.md.pristine" "$OV/$PRODUCT/README.md"
  rm -f "$HERE/applied-rehearsal.json"
  (cd "$HERE" && python3 -I apply.py --root "$ROOT" --readme "$OV/$PRODUCT/README.md" --write | tail -1) >> "$OUT"
  suite applied
  cp "$OV/$PRODUCT/README.md.pristine" "$OV/$PRODUCT/README.md"
fi
echo "== done" >> "$OUT"
cat "$OUT"
