#!/bin/zsh
# land.sh <group> <testfile>...   lock, apply, run tests in parallel via genuine-birth harness, restore on red, release
G=$1; shift
R=/Users/ueli/Documents/semio
FT="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=${0:A:h}
until mkdir "$FT/landing.lock" 2>/dev/null; do sleep 5; done
echo claude-R153 > "$FT/landing.lock/owner"; echo "lock $(date -u +%H:%M:%S)"
release(){ rm -f "$FT/landing.lock/owner"; rmdir "$FT/landing.lock"; echo "released $(date -u +%H:%M:%S)"; }
python3 $S/apply.py $G apply || { release; exit 3; }
i=0; for f in "$@"; do i=$((i+1)); ( $S/run.sh "$f" > $S/land-$G-$i.txt 2>&1; echo "$(grep -E '^ *[0-9]+ (pass|fail)$' $S/land-$G-$i.txt | tail -2 | tr '\n' ' ') :: ${f:t:h}" > $S/land-$G-$i.sum ) & done
wait
cat $S/land-$G-*.sum
if [ -n "$LAND_MAXFAIL" ]; then tot=$(cat $S/land-$G-*.sum | sed -E 's/.* ([0-9]+) fail.*/\1/' | paste -sd+ - | bc); echo "fails=$tot max=$LAND_MAXFAIL"; if [ "$tot" -gt "$LAND_MAXFAIL" ]; then echo RED; python3 $S/apply.py $G restore; fi
elif grep -q "[1-9][0-9]* fail" $S/land-$G-*.sum; then echo RED; python3 $S/apply.py $G restore; fi
release
