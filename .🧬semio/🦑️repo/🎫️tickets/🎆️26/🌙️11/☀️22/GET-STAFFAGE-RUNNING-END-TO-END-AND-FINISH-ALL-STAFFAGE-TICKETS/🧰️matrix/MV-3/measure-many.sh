#!/bin/zsh
# 📐️ The guard of the other tests on trial copies, before anything is written to the catalog (meant for one grant of 2500 MB): for every plant `M/measure.ts` (silhouette guard of the body, `validateAsset`, three-views sweep) on the backup and on `$GEN/try/<id>.json`; the verdict lines go to `$GEN/log/measure-try.txt`. measure-many.sh <id>…   (plants named in the file `$GEN/retry` get their `trial.sh` run again first: a trial that failed earlier in the same grant)
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
q() { if [[ -n "$MV3_GRANT" ]]; then local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; else "$W/q.sh" "$@"; fi }
if [[ -f "$GEN/retry" ]]; then for r in $(cat "$GEN/retry"); do { echo "== $r $(date +%H:%M) (again)"; "$W/trial.sh" "$r"; } >> "$GEN/log/trials.txt" 2>&1; done; mv "$GEN/retry" "$GEN/log/retried.txt"; fi
for a in "$@"; do
  q 2500 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$GEN/try/$a.json" > "$GEN/log/$a-measure-try.log" 2>&1
  { echo "== $a exit $?"; grep -E "silhouette:|validate:|sweep:|!" "$GEN/log/$a-measure-try.log" | cut -c1-300; } >> "$GEN/log/measure-try.txt"
done
echo "measure done: $*"
