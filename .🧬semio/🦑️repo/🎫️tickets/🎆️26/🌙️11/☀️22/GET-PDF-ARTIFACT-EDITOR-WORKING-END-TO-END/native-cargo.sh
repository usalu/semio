#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
ticket="$root/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END"
TC=$(ls -d "$root"/.cache/b/t/m/r/*/toolchains/nightly-2026-07-20-*/bin | head -1)
target="$ticket/🗑️generated/native-target"
lanes=${NATIVE_CARGO_LANES:-3}
mkdir -p "$target/locks"
lane=""
while [ -z "$lane" ]; do
  i=0
  while [ "$i" -lt "$lanes" ]; do
    lock="$target/locks/lane-$i"
    if mkdir "$lock" 2>/dev/null; then
      echo "$$" > "$lock/pid"
      lane=$i
      break
    fi
    holder=$(cat "$lock/pid" 2>/dev/null || echo "")
    if [ -n "$holder" ] && ! kill -0 "$holder" 2>/dev/null; then
      rm -rf "$lock"
      continue
    fi
    i=$((i + 1))
  done
  [ -n "$lane" ] || sleep 5
done
trap 'rm -rf "$target/locks/lane-$lane"' EXIT INT TERM
echo "native-cargo: lane $lane" >&2
cd "${NATIVE_CARGO_DIR:-$root/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf}"
status=0
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 PATH="$TC:$PATH" "$TC/cargo" --config 'build.rustc-wrapper=""' --config "build.rustc=\"$TC/rustc\"" --config "build.target-dir=\"$target/lane-$lane/t\"" --config "build.build-dir=\"$target/lane-$lane/b\"" "$@" || status=$?
exit "$status"
