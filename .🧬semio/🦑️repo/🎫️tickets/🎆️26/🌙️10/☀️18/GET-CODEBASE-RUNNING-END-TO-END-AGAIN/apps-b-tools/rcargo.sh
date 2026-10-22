#!/bin/sh
# usage: rcargo.sh <log> <manifest-for-lock-refresh> <cargo args...>
cd /Users/ueli/Documents/semio || exit 1
log="$1"; man="$2"; shift 2
try=0
while [ $try -lt 5 ]; do
  try=$((try+1))
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$log" 2>&1
  rc=$?
  [ $rc -eq 0 ] && break
  if /usr/bin/grep -q "cannot update the lock file" "$log"; then
    nice -n 10 bun repo cargo --store=shared --configuration=prototype -- update --manifest-path "$man" --workspace --offline > "$log.update" 2>&1
    continue
  fi
  if /usr/bin/grep -q "SEMIO_LEASE_ORDER\|Uncaptured implementation\|Authorizer DATA\|Captured implementation Source\|Refusing to wait\|Host capacity wait cancelled\|Original capture invocation changed\|Insufficient physical compiler\|Source refused before compiler" "$log"; then sleep 120; continue; fi
  break
done
echo "rc=$rc try=$try" >> "$log.done"
