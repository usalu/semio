#!/bin/zsh
# 🧪️ Proof of the s15 renderer test-isolation set on the LIVE tree (native lane, shared build-fleet-b, U6 target): builds the
# renderer wgpu lib test binary, then (1) the slot-table guards, the isolated saturation law and the frame-job culprit→victim pair,
# (2) the async-boundary group 40× with 4 threads (was 27/40 aborts + 5 sibling reds), (3) the whole suite twice with 4 threads and
# once serially (420 s alarm each; UNFINISHED names on a hang or abort).
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
cd /Users/ueli/Documents/semio || exit 2
export RUST_MIN_STACK=67108864 CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-target
echo "BUILD $(date +%T)"
cargo test --offline --no-run --message-format short --lib -p semio-framework-os-renderer-wgpu 2>&1 | tee $L/ri-build.txt | /usr/bin/grep -E "^error|Executable" | head -20
B=$(/usr/bin/grep "Executable" $L/ri-build.txt | sed 's/.*(\(.*\))$/\1/' | head -1)
[ -x "$B" ] || { echo "NO-BINARY"; exit 3; }
echo "BINARY $B $(date +%T)"
S=async_boundary_tests::an_independent_decoder_job_recovers_its_response_from_an_exact_rejected_session
C=frame_job::tests::retained_clock_publication_preserves_the_live_frame_and_uses_its_completion_wake
V=frame_job::tests::stale_completed_frame_returns_its_exact_presented_input_candidate
for t in engine_canvas::engine_surface_attach_tests::engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack kernel_runtime::kernel_runtime_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack $S; do
  "$B" --exact "$t" --test-threads 1 2>&1 | /usr/bin/grep -E "^test |left:|right:" | cut -c1-300
done
echo "PAIR $("$B" --exact "$C" "$V" --test-threads 1 2>&1 | /usr/bin/grep -E '^test result' )"
typeset -A seen
for i in {1..40}; do
  out=$(perl -e 'alarm 60; exec @ARGV' "$B" async_boundary_tests:: --test-threads 4 2>&1); rc=$?
  key="rc=$rc"; [[ $rc == 0 ]] || key="$key $(print -r -- "$out" | /usr/bin/grep -E '^test .* FAILED$|fixed session admits|non-unwinding' | sort -u | tr '\n' ' ' | cut -c1-300)"
  seen[$key]=$(( ${seen[$key]:-0} + 1 ))
done
for k in ${(k)seen}; do echo "ASYNC-BOUNDARY-40 ${seen[$k]} x $k"; done
for run in t4a t4b t1; do
  threads=4; [ $run = t1 ] && threads=1
  raw=$L/ri-suite-$run.txt
  start=$(date +%s)
  perl -e 'alarm 420; exec @ARGV' "$B" --test-threads $threads > $raw 2>&1; rc=$?
  echo "SUITE $run rc=$rc secs=$(( $(date +%s) - start )) $(/usr/bin/grep '^test result' $raw | tr -d '\n')"
  /usr/bin/grep -E '^test .* FAILED$' $raw | sed 's/^/  RED /'
  if [ $rc != 0 ] && [ $rc != 101 ]; then
    "$B" --list --format terse 2>/dev/null | sed -n 's/: test$//p' | sort > $raw.all
    sed -nE 's/^test (.*) \.\.\. (ok|FAILED|ignored)$/\1/p' $raw | sort > $raw.done
    comm -23 $raw.all $raw.done | head -20 | sed 's/^/  UNFINISHED /'
    rm -f $raw.all $raw.done
  fi
done
echo "RI-DONE $(date +%T)"
