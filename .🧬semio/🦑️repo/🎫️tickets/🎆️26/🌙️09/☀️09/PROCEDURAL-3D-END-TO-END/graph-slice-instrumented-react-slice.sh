#!/bin/sh
# Instrumented real read of the heavy React slice (plan only, nothing is built).
T="$(dirname "$0")/🗑️generated"; cd "$(git rev-parse --show-toplevel)" || exit 1
date +%s > "$T/react-start.txt"
SEMIO_LANGUAGE=en SEMIO_GRAPH_PHASES=1 SEMIO_BUILD_PROFILE_INPUTS=1 repo run @semio-tech/framework-os-dev:dev-generation3d-react-prototype --dry-run --output=stream > "$T/react-run.txt" 2>&1
date +%s > "$T/react-end.txt"
