#!/bin/sh
# usage: step.sh <step-id> [extra cargo args] (reads cargo-fix plan steps.tsv; runs cargo test --no-fail-fast for that chunk; retries on infrastructure refusals)
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
id=$1; shift
row=$(awk -F'\t' -v id="$id" '$1==id' "$C/cargo-fix-tools/plan/steps.tsv")
ws=$(printf '%s' "$row" | cut -f5); crates=$(printf '%s' "$row" | cut -f9)
[ -n "$CRATES" ] && crates=$CRATES
pargs=""; for c in $crates; do pargs="$pargs -p $c"; done
export RUST_MIN_STACK=33554432
tmp="$C/🗑️generated/rust-tests-logs/.$id.$$"
for attempt in 1 2 3 4; do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ws" $pargs --no-fail-fast --message-format=short "$@" > "$tmp" 2>&1
  rc=$?
  cat "$tmp"
  if [ $rc -ne 0 ] && /usr/bin/grep -q 'LeaseOrderErrorV1\|Host capacity wait cancelled\|Refusing to wait' "$tmp" && ! /usr/bin/grep -q 'test result\|could not compile' "$tmp"; then echo "[rust-tests] infrastructure refusal, retry $attempt"; sleep 20; continue; fi
  break
done
rm -f "$tmp"
exit $rc
