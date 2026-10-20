#!/bin/sh
# UNRUN (Docker was down when this set was staged). Runs the Linux-gated suites of the xplat-linux-fix set in an UNPRIVILEGED arm64 Linux container on a throw-away copy.
# usage: sh run-linux-suites.sh            (needs `docker info` to work; never starts Docker Desktop; nice, one container, <= 4 GB copy, volume removed at the end)
# Evidence expected from a green run: every file below prints "0 fail"; the `skipIf(platform !== linux)` rows (real /proc, real ps, init start tick, real Bazel absent) run for the first time.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
docker info >/dev/null 2>&1 || { echo "docker is not running: static verification only"; exit 3; }
IMAGE=${IMAGE:-oven/bun:1.4.2-debian}
OV=/private/tmp/xplat-linux-fix/linux-real
VOL=xplat-linux-fix-$$
python3 "$HERE/mkreal.py" "$OV"
python3 "$HERE/apply.py" --root "$OV" --write --backup /private/tmp/xplat-linux-fix/bk-linux
cp "$HERE/envwire.ts" "$OV/envwire.ts"
docker volume create "$VOL" >/dev/null
trap 'docker volume rm -f "$VOL" >/dev/null 2>&1 || true' EXIT
tar -C "$OV" -cf - . | docker run --rm -i -v "$VOL":/w --user 0:0 "$IMAGE" sh -c 'tar -C /w -xf - && chown -R 1000:1000 /w'
F="🧰️framework"; P="$F/🔨️modules/🏃️process"; NB="$P/📦️artifacts/🏗️native-build/💾️storage"; H="$P/🏗️build/🌡️host-capacity"; U="$F/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache"
FILES="$NB/📁️directory/🧪️tests/🟦️.ts $NB/📁️directory/📏️usage/🧪️tests/🟦️.ts $NB/🌐️platform/📋️contract/📐️statfs/🧪️tests/🟦️.ts $NB/🌐️platform/🧪️tests/🧭️dispatch/🟦️.ts $NB/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🧪️tests/🟦️.ts $NB/🧱️fixed/🧪️tests/🎚️capacity/🟦️.ts $NB/🧱️fixed/🧪️tests/🟦️.ts $NB/🧱️fixed/🧪️tests/🌱️birth/🟦️.ts $NB/📤️publication/🧱️fixed/🧪️tests/🟦️.ts $H/🧪️tests/📡️sampling/🟦️.ts $H/🧪️tests/📡️sampling/🐧️linux/🟦️.ts $H/🧪️tests/🚪️gate/🟦️.ts $U/🧵️lane/🧪️tests/🟦️.ts"
docker run --rm --user 1000:1000 --cap-drop ALL --security-opt no-new-privileges --cpus 4 --memory 6g -v "$VOL":/w -w /w -e FILES="$FILES" "$IMAGE" sh -c '
  export NO_COLOR=1 SEMIO_BUILD_MODE=prototype SEMIO_REPO_ROOT=/w
  WIRE=$(bun envwire.ts); export SEMIO_SCRIPT_PROCESS_INVOCATION="$WIRE"
  mkdir -p /tmp/art; export SEMIO_TEST_ARTIFACT_DIR=/tmp/art
  echo "$FILES" | tr " " "\n" | while read f; do [ -f "$f" ] || { echo "MISSING $f"; continue; }; printf "%s :: " "$f"; nice -n 10 bun test --timeout 120000 "./$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail|skip)|^\(fail\)" | tr "\n" " "; echo; done'
