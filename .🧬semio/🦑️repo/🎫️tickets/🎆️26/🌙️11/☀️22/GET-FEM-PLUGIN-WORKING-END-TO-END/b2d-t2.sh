#!/usr/bin/env bash
D="$(dirname "$0")"
export RUST_BACKTRACE=1
EXTRA="remove_selection set_result_animation window_config_ownership result_animation_tick" "$D/b2d-run.sh" t2a test -p semio-s-artifact-fem-2d-editor --test editor_app_2 --no-fail-fast
EXTRA="every_command_matches mounted_camera a_multi_selection" "$D/b2d-run.sh" t2b test -p semio-s-artifact-fem-2d-editor --test editor_app --no-fail-fast
EXTRA="" "$D/b2d-run.sh" t2c test -p semio-s-artifact-fem-2d-editor --lib --no-fail-fast
EXTRA="canvas_pointer" "$D/b2d-run.sh" t2d test -p semio-s-artifact-fem-2d-editor --lib --no-fail-fast
