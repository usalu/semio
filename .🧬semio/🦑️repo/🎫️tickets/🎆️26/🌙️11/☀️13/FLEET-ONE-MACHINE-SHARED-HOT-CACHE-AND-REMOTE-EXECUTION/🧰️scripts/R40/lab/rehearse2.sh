#!/bin/zsh
S=${0:A:h}
LAND="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R40/land.py"
case "$S" in /private/tmp/claude-501/*) ;; *) echo "scratch only"; exit 2;; esac
[[ -d $S/rh/orig ]] || { echo "run rehearse.sh first"; exit 1; }
rm -rf $S/rh2 && mkdir -p $S/rh2 && cp -R $S/rh/orig $S/rh2/live
echo "== step 1: R29 + R37 + R39 land first (R40 skipped)"
LAND_SKIP=R40 python3 -I "$LAND" --land --root $S/rh2/live --authorized --no-tests --merge | grep -E "^sets|^landed"
cp -R $S/rh2/live $S/rh2/mid
echo "== step 2: R40 alone"
python3 -I "$LAND" --land --root $S/rh2/live --authorized --no-tests --merge | grep -E "^sets|^landed|stamps|output-stamps"
grep -o 'script-inputs/[0-9]*' $S/rh2/mid/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧬️schema/🔣️.json | sort -u | head -2 | tr '\n' ' '; echo "(before R40)"
grep -o 'script-inputs/[0-9]*' $S/rh2/live/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧬️schema/🔣️.json | sort -u | head -2 | tr '\n' ' '; echo "(after R40)"
echo "== step 3: second run"; python3 -I "$LAND" --land --root $S/rh2/live --authorized --no-tests --merge | tail -1
B=$(find $S/rh2/live -type d -name "R40-landing-*" | sort | tail -1)
echo "== step 4: rollback of step 2"; python3 -I "$LAND" --rollback "$B" --root $S/rh2/live | tail -1
diff -rq $S/rh2/live $S/rh2/mid -x 'R40-landing-*' -x landings.md -x landing.lock | grep -v "🗑️generated" | head -5; echo "rollback compare done"
