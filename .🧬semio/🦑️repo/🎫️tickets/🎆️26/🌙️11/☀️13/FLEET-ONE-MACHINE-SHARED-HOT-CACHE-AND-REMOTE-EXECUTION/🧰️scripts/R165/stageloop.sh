#!/bin/zsh
# usage: stageloop.sh <testdir-relative-to-stage> <file>   copies the directory of any missing module from live, retries
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R165/stage
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
ART=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R165/art; mkdir -p $ART
for i in {1..120}; do
 out=$(cd "$S/$1" && env -u SEMIO_FLEET_GROUP SEMIO_TEST_ARTIFACT_DIR=$ART nice -n 10 $BUN test "${@:2}" --timeout 180000 2>&1)
 miss=$(echo "$out" | grep -o "Cannot find module '[^']*' from '[^']*'" | head -1)
 if [ -z "$miss" ]; then echo "$out" | grep -E "^\(fail\)|^\(pass\)| pass| fail|Ran |error:" | head -40; exit 0; fi
 rel=$(echo "$miss" | sed -E "s/Cannot find module '([^']*)' from '([^']*)'/\1/"); from=$(echo "$miss" | sed -E "s/.* from '([^']*)'/\1/")
 case "$rel" in .*) ;; *) echo "bare module missing: $rel"; exit 2;; esac
 tgt=$(python3 -c 'import os,sys;print(os.path.normpath(os.path.join(os.path.dirname(sys.argv[2]),sys.argv[1])))' "$rel" "$from")
 src="/Users/ueli/Documents/semio${tgt#$S}"; d=$(dirname "$tgt"); sd=$(dirname "$src")
 [ -e "$src" ] || { echo "no live source for ${tgt#$S}"; exit 2; }
 mkdir -p "$d"; cp -Rn "$sd/." "$d/" 2>/dev/null; echo "staged ${d#$S}" | cut -c1-200
done
