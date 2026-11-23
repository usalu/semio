#!/usr/bin/env bash
D="$(dirname "$0")"
export RUST_BACKTRACE=1
EXTRA="--skip history_edits_end_to_end --skip two_actors_reconnect --skip two_instances_converge --skip a_gumball_move_edited_in_history --skip frame_cost_stays_flat --skip playback_presses_are_one_config --skip fem2d_window_config_runtime_isolates" "$D/b2d-run.sh" t3 test -p semio-s-artifact-fem-2d-editor -p semio-s-artifact-fem-2d-editor-interaction --no-fail-fast --lib --test editor_app --test editor_app_2
