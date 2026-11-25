#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
ticket="$root/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END"
TC=$(ls -d "$root"/.cache/b/t/m/r/*/toolchains/nightly-2026-07-20-*/bin 2>/dev/null | head -1)
[ -n "$TC" ] || TC=$HOME/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin
target="$ticket/🗑️generated/native-target"
lane=${NATIVE_CARGO_LANE:-0}
jobs=3
floor=45
gate="$target/locks/gate"
free_percent() {
  memory_pressure 2>/dev/null | awk -F': ' '/free percentage/ { gsub("%", "", $2); print $2 }' || true
}
queue="$target/locks/queue"
mkdir -p "$queue"
ticket_number=$(( $(date +%s) * 100000 + $$ % 100000 ))
ticket_file="$queue/$ticket_number.$$"
: > "$ticket_file"
trap 'rm -f "$ticket_file"' EXIT INT TERM
first_in_queue() {
  for waiting in $(ls "$queue" 2>/dev/null | sort -n); do
    if kill -0 "${waiting##*.}" 2>/dev/null; then [ "$waiting" = "${ticket_file##*/}" ]; return; fi
    rm -f "$queue/$waiting"
  done
  return 1
}
held=""
while [ -z "$held" ]; do
  if ! first_in_queue; then
    sleep 5
    continue
  fi
  holder=$(cat "$gate/pid" 2>/dev/null || echo "")
  if [ -n "$holder" ] && ! kill -0 "$holder" 2>/dev/null; then
    rm -rf "$gate"
  fi
  if pgrep -x rustc >/dev/null 2>&1 && [ ! -d "$gate" ]; then
    sleep 10
    continue
  fi
  free=$(free_percent)
  if [ -n "$free" ] && [ "$free" -lt "$floor" ]; then
    echo "native-cargo: waiting for memory ($free% free, floor $floor%)" >&2
    sleep 15
    continue
  fi
  if mkdir "$gate" 2>/dev/null; then
    echo "$$" > "$gate/pid"
    held=1
  else
    sleep 5
  fi
done
rm -f "$ticket_file"
trap 'rm -rf "$gate"' EXIT INT TERM
echo "native-cargo: lane $lane, $jobs jobs, exclusive" >&2
cd "${NATIVE_CARGO_DIR:-$root/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf}"
status=0
CARGO_BUILD_JOBS="$jobs" RUST_TEST_THREADS="$jobs" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 PATH="$TC:$PATH" "$TC/cargo" --config 'build.rustc-wrapper=""' --config 'build.rustflags=["-Z","threads=1"]' --config "build.rustc=\"$TC/rustc\"" --config "build.target-dir=\"$target/lane-$lane/t\"" --config "build.build-dir=\"$target/lane-$lane/b\"" "$@" || status=$?
exit "$status"
