#!/usr/bin/env bash
set -euo pipefail

ticket=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$ticket/../../../../../../.." && pwd)
output="$ticket/🗑️generated"
label="${1:-live-5d-confirmation}"
puzzle='✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d'
editor="$puzzle/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
build_log="$output/$label-build.log"
export SEMIO_5D_NATIVE_JOBS="${SEMIO_5D_NATIVE_JOBS:-2}"

cd "$root"
bash "$ticket/native-verification.sh" --manifest-path "$puzzle/Cargo.toml" \
    -p semio-s-artifact-puzzle-5d -p semio-s-artifact-puzzle-5d-v1 \
    -p semio-s-artifact-puzzle-5d-v1-unit-tests \
    --config profile.test.package.semio-s-artifact-puzzle-5d-v1-unit-tests.strip=false \
    --lib --no-run >"$build_log" 2>&1

run_suite() (
    local crate="$1" suite="$2" directory="$3" binary
    shift 3
    binary=$(sed -n 's/^.*Executable unittests .* (\(\/tmp\/build\/.*\))$/\1/p' "$build_log" | rg -F "/$crate/" | tail -1)
    test -n "$binary"
    cd "$directory"
    local runtime=(/home/deck/.cache/semio/runtimes/bun/*/bun)
    export PATH="${runtime[0]%/bun}:$PATH" RUST_MIN_STACK=67108864
    "$output/native-target/${binary#/tmp/build/}" "$@" >"$output/$label-$suite.log" 2>&1
)

run_suite semio-s-artifact-puzzle-5d base "$puzzle/📦️packages/🦀️rust" --test-threads=4 &
base_pid=$!
run_suite semio-s-artifact-puzzle-5d-v1 v1 "$puzzle/🏅️standards/🔖️1/📦️packages/🦀️rust" --test-threads=4 &
version_pid=$!
run_suite semio-s-artifact-puzzle-5d-v1-unit-tests editor "$editor/🧪️tests/🔬️unit-2/📦️packages/🦀️rust" \
    nakagin_play_snapshot_declares_the_typed_composition_slots \
    set_active_example_reaches_the_capsule_dream_document \
    set_active_example_switches_the_document_and_never_faults_on_capacity \
    created_part_board_and_world_edits_reopen_with_history --test-threads=3 --nocapture &
editor_pid=$!

status=0
wait "$base_pid" || status=1
wait "$version_pid" || status=1
wait "$editor_pid" || status=1
exit "$status"
