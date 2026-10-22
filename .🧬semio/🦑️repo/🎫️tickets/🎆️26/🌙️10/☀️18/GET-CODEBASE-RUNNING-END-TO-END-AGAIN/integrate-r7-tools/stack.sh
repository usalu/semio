#!/bin/sh
# usage: stack.sh <root> [--write-live]   applies the round-7 sets in dependency order to <root> (an overlay or the live checkout).
# Every step re-derives from the text of <root>; stops at the first failure.
set -eu
ROOT=$(cd "$1" && pwd)
here=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$here/.." && pwd)
LIB="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
step() { printf '== %s\n' "$1"; }
step bzl-paths
SEMIO_ROOT=$ROOT nice -n 10 python3 "$C/bzl-hermetic-tools/staged-paths/apply.py" --rebase --write | tail -1
step lawcov-build
for p in apply apply-fixes apply-tests; do LAWCOV_LIBRARY="$LIB" nice -n 10 python3 "$C/lawcov-build-tools/staged/$p.py" --apply | tail -1; done
step boot-fast
nice -n 10 python3 "$C/boot-fast-tools/staged/apply.py" --root "$ROOT" --write | tail -1
step i18n-core
(cd "$C/i18n-core-tools" && nice -n 10 python3 build.py --root "$ROOT" | tail -1 && nice -n 10 python3 apply.py --root "$ROOT" --yes | tail -1 | cut -c1-120)
step tool-direct-code
(cd "$C/tool-direct-code-tools/build" && nice -n 10 python3 build.py --base "$ROOT" 2>&1 | grep -v '^NOTE' | tail -3 | cut -c1-300)
(cd "$C/tool-direct-code-tools" && nice -n 10 python3 apply.py apply --write --root "$ROOT" | tail -1)
step tool-direct-manifests
(cd "$C/tool-direct-manifests-tools" && sh regen.sh "$ROOT" > "${TMPDIR:-/tmp}/tdm-regen.txt" 2>&1; tail -2 "${TMPDIR:-/tmp}/tdm-regen.txt"; nice -n 10 python3 -I apply.py apply --write --allow-frozen --root "$ROOT" | tail -1 | cut -c1-200)
