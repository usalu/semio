#!/bin/zsh
# H60 landing (graph area; ONLY after the coordinator lifts the 08:15 seal quiet window): resolve-row journal vouching + tests
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
H=${0:A:h}
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
cd /Users/ueli/Documents/semio || exit 1
if mkdir "$T/landing.lock.graph" 2>/dev/null; then echo "H60 $(date +%T) resolve-row journal vouching" > "$T/landing.lock.graph/owner"; else echo "lock graph held: $(cat $T/landing.lock.graph/owner 2>/dev/null)"; exit 2; fi
release(){ rm -f "$T/landing.lock.graph/owner"; rmdir "$T/landing.lock.graph" 2>/dev/null; }
DRY=1 python3 $H/apply.py $B || { release; echo "nothing written"; exit 3; }
python3 $H/apply.py $B; rc=$?
release
echo "landed rc=$rc $(date +%T); next: append landings.md entry, repo test-file the 3 tests, repo typecheck, dangling-imports.py"
