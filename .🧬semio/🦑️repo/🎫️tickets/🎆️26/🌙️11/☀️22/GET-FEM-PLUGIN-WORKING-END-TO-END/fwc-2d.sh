#!/usr/bin/env bash
D="$(dirname "$0")"
EXTRA="window_config_ownership playback_presses" "$D/fwc-2d-run.sh" fwc-$1 test -p semio-s-artifact-fem-2d-editor --test editor_app_2 --no-fail-fast
