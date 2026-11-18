#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
R=/Users/ueli/Documents/semio
T=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
BUN=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R147
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
LIVE="$L/🧪️testing/🔬️slice-lab/👷️machine/🟦️.ts"
while ! mkdir "$T/landing.lock" 2>/dev/null; do echo "held: $(cat $T/landing.lock/owner 2>/dev/null)"; sleep 20; done
echo "claude-R147 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
cp "$LIVE" $SC/machine.backup
if [ "$(shasum -a 256 "$LIVE" | cut -c1-12)" = "$(shasum -a 256 $SC/base/machine.ts | cut -c1-12)" ]; then cp $SC/stage/machine.ts "$LIVE"; else cp "$LIVE" $SC/m.tmp; git merge-file $SC/m.tmp $SC/base/machine.ts $SC/stage/machine.ts || { echo conflict; rm -r "$T/landing.lock"; exit 3; }; cp $SC/m.tmp "$LIVE"; fi
$BUN "$L/🧪️testing/🌱️genuine-birth/📜️script.ts" --file="$L/🧪️testing/🔬️slice-lab/🧪️tests/🟦️.ts" --birth=setup --root=$R > $SC/winm.txt 2>&1; rc=$?
grep -E '^ *[0-9]+ (pass|fail)' $SC/winm.txt | tr '\n' ' '
if [ $rc != 0 ]; then echo RED; cp $SC/machine.backup "$LIVE"; else echo GREEN; fi
echo "started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
