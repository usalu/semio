#!/bin/zsh
# R189 landing: live <- staged for the rows of land.tsv (live, staged, base; empty base = new file, removed on restore). Window hh:00-05 / hh:30-35 UTC only. Lock landing.lock with an exit trap. Live sha is compared with the base BEFORE and again UNDER the lock. Gate: fail counts may not grow over the baseline; the new laws and the R185 ready law must have 0 fails.
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R189
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
rows=("${(@f)$(cat $S/land.tsv)}")
shas(){ for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; st=${rest%%$'\t'*}; base=${rest#*$'\t'}
  if [ -n "$base" ]; then [ "$(shasum -a 256 < $base)" = "$(shasum -a 256 < $live)" ] || { echo "live file moved: $live"; return 2; }; else [ ! -e "$live" ] || { echo "new file exists: $live"; return 2; }; fi; done; return 0; }
shas || exit 2
[ "$1" = "check" ] && { echo "bases match"; exit 0; }
m=$(date -u +%M); m=$((10#$m)); if ! { [ $m -lt 5 ] || { [ $m -ge 30 ] && [ $m -lt 35 ]; }; }; then echo "outside the landing window ($m)"; exit 4; fi
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 120 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R189 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock" 2>/dev/null; }
trap release EXIT
restore(){ for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; base=${rest#*$'\t'}; if [ -n "$base" ]; then cp "$base" "$live"; else rm -f "$live"; rmdir "$(dirname $live)" 2>/dev/null; rmdir "$(dirname $(dirname $live))" 2>/dev/null; fi; done; echo "restored $(date -u +%H:%M:%S)"; }
fail(){ echo "RED: $1"; restore; exit 3; }
shas || { echo "moved under the lock"; exit 2; }
for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; st=${rest%%$'\t'*}; mkdir -p "$(dirname $live)"; cp "$st" "$live" || fail copy; done
for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; st=${rest%%$'\t'*}; [ "$(shasum -a 256 < $st)" = "$(shasum -a 256 < $live)" ] || fail "copy differs $live"; done
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 TMPDIR=$S/tmp/
while read f; do
  d=$(dirname "$f"); b=$(basename "$f"); out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./$b --timeout 180000 2>&1)
  line="$(basename $(dirname $d))/$(basename $d)/$b $(echo "$out"|/usr/bin/grep -a -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; echo "$line"
  nf=$(echo "$out"|/usr/bin/grep -a -E '^ *[0-9]+ fail'|tr -d ' fail'); nf=${nf:-0}
  case "$f" in *windows-shaped*|*🪟️volume*|*🕳️disk-unknown*|*💽️disk-recovery*) bf=0;; *) bf=$(/usr/bin/grep -F "$f"$'\t' $S/baseline.tsv | head -1 | cut -f2); bf=${bf:-0};; esac
  [ "$nf" -le "$bf" ] || { echo "$out" | /usr/bin/grep -a -E '^\(fail\)' | head; fail "$f worse ($nf > $bf)"; }
done < $S/gate-all.list
echo "GREEN $START $(date -u +%H:%M:%S)"
