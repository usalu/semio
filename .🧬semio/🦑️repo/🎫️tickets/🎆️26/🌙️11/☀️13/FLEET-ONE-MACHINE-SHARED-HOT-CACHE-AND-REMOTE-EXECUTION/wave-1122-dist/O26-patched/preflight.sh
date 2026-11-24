#!/bin/zsh
# O26 pre-flight before restarting the play chain with this set. usage: preflight.sh <clone root> [site-build log]   READ ONLY (df, grep, the lanes' recorded sizes; no tree walk).
# It answers: would the new run refuse release-00-b0 at the storage gate? Image free room against the batch's prediction less what the old run left in the shared build directory.
# The driver measures the exact present bytes itself (a bounded walk of the unit outputs); here the recorded lane size (`lane.size`, without incremental sessions) stands in for it.
set -e
clone=${1:?clone root}
log=${2:-/Users/ueli/Documents/semio/.🧬semio/🌐hub/play-cdn-1122/pinned/site-build.log}
mount="$clone/.cache/b/r/m"
reserve=2048
line=$(grep -h "release-00-b0: .* component(s), predicted" "$log" | tail -1)
[[ -n $line ]] || { echo "no 'release-00-b0 ... predicted' line in $log: the new run plans it itself, nothing to compare"; exit 0; }
predicted=$(print -r -- "$line" | sed -E 's/.*predicted ([0-9]+) MiB.*/\1/')
free=$(df -m "$mount" | awk 'NR==2{print $4}')
present=0
for size in "$mount"/u/l/*/lane.size(N); do
 bytes=$(sed -E 's/.*"bytes":([0-9]+).*/\1/' "$size"); inc=$(sed -E 's/.*"incrementalBytes":([0-9]+).*/\1/' "$size")
 present=$((present + (bytes - inc) / 1048576))
done
credit=$(( present < predicted ? present : predicted ))
need=$(( predicted - credit + reserve ))
echo "image $mount: ${free} MiB free; release-00-b0 predicted ${predicted} MiB; recorded lane content ${present} MiB; the gate will want $((predicted - credit)) MiB + ${reserve} MiB reserve = ${need} MiB"
echo "without the O26 credit it would want $((predicted + reserve)) MiB"
if (( free >= need )); then echo "OK: b0 is admitted (${free} >= ${need})"; else echo "REFUSED: b0 would be refused, $((need - free)) MiB missing; free room first or wait"; exit 1; fi
