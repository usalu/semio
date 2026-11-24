#!/bin/zsh
# 🏗️ Writes the repair ops of `tower-crane` of batch R4-X (ops/tower-crane.json, applied after the ops of SAM, R2-M and R3-M), authored against the catalog file as it stood before this batch (ASSET_FILE, default 🗑️generated/silhouette/R4-X/before/tower-crane.json): the parametric ops (tower-crane.py), then per scale and view the bars the drawing leaves out (cover.ts; finer thresholds at 1:50), then flat outlines where a flat shape is right: in the plan (the slewing unit turns about the vertical) and on the fixed base (outline.ts).
source "${0:A:h}/../env.sh"
export ASSET_FILE="${ASSET_FILE:-$GEN/before/tower-crane.json}"
O="$W/ops/tower-crane.json"; mkdir -p "$GEN/make"
python3 "$W/make/tower-crane.py" || exit 1
cd "$S/📦️packages/🟦️typescript" || exit 1
for s in ${=SCALES:-500 200 100 50}; do [[ $s == 50 ]] && T=(0.01 4) || T=(0.04); for v in elevation front plan; do
  SKIP="^rope-" timeout 2400 "$B" "$W/cover.ts" "$ASSET_FILE" $s $v "$O" "$O" $T > "$GEN/make/cover-$s-$v.log" 2>&1 || { tail -5 "$GEN/make/cover-$s-$v.log"; exit 1 }
  echo "$(grep -c "^  added" "$GEN/make/cover-$s-$v.log") added, $(grep -c "^  moved" "$GEN/make/cover-$s-$v.log") moved, $(grep -c "^  redrawn" "$GEN/make/cover-$s-$v.log") redrawn"; grep -E "^==|uncovered, no bar|beside" "$GEN/make/cover-$s-$v.log" | cut -c1-${CUT:-700}
done; done
ol() { timeout 1500 "$B" "$W/outline.ts" "$ASSET_FILE" "$1" "$2" "$O" --base "$O" "${@:3}" > "$GEN/make/outline-$1-$2.log" 2>&1 || { tail -5 "$GEN/make/outline-$1-$2.log"; exit 1 }; grep -v "placed off 0.00" "$GEN/make/outline-$1-$2.log" }
for s in ${=SCALES:-500 200 100 50}; do
  case $s in
    500|200) ol $s plan "winch-vent-stack=winch-vent-stack@z=3.45,tol=1";;
    100|50) ol $s plan "winch-vent-stack=winch-vent-stack@z=3.45,tol=0.5" "skip-lug-r=skip-lug-r@like=bail-leg-r" "skip-lug-l=skip-lug-l@like=bail-leg-l" "trolley-wheels-r=trolley-wheel-r-1+trolley-wheel-r-2+trolley-wheel-r-1-axle+trolley-wheel-r-2-axle@tol=0.5" "trolley-wheels-l=trolley-wheel-l-1+trolley-wheel-l-2+trolley-wheel-l-1-axle+trolley-wheel-l-2-axle@tol=0.5";;
  esac
done
if [[ " ${=SCALES:-50} " == *" 50 "* ]]; then
  ol 50 plan "skip-rim=skip-rim@like=skip-body,z=4.45,tol=0.5"
  ol 50 front "cross-base=cross-base-x+cross-base-z@tol=0.3" "ballast-slab=ballast-x-pos+ballast-x-neg+ballast-z-pos+ballast-z-neg@tol=0.3"
  ol 50 elevation "cross-base=cross-base-x+cross-base-z@tol=0.3" "ballast-slab=ballast-x-pos+ballast-x-neg+ballast-z-pos+ballast-z-neg@tol=0.3"
fi
true
