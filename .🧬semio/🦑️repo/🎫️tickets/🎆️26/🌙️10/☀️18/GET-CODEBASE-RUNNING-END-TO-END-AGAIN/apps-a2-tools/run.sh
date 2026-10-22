#!/bin/bash
# usage: run.sh <list.tsv(plugin<TAB>dir)> <results.tsv> [mode=check|update-check]
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="$C/🗑️generated/apps-a2-x"
LIST="$1"; OUT="$2"
infra='Insufficient physical compiler|LeaseOrderErrorV1|SEMIO_LEASE_ORDER|Refusing to wait|Authorizer DATA directory changed|Uncaptured implementation|Captured implementation Source differs|Captured require declaring|admission\] Uncaptured|Host capacity wait cancelled|Original capture invocation changed|Stale loaded repository Store'
run() {
  local log="$1"; shift
  local rc=1
  for t in 1 2 3 4 5 6 7 8 9 10; do
    nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$log" 2>&1
    rc=$?
    [ $rc = 0 ] && return 0
    /usr/bin/grep -qE "$infra" "$log" || return $rc
    sleep 60
  done
  return $rc
}
while IFS=$'\t' read -r plugin dir; do
  [ -f "$L/stop" ] && break
  key=$(echo "$dir" | md5 -q | cut -c1-8)
  /usr/bin/grep -qF "$dir	" "$OUT" 2>/dev/null && continue
  start=$(date +%s)
  upd=skip
  if [ ! -f "$L/$key-update.done" ]; then run "$L/$key-update.log" update --workspace --offline --manifest-path "$dir/Cargo.toml"; upd=$?; [ $upd = 0 ] && touch "$L/$key-update.done"; fi
  run "$L/$key-check.log" check --workspace --keep-going --message-format=short --manifest-path "$dir/Cargo.toml"; chk=$?
  errs=$(/usr/bin/grep -cE '^[^ ]*\.rs:[0-9]+:[0-9]+: error' "$L/$key-check.log")
  echo "$dir	$plugin	$key	update_rc=$upd	check_rc=$chk	errors=$errs	$(( $(date +%s) - start ))s" >> "$OUT"
done < "$LIST"
echo DONE >> "$OUT"
