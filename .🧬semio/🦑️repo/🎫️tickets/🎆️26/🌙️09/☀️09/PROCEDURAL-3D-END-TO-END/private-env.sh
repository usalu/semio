#!/bin/sh
# 🧊️ Runs a command inside the private root with the scrubbed environment (usage: private-env.sh <command...>).
P="${SINC_ROOT:-$HOME/.sinc/g3d}"; cd "$P" || exit 1
exec env -u REPO_ROOT -u SEMIO_MACHINE_STATE_DIRECTORY -u CARGO_TARGET_DIR -u CARGO_BUILD_BUILD_DIR -u NX_WORKSPACE_ROOT $(env | grep -E '^(SEMIO_CALLER|SEMIO_BUILD_SERVICE|NX_)' | cut -d= -f1 | sed 's/^/-u /') SEMIO_WORKSPACE_ROOT="$P" SEMIO_BUILD_SERVICE=off BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_REPO_SEAL=live SEMIO_LANGUAGE=en "$@"
