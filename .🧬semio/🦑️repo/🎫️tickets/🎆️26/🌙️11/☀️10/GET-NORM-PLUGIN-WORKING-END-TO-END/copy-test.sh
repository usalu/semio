#!/usr/bin/env bash
# 🧪️ Runs native cargo in the private copy under a machine capacity grant. Usage: copy-test.sh <log-name> <member directory relative to the repository root> <cargo args...>
set -u
T="$(cd "$(dirname "$0")" && pwd)"; ROOT="$(cd "$T/../../../../../../.." && pwd)"
COPY="${NORM_COPY:?}"; log="$T/🗑️generated/$1.log"; dir="$2"; shift 2
export CARGO_HOME="$ROOT/.cache/b/r/m/cargo/home" CARGO_TARGET_DIR="$COPY/target"
export PATH="$(ls -d "$ROOT"/.cache/b/t/m/r/*/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin | head -1):$PATH"
cd "$ROOT" || exit 1
run() {
  repo capacity run --memory="$1" --cpu=3 --lang=en -- env SEMIO_PYTHON="$(ls -d "$ROOT"/.cache/b/d/m/python/environments/workspace-*/bin/python | head -1)" CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_HOME="$CARGO_HOME" CARGO_TARGET_DIR="$CARGO_TARGET_DIR" PATH="$PATH" sh -c 'cd "$0" && exec cargo "$@"' "$COPY/src/$dir" "${@:2}" >> "$log" 2>&1
}
: > "$log"; run 3000 "$@"; code=$?
for retry in 1 2; do
  [ "$code" -ne 0 ] && grep -aq "signal: 9" "$log" || break
  echo "[copy-test] compiler was killed, retrying with one job" >> "$log"; sed -i '' 's/(signal: 9[^)]*)/(killed compiler, retried)/' "$log"
  args=(); for a in "$@"; do [ "$a" = 3 ] || [ "$a" = 2 ] && a=1; args+=("$a"); done
  run 4000 "${args[@]}"; code=$?
done
echo "exit $code" >> "$log"; exit $code
