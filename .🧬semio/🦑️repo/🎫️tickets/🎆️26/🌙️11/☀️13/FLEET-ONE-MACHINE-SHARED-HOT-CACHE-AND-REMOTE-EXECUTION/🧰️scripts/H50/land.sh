#!/bin/zsh
# lands apply-process.py then apply-product.py on the live tree under the five landing locks; takes only locks it created itself
T="/Users/ueli/Documents/semio/$(cat /tmp/t1127h50)"
H=${0:A:h}
P="🧰️framework/🔨️modules/🏃️process"
Q="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
cd /Users/ueli/Documents/semio || exit 1
taken=()
release(){ for a in $taken; do rm -f "$T/landing.lock.$a/owner"; rmdir "$T/landing.lock.$a" 2>/dev/null; done; }
for area in process test-file remote-execution suites caching; do
  if mkdir "$T/landing.lock.$area" 2>/dev/null; then echo "H50 $(date +%T) suite offload + stretch inheritance" > "$T/landing.lock.$area/owner"; taken+=($area); else echo "lock $area held: $(cat $T/landing.lock.$area/owner 2>/dev/null)"; release; exit 2; fi
done
DRY=1 python3 $H/apply-process.py $P && DRY=1 python3 $H/apply-product.py $Q || { release; echo "anchors moved, nothing written"; exit 3; }
python3 $H/apply-process.py $P && python3 $H/apply-product.py $Q; rc=$?
release
echo "landed rc=$rc $(date +%T)"
