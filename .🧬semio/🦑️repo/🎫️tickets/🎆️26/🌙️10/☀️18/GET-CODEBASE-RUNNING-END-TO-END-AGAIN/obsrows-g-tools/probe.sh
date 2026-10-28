#!/bin/sh
# usage: ffi-probe.sh <listfile> <outfile> [birth] -> per file: exit status, pass/fail counts, native-capability errors (genuine birth, default setup-sdk); resumable
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
B=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
birth="${3:-setup-sdk}"
mkdir -p /Users/ueli/Documents/semio/temp/ng-birth/tmp "$C/🗑️generated/nodeffi-g/ffi"
unset SEMIO_SCRIPT_STALL_MS
while read -r f; do
  [ -z "$f" ] && continue
  /usr/bin/grep -qF "$f	" "$2" 2>/dev/null && continue
  log="$C/🗑️generated/nodeffi-g/ffi/$(printf '%s' "$f" | md5 -q)-$birth.log"
  TMPDIR=/Users/ueli/Documents/semio/temp/ng-birth/tmp SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio nice -n 10 "$B" "$G" "--file=$M/$f" "--birth=$birth" > "$log" 2>&1 &
  pid=$!; n=0
  while kill -0 $pid 2>/dev/null && [ $n -lt ${FFI_PROBE_SECONDS:-600} ]; do sleep 1; n=$((n+1)); done
  if kill -0 $pid 2>/dev/null; then kill $pid 2>/dev/null; sleep 1; kill -9 $pid 2>/dev/null; echo "timeout" >> "$log"; wait $pid 2>/dev/null; status=timeout; else wait $pid; status=$?; fi
  pass=$(/usr/bin/grep -c "^(pass)" "$log"); fail=$(/usr/bin/grep -c "^(fail)" "$log"); native=$(/usr/bin/grep -c "RuntimeCapabilityMissing\|NATIVE_PROVIDER" "$log")
  printf '%s\t%s\tpass=%s\tfail=%s\tnative=%s\n' "$f" "$status" "$pass" "$fail" "$native" >> "$2"
done < "$1"
