#!/bin/sh
# usage: check-all.sh <list> <startIndex> <endIndex> [mode=check|test]
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/apps-b-1022"
LIST="$1"; S="${2:-1}"; E="${3:-100000}"; MODE="${4:-check}"
SUM="$G/$MODE-summary.tsv"
run() { nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" ; }
i=0
while IFS= read -r m; do
  i=$((i+1))
  [ "$i" -lt "$S" ] && continue
  [ "$i" -gt "$E" ] && break
  [ -f "$G/stop" ] && break
  log="$G/$MODE-$i.log"
  echo "$m" > "$G/$MODE-$i.manifest"
  try=0
  while [ $try -lt 5 ]; do
    try=$((try+1))
    if [ "$MODE" = test ]; then
      RUST_MIN_STACK=33554432 run test --manifest-path "$m" --workspace --no-fail-fast > "$log" 2>&1
    else
      run check --manifest-path "$m" --workspace --all-targets --keep-going --message-format=short > "$log" 2>&1
    fi
    rc=$?
    if [ $rc -ne 0 ] && /usr/bin/grep -q "cannot update the lock file" "$log"; then
      run update --manifest-path "$m" --workspace --offline > "$G/update-$i.log" 2>&1
      continue
    fi
    if [ $rc -ne 0 ] && /usr/bin/grep -q "SEMIO_LEASE_ORDER\|Uncaptured implementation\|Authorizer DATA\|Captured implementation Source\|Refusing to wait\|Host capacity wait cancelled\|Original capture invocation changed\|Insufficient physical compiler\|Source refused before compiler" "$log"; then
      sleep 120; continue
    fi
    break
  done
  err=$(/usr/bin/grep -c "error\[E\|: error:\|^error: could not compile" "$log")
  printf '%s\t%s\t%s\terrors=%s\ttry=%s\n' "$i" "$rc" "$m" "$err" "$try" >> "$SUM"
done < "$LIST"
echo done >> "$SUM"
