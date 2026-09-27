#!/bin/zsh
# 📊️ W3: one-line chain status (rebuild step, component tasks done, lane, errors) for the blocking monitor waits.
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-w3-logs"
P="${1:-b3}"
step=$(/usr/bin/grep -o 'rebuild-all [0-9]*/[0-9]* [a-z-]* ([a-z]*) \(started\|done in [0-9]* s\)' "$L/$P-rebuild-all.txt" 2>/dev/null | tail -1)
desc=$(/usr/bin/grep -c 'described [a-z0-9-]* (' "$L/$P-rebuild-all.txt" 2>/dev/null)
mat=$(/usr/bin/grep -c 'Materialized [a-z0-9-]* dev' "$L/$P-rebuild-all.txt" 2>/dev/null)
errs=$(/usr/bin/grep -c 'could not compile\|error\[E' "$L/$P-rebuild-all.txt" 2>/dev/null)
lane=$(/usr/bin/grep '\[lane\]' "$L/$P-warm-fwd.txt" 2>/dev/null | tail -1)
echo "$(date '+%T') step=[$step] described=$desc/60 materialized=$mat/60 errors=$errs lane=[$lane] rustc=$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ') load=$(sysctl -n vm.loadavg | cut -d' ' -f2)"
tail -2 "$L/chain-$P.txt"
