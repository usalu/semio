#!/bin/bash
# usage: run2.sh <list.tsv(plugin<TAB>dir)> <results.tsv>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="$C/🗑️generated/apps-a2-x"
LIST="$1"; OUT="$2"
infra='Insufficient physical compiler|LeaseOrderErrorV1|SEMIO_LEASE_ORDER|Refusing to wait|Authorizer DATA directory changed|Uncaptured implementation|Captured implementation Source differs|Captured require declaring|admission\] Uncaptured|Host capacity wait cancelled|Original capture invocation changed|Stale loaded repository Store'
real='could not compile|error\[E[0-9]|^[^ ]+:[0-9]+:[0-9]+: error|failed to (select a version|load manifest|parse manifest)|no matching package'
cg() { nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@"; }
upd() { local dir="$1" log="$2" rc=1; for t in 1 2 3 4 5 6; do cg update --workspace --offline --manifest-path "$dir/Cargo.toml" > "$log" 2>&1; rc=$?; [ $rc = 0 ] && return 0; /usr/bin/grep -qE "$infra" "$log" || return $rc; sleep 60; done; return $rc; }
chk() {
  local dir="$1" log="$2" rc=1
  for t in 1 2 3 4 5 6 7 8 9 10 11 12; do
    cg check --workspace --keep-going --message-format=short --manifest-path "$dir/Cargo.toml" > "$log" 2>&1; rc=$?
    [ $rc = 0 ] && return 0
    if /usr/bin/grep -q 'cannot update the lock file' "$log"; then upd "$dir" "$log.update" ; continue; fi
    /usr/bin/grep -qE "$real" "$log" && return $rc
    sleep 60
  done
  return $rc
}
while IFS=$'\t' read -r plugin dir; do
  [ -f "$L/stop" ] && break
  key=$(echo "$dir" | md5 -q | cut -c1-8)
  /usr/bin/grep -qF "$dir	" "$OUT" 2>/dev/null && continue
  start=$(date +%s)
  chk "$dir" "$L/$key-check.log"; c=$?
  errs=$(/usr/bin/grep -cE '^[^ ]*\.rs:[0-9]+:[0-9]+: error' "$L/$key-check.log")
  echo "$dir	$plugin	$key	check_rc=$c	errors=$errs	$(( $(date +%s) - start ))s" >> "$OUT"
done < "$LIST"
echo DONE >> "$OUT"
