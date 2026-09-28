#!/bin/zsh
# 🪞️ T14 overlay refresh (file clones only, no lane): sync the overlay to the live tree (tracked deletions since <base-commit>
# included), then apply the window-3 pass in `land-w3.py`'s order: F9 + the overlay-only id recorder, G12's authoring-seed pass,
# the class fix (derivable children follow their coordinate), P8 orphan, H9-L, 5b A + B1, 5b B2, item 6; the rule-22 test fixes
# (plugin tests, SDK lib reds) are live already and only verified. usage: overlay-apply.sh <tag> <base-commit>
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-overlay"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-logs"
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14
TAG="$1"; BASE="$2"
cd /Users/ueli/Documents/semio || exit 2
python3 $T/overlay.py sync "$O" "$BASE" > $L/$TAG-sync.txt 2>&1 || { echo "SYNC-FAILED"; exit 2; }; tail -2 $L/$TAG-sync.txt
apply() { local name=$1; shift; "$@" > $L/$TAG-$name-apply.txt 2>&1; local rc=$?; echo "$name $rc $(tail -1 $L/$TAG-$name-apply.txt)"; return $rc; }
apply f9 python3 $T/f9/content-id.py --write --root "$O" || exit 3
apply record python3 $T/f9/record-instrument.py --root "$O" || exit 3
apply g12 python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-g12/g12-authoring-seed.py --write --root "$O" || exit 3
apply follow python3 $T/p8class/follow-children.py --write --root "$O" || exit 3
P8_ROOT="$O" apply orphan python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-p8/patches/p8-orphan.py --write || exit 3
apply h9l python3 $T/h9l/kind-label-patch.py --apply --root "$O" || exit 3
apply 5b python3 $T/5b/dsl-value.py --write --root "$O" || exit 3
apply 5b2 python3 $T/5b/dsl-value-b2.py --write --root "$O" || exit 3
apply item6 python3 $T/item6/fallback-wrappers.py --write --root "$O" || exit 3
apply plugin-tests python3 $T/plugin-tests/plugin-lib-tests.py --write --root "$O" || exit 3
apply sdk-reds python3 $T/sdk-reds/sdk-lib-reds.py --write --root "$O" || exit 3
echo "APPLIED $TAG $(date +%T)"
