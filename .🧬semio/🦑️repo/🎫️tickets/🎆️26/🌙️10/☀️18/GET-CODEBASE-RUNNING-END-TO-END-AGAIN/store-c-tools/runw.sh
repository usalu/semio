#!/bin/sh
# usage: runw.sh <log> <crate> <max-test-binary-minutes> [extra test-binary args, e.g. a test filter]
# One test crate: cargo test with 4 test threads, plus a watchdog that kills ONLY the descendant test binary of this run after <max> minutes and records the hung tests in <log>.hung
cd /Users/ueli/Documents/semio || exit 2
log=$1; crate=$2; max=$3; shift 3
rm -f "$log.status" "$log.hung"
me=$$
descendants() { for c in $(pgrep -P "$1"); do echo "$c"; descendants "$c"; done; }
( while [ ! -f "$log.status" ]; do
    sleep 20
    for p in $(descendants $me); do
      cmd=$(ps -o command= -p "$p" 2>/dev/null)
      case "$cmd" in */build/$crate/*/out/*) ;; *) continue;; esac
      case "$cmd" in *cargo*) continue;; esac
      et=$(ps -o etime= -p "$p" | tr -d ' ')
      mins=$(echo "$et" | awk -F'[:-]' '{n=NF; s=$n+60*$(n-1); if(n>=3)s+=3600*$(n-2); if(n>=4)s+=86400*$(n-3); print int(s/60)}')
      if [ "${mins:-0}" -ge "$max" ]; then
        { echo "killed pid $p after $mins min: $cmd"; /usr/bin/grep -E 'has been running for over|^test .* \.\.\. ' "$log" | tail -40; } > "$log.hung"
        kill "$p"
      fi
    done
  done ) &
export RUST_MIN_STACK=33554432
for attempt in $(seq 1 30); do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml --no-fail-fast --message-format=short -p "$crate" -- --test-threads=4 "$@" > "$log" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && /usr/bin/grep -q 'LeaseOrderErrorV1\|Host capacity wait cancelled\|Refusing to wait\|Insufficient physical compiler\|Uncaptured implementation import refused\|Invalid repository Cargo launch\|SyntaxError: Export named\|SyntaxError: Unexpected' "$log" && ! /usr/bin/grep -q 'test result\|could not compile' "$log"; then echo "retry $attempt" >> "$log.retries"; sleep 45; continue; fi
  break
done
echo "rc=$rc" > "$log.status"
