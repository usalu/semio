#!/bin/sh
# usage: tst.sh <groups.tsv> <verdicts.tsv>  groups: id TAB manifest TAB "-p a -p b" TAB "--lib|--test n"; one `cargo test ... sqlite_snapshot` per group (retry infra refusals)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/plugc3-d"
V="$2"
TAB=$(printf '\t')
export SEMIO_BUILD_TERMINAL=false
while IFS="$TAB" read -r id manifest pkgs sel; do
  [ -n "$id" ] || continue
  /usr/bin/grep -q "^$id$TAB" "$V" 2>/dev/null && continue
  log="$G/t-$id.raw"
  n=0
  while [ $n -lt 8 ]; do
    n=$((n+1))
    # shellcheck disable=SC2086
    env -u SEMIO_SCRIPT_STALL_MS nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path "$manifest" $pkgs $sel sqlite_snapshot --no-fail-fast --message-format=short > "$log" 2>&1
    rc=$?
    if /usr/bin/grep -qE 'SEMIO_LEASE_ORDER|LeaseOrderErrorV1|Authenticated captured build command entry required|Insufficient physical compiler|Uncaptured implementation|Original Cargo workspace topology changed|Original capture invocation changed|ENOENT: no such file or directory|Refusing to wait|Stale loaded repository Store|Host capacity wait cancelled|Relative filesystem [a-z]* failed|Current shared Source-owner generation changed|outdated implementation|Original Script process terminated|Rust crate Source refused before compiler|Original Script process interrupted' "$log"; then
      echo "$id attempt $n refused rc=$rc" >> "$V.attempts"; sleep 60; continue
    fi
    break
  done
  passed=$(/usr/bin/grep -E '^test result:' "$log" | sed -E 's/.* ([0-9]+) passed; ([0-9]+) failed.*/\1/' | awk '{s+=$1} END{print s+0}')
  failed=$(/usr/bin/grep -E '^test result:' "$log" | sed -E 's/.* ([0-9]+) passed; ([0-9]+) failed.*/\2/' | awk '{s+=$1} END{print s+0}')
  errs=$(/usr/bin/grep -c ': error' "$log")
  printf '%s\trc=%s\tpassed=%s\tfailed=%s\terrorLines=%s\t%s %s\n' "$id" "$rc" "$passed" "$failed" "$errs" "$pkgs" "$sel" >> "$V"
done < "$1"
echo chain-done >> "$V"
