#!/bin/sh
# usage: chain2.sh <batch-file>...  like chain.sh, but before every attempt waits (max 90 min) until no `dependencies lock` process runs and `deps-d-tools/lane.ts locks` reports no frozen refusal (a peer's live lock regeneration kills every run otherwise); lock-refusal failures do not count as an attempt
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/fin-d"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
gate() {
  w=0
  while [ $w -lt 270 ]; do
    if ! ps -eo command | grep -q '[d]ependencies lock' && nice -n 10 bun "$C/deps-d-tools/lane.ts" locks 2>&1 | grep -q 'frozen-refusals=0'; then return 0; fi
    sleep 20; w=$((w+1))
  done
}
for batch in "$@"; do
while IFS='|' read -r id projects targets; do
  case "$id" in ''|'#'*) continue ;; esac
  n=1; lockretry=0
  while [ $n -le 3 ]; do
    gate
    log="$O/$id.b$n.$lockretry.log"
    st=$(date +%H:%M:%S)
    nice -n 10 bun repo --tool=bazel run-many --projects="$projects" --targets="$targets" --parallel=1 > "$log" 2>&1
    rc=$?
    printf '%s\tb%s\trc=%s\t%s-%s\t%s\n' "$id" "$n" "$rc" "$st" "$(date +%H:%M:%S)" "$targets" >> "$O/chain.tsv"
    [ $rc -eq 0 ] && break
    if grep -q 'Sole exact committed recomputable\|lock file of dependency scope\|Bun refused the frozen installation' "$log"; then lockretry=$((lockretry+1)); [ $lockretry -lt 8 ] && { sleep 60; continue; }; fi
    n=$((n+1))
  done
done < "$batch"
done
echo "done2 $(date +%H:%M:%S)" >> "$O/chain.tsv"
