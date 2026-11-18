#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R138
STEP=${1:-all}
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R138
LIB=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
M=$(date -u +%M); [ $((10#$M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R138 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
fail(){ echo "RED: $1"; python3 $D/$STEP.py restore; rm -r "$T/landing.lock"; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SC/backup $SC/backup2 $SC/backup3; python3 $D/$STEP.py apply || fail apply
export BUN LIB SC
rm -f $SC/res.*
cat $D/$STEP.tests | xargs -P4 -n1 $D/one.sh
bad=0
for f in $(cat $D/$STEP.tests); do h=$(echo "$f" | md5 -q); n=$(awk '/ fail$/{print $1}' $SC/res.$h); allowed=0; [[ $f == *🌐️host ]] && allowed=1; echo "$f $(tr '\n' ' ' < $SC/res.$h)"; [ "${n:-9}" -le $allowed ] || bad=1; done
[ $bad = 0 ] || fail tests
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
