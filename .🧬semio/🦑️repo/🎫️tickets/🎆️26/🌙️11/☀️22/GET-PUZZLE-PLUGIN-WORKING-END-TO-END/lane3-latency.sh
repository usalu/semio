#!/usr/bin/env bash
set -u
LOG="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END/🗑️generated/lane3-latency.txt"
cargo test --jobs 2 --offline --color never -p semio-s-artifact-puzzle-3d-editor --test editor-gestures one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns -- --nocapture --test-threads=1 > /tmp/lane3-lat.txt 2>&1
rc=$?
{
  echo "RC=$rc"
  grep -E '^error|could not compile|\[DEBUG\] pump-stop|test result:|panicked at|assertion|FAILED' /tmp/lane3-lat.txt | head -n 400
  echo "---- tail ----"
  tail -n 30 /tmp/lane3-lat.txt
} > "$LOG"
find /tmp/build -type f -perm -u+x ! -name "*.so" ! -name "*.rlib" ! -name "*.rmeta" ! -name "*build*script*" -delete 2>/dev/null || true
exit "$rc"
