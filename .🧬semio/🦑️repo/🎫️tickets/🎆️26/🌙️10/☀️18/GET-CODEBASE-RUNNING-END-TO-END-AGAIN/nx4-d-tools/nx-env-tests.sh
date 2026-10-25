#!/bin/sh
# Runs each named test file in its own `bun test` inside the Nx task env of `repo --tool=nx exec` (cwd = repo root).
ROOT=/Users/ueli/Documents/semio
cd "$ROOT" || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/nx4-d"
mkdir -p "$G/artifacts"
export SEMIO_TEST_ARTIFACT_DIR="$ROOT/$G/artifacts"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
for f in "$@"; do
  name=$(printf '%s' "$f" | md5 -q 2>/dev/null || printf '%s' "$f" | md5sum | cut -c1-32)
  echo "=== $f"
  bun test "./$f" > "$G/nxenv-$name.log" 2>&1
  echo "rc=$? log=nxenv-$name.log"
  rg "^\(fail\)|^\(pass\)| pass$| fail$|^ [0-9]+ (pass|fail)" "$G/nxenv-$name.log" | head -40
done
