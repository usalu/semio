#!/bin/zsh
# usage: zsh loop.sh <binary> <iterations> <lane> [laws...]; hangs (>30 s) are sampled and left alive
bin="$1"; n="$2"; lane="$3"; shift 3
out="/Users/ueli/Documents/semio/.tmp-ticket/wp-h6/generated"
laws=("$@")
[ ${#laws} -eq 0 ] && laws=(tests::document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped tests::document_open_plan_admin_revocation_invalidates_session_and_share_bindings tests::document_open_plan_receipt_exchange_admits_one_exact_bounded_secret_free_socket_grant)
export RUST_MIN_STACK=268435456
cd "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust"
hangs=0; fails=0
for i in $(seq 1 $n); do
  for law in $laws; do
    "$bin" "$law" --exact --test-threads=1 > "$out/lane-$lane.last.txt" 2>&1 &
    pid=$!; t=0
    while kill -0 $pid 2>/dev/null && [ $t -lt 300 ]; do sleep 0.1; t=$((t+1)); done
    if kill -0 $pid 2>/dev/null; then
      hangs=$((hangs+1)); echo "HANG lane=$lane iter=$i law=$law pid=$pid" >> "$out/hangs.txt"
      sample $pid 3 -file "$out/hang-$lane-$i-sample.txt" > /dev/null 2>&1
      echo "HANG lane=$lane iter=$i law=$law pid=$pid (left alive)"; exit 3
    fi
    wait $pid || { fails=$((fails+1)); cp "$out/lane-$lane.last.txt" "$out/fail-$lane-$i.txt"; }
  done
done
echo "DONE lane=$lane iterations=$n hangs=$hangs fails=$fails"
