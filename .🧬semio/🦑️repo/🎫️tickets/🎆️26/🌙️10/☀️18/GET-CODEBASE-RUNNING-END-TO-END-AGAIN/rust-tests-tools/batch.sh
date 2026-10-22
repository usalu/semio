#!/bin/sh
# usage: batch.sh <log-name> <first-step> <last-step> [extra cargo args...]
# Runs ONE cargo test over the union of the crates of steps first..last (same workspace), so the memory-capacity queue is paid once.
# MODE=compile -> `build --tests --keep-going` (lists every compile error of every test target at once); default MODE=run -> `--no-fail-fast`.
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
name=$1; first=$2; last=$3; shift 3
[ -n "$SERIAL" ] && set -- "$@" -- --test-threads=1
ws=$(awk -F'\t' -v a="$first" '$1==a{print $5}' "$C/cargo-fix-tools/plan/steps.tsv")
crates=$(awk -F'\t' -v a="$first" -v b="$last" '$2=="test" && $1>=a && $1<=b && $5==ws0 {print $9}' ws0="$ws" "$C/cargo-fix-tools/plan/steps.tsv" | tr ' ' '\n' | awk '!s[$0]++')
[ -n "$CRATES" ] && crates=$CRATES
for x in $EXCLUDE; do crates=$(printf "%s\n" $crates | /usr/bin/grep -vx "$x"); done
pargs=""; for c in $crates; do pargs="$pargs -p $c"; done
export RUST_MIN_STACK=33554432
if [ "$MODE" = compile ]; then verb=build; mode="--tests --keep-going"; elif [ "$MODE" = nextest ]; then verb="nextest run"; mode="--no-fail-fast --failure-output final --final-status-level fail"; else verb=test; mode="--no-fail-fast"; fi
log="$C/🗑️generated/rust-tests-logs/$name.log"; tmp="$C/🗑️generated/rust-tests-logs/.$name.$$"
for attempt in 1 2 3 4; do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- $verb --manifest-path "$ws" $pargs $mode --message-format=short "$@" > "$tmp" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && /usr/bin/grep -q 'LeaseOrderErrorV1\|Host capacity wait cancelled\|Refusing to wait\|Original capture invocation changed\|Insufficient physical compiler\|Authenticated captured build' "$tmp" && ! /usr/bin/grep -q 'test result\|could not compile' "$tmp"; then cp "$tmp" "$log.infra$attempt"; echo "[rust-tests] infrastructure refusal, retry $attempt" >> "$log.status"; sleep 30; continue; fi
  break
done
mv "$tmp" "$log"
echo "$name rc=$rc" >> "$C/🗑️generated/rust-tests-logs/chain.status"
exit $rc
