#!/bin/sh
# usage: chk.sh <rows.tsv> <verdicts.tsv>  rows: manifest TAB package ; one `cargo check -p pkg --keep-going` per row, sequential, retry infra refusals
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/plugc3-d"
V="$2"
TAB=$(printf '\t')
export SEMIO_BUILD_TERMINAL=false
while IFS="$TAB" read -r manifest pkg; do
  [ -n "$pkg" ] || continue
  /usr/bin/grep -q "^$pkg$TAB" "$V" 2>/dev/null && continue
  log="$G/c-$pkg.raw"
  n=0
  while [ $n -lt 8 ]; do
    n=$((n+1))
    env -u SEMIO_SCRIPT_STALL_MS nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check --manifest-path "$manifest" -p "$pkg" --keep-going --message-format=short > "$log" 2>&1
    rc=$?
    if /usr/bin/grep -qE 'SEMIO_LEASE_ORDER|LeaseOrderErrorV1|Authenticated captured build command entry required|Insufficient physical compiler|Uncaptured implementation|Original Cargo workspace topology changed|Original capture invocation changed|ENOENT: no such file or directory|Refusing to wait|Stale loaded repository Store|Host capacity wait cancelled|Relative filesystem [a-z]* failed|Current shared Source-owner generation changed|outdated implementation|Original Script process terminated|Rust crate Source refused before compiler|Original Script process interrupted' "$log"; then
      echo "$pkg attempt $n refused rc=$rc" >> "$V.attempts"; sleep 60; continue
    fi
    break
  done
  errs=$(/usr/bin/grep -c ': error' "$log")
  printf '%s\trc=%s\terrorLines=%s\n' "$pkg" "$rc" "$errs" >> "$V"
done < "$1"
echo chain-done >> "$V"
