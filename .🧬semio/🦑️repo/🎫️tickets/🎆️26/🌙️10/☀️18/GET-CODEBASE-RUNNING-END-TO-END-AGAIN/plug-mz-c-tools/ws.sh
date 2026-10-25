#!/bin/sh
# usage: ws.sh MODE(check|test) TAG MANIFEST   (one workspace, retries on infra refusals, lock refresh on stale lock)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/plug-mz-c"
MODE=$1; TAG=$2; M=$3
LOG="$G/$TAG.$MODE.log"
n=0
while [ $n -lt 8 ]; do
  n=$((n+1))
  : > "$LOG"
  if [ "$MODE" = test ]; then
    RUST_MIN_STACK=33554432 nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" --workspace --no-fail-fast --message-format=short > "$LOG" 2>&1
  else
    nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check --manifest-path "$M" --workspace --all-targets --keep-going --message-format=short > "$LOG" 2>&1
  fi
  rc=$?
  if [ $rc -ne 0 ] && /usr/bin/grep -q "cannot update the lock file" "$LOG"; then
    nice -n 10 bun repo cargo --store=shared --configuration=prototype -- update --manifest-path "$M" --workspace --offline > "$LOG.update" 2>&1
    continue
  fi
  if [ $rc -ne 0 ] && /usr/bin/grep -qE 'SEMIO_LEASE_ORDER|Authenticated captured build command entry required|Insufficient physical compiler|"await" can only be used|Expected ";" but found|Uncaptured implementation|Original Cargo workspace topology changed|Original capture invocation changed|ENOENT: no such file or directory|Authorizer DATA|Captured implementation Source|Refusing to wait|Host capacity wait cancelled|Source refused before compiler' "$LOG"; then
    echo "attempt $n refused rc=$rc" >> "$LOG.attempts"; sleep 90; continue
  fi
  break
done
err=$(/usr/bin/grep -c "error\[E\|: error:\|^error: could not compile" "$LOG")
printf '%s\t%s\t%s\t%s\terrors=%s\ttry=%s\n' "$MODE" "$rc" "$TAG" "$M" "$err" "$n" >> "$G/summary.tsv"
