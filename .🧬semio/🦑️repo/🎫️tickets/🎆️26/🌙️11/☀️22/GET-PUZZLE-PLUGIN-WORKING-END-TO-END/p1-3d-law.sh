#!/usr/bin/env bash
set -u
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END/🗑️generated"
cargo test --jobs 1 --offline --color never -p semio-s-artifact-puzzle-3d-editor --test editor-gestures one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns -- --nocapture --test-threads=1 > "$G/p1-law.log" 2>&1
echo "RC=$?" >> "$G/p1-law.log"
