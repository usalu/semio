#!/bin/sh
# usage: mrun.sh <mirror> <test file relative to the library 🏗️build dir> <log> [name-pattern]  (obsrows2-g)
# plain pinned-bun single-file test inside a mirror, cwd = mirror root, no root bunfig preload (-c empty), Nx env pointing at mirror-local dirs; own pid in <log>.pid
cd /Users/ueli/Documents/semio || exit 1
M="$(cd "${SEMIO_MIRROR_HOME:-/private/tmp/sm}/$1" && pwd -P)"; B=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
L="$M/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; F="$L/🏗️build/$2"; [ -f "$F" ] || F="$L/$2"
FX="$M/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m"; mkdir -p "$FX/nx/artifacts" "$M/.cache/obr2/art" "$M/.cache/obr2/tmp"
: > "$M/.cache/obr2/empty.toml"
NXID=$(basename "$(readlink /Users/ueli/Documents/semio/.nx/installation)")
mkdir -p "$M/.cache/b/t/m" "$M/.nx"; ln -sfn /Users/ueli/Documents/semio/.cache/b/t/m/nx "$M/.cache/b/t/m/nx"; ln -sfn "/Users/ueli/Documents/semio/.cache/b/t/m/nx/$NXID" "$M/.nx/installation"
echo $$ > "$3.pid"
cd "$M" || exit 1
set -- "$F" "$3" "$4"
env SEMIO_NX_TOOLING_IDENTITY="$NXID" TMPDIR="$M/.cache/obr2/tmp" SEMIO_WORKSPACE_ROOT="$M" SEMIO_TEST_ARTIFACT_DIR="$M/.cache/obr2/art" SEMIO_NX_FIXED_STORAGE="$FX" NX_CACHE_DIRECTORY="$FX/nx/artifacts" ${EXTRA_ENV:-} \
  nice -n 10 "$B" test -c "$M/.cache/obr2/empty.toml" --timeout 120000 ${3:+-t "$3"} "$1" > "$2" 2>&1
