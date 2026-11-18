#!/bin/zsh
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 3; done
echo "claude-R149 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
python3 "$T/🧰️scripts/R149/$1.py" restore
export BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f LIB=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R149/res-restore
mkdir -p $SC; rm -f $SC/res.* 2>/dev/null
$T/🧰️scripts/R149/one.sh "../🧪️testing/🔬️slice-lab/🧪️tests"; cat $SC/res.*
rm -r "$T/landing.lock"
