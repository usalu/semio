#!/bin/sh
T=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$T/.." && pwd)
MODE=${1:?orig|split}
shift
OV=/private/tmp/claude-501/fixups6-ov-$MODE
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph"
TARGET=${1:-$G/🟦️.ts}
TAG=$(printf '%s' "$TARGET" | cksum | cut -d' ' -f1)
mkdir -p "$C/🗑️generated/fixups6/art-$MODE-$TAG"
cd "$OV" || exit 2
SEMIO_WORKSPACE_ROOT="$OV" SEMIO_TEST_ARTIFACT_DIR="$C/🗑️generated/fixups6/art-$MODE-$TAG" perl -e 'alarm 580; exec @ARGV' -- bun "$C/bzl-act-tools/wrap.ts" "./$TARGET" --reporter=junit --reporter-outfile="$C/🗑️generated/fixups6/junit-$MODE-$TAG.xml" > "$C/🗑️generated/fixups6/run-$MODE-$TAG.log" 2>&1
RC=$?
"$T/detach.sh" "$OV"
echo "rc=$RC log=run-$MODE-$TAG.log"
