#!/bin/sh
# 🚦️ Runs one cargo invocation of the sourcing ticket through the repository's admitted entry `repo cargo`: the machine-wide capacity gate grants the jobs (waiting with progress while memory is short, ending on cancellation), the unit-cache rustc wrapper stays active and the shared store carries the target; nothing here names a job count, a target directory or a compiler.
# Usage: [OUT=<dir>] native-cargo.sh <label> <workspace-dir-relative-to-root> <cargo args...>   (prints `exit <code> -> <file>`; the log is <OUT>/<label>.txt, OUT defaults to 🗑️generated)
# @see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🚦️native-cargo-grant/🟦️.ts
# @see ../../../🌙️11/☀️24/WARDEN-COVERAGE-FOR-EVERY-REPO-TOOLCHAIN-PROCESS-AND-DISK-BOUNDED-SWAP/🚦️native-cargo-admission.md
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
out=${OUT:-$here/🗑️generated}
label=$1; workspace=$2; shift 2
mkdir -p "$out/artifacts"
SEMIO_TEST_ARTIFACT_DIR="$out/artifacts"; export SEMIO_TEST_ARTIFACT_DIR
cd "$root/$workspace"
status=0
repo cargo --store=shared --configuration="${CONFIGURATION:-prototype}" -- "$@" > "$out/$label.txt" 2>&1 || status=$?
echo "exit $status -> $out/$label.txt"
exit "$status"
