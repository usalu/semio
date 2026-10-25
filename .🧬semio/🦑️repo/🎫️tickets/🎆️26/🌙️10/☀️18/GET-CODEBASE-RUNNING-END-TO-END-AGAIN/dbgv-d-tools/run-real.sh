#!/bin/sh
# usage: run-real.sh <step>... ; each step runs the named real test targets through `repo run-many` and logs under 🗑️generated/dbgv-d/real/
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/dbgv-d/real"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS
art="/Users/ueli/Documents/semio/$C/🗑️generated/dbgv-d/a"; mkdir -p "$art"; export SEMIO_TEST_ARTIFACT_DIR="$art"
run() { name="$1"; proj="$2"; targets="$3"
  nice -n 10 bun repo --tool=semio run-many --projects="$proj" --targets="$targets" --exclude-dependencies --parallel=1 > "$out/$name.log" 2>&1
  echo $? > "$out/$name.rc"; }
for step in "$@"; do
  case "$step" in
    arguments) run arguments @semio-tech/repo-lib test-nx-owner-arguments,probe-nx-owner-arguments ;;
    process) run process @semio-tech/framework-process test-native-image-observation,test-resource-lease-waits,test-test-command,test-workspace,test-command-owner,test-exact-cargo-storage,test-cargo-driver,test-native-cache-descriptors,test-windows-conditional-retirement,test-native-storage-platform,test-owned-execution,test-capture,test-resource-leases,test-descriptor-lifetime ;;
    caching) run caching repo test-selected-graph,test-cache-reset,test-rustup-protocol,test-rust-units,test-pruning,test-authorizers,test-cargo-launcher ;;
    build) run build workspace test-workspace-build ;;
    cargo) run cargo @semio-tech/cargo-workspaces contract-check,original-control-contract-check ;;
    probe) nice -n 10 bun test "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/🟦️.ts" --test-name-pattern "canonical build dispatcher|retains both original Nextest" > "$out/probe.log" 2>&1; echo $? > "$out/probe.rc" ;;
    small) run small repo-task-closure,repo-configured-source,@semio-tech/repo-developer-tools test ;;
  esac
done
