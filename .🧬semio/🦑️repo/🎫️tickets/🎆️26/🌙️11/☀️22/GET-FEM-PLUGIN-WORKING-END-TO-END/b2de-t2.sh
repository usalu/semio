#!/usr/bin/env bash
D="$(dirname "$0")"
export RUST_BACKTRACE=1
EXTRA="admitted_document_mutations remove_selection_covers_nodes remove_selection_mounted every_command_matches --test-threads 1" "$D/b2de-run.sh" e2 test -p semio-s-artifact-fem-2d-editor --no-fail-fast --lib --test editor_app --test editor_app_2
