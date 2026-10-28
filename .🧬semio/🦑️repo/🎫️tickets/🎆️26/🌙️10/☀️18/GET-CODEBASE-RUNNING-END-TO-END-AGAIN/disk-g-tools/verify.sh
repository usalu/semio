#!/bin/sh
# Mirror verification of one staged batch, graph-free, live tree untouched (mirror seal + guard).
# sh verify.sh <batch> [before]   after: apply the batch on a fresh mirror and run tests-<batch>.list; before: only the NEW test files on an unedited mirror (they must fail).
T="$(cd "$(dirname "$0")" && pwd)"; C="$(cd "$T/.." && pwd)"; B="$1"; MODE="${2:-after}"
[ -f "$T/batches/$B.py" ] || { echo "unknown batch: $B"; exit 2; }
G="$C/🗑️generated/disk-g"; mkdir -p "$G"
LIVE="$(cd "$C/../../../../../../.." && pwd)"
( cd "$LIVE" && git -c core.quotepath=off grep -lFz '⚡️cache' -- . ':(exclude).🧬semio' | tr '\0' '\n' > "$G/law-files-repo.txt" && git -c core.quotepath=off grep -lFz '.cache' -- . ':(exclude).🧬semio' | tr '\0' '\n' > "$G/law-files-native.txt" )
export SEMIO_CACHE_LAW_FILES="$G/law-files-repo.txt" SEMIO_CACHE_NATIVE_LAW_FILES="$G/law-files-native.txt"
if [ "$MODE" = before ]; then NAME="dg-b$(printf %s "$B" | cut -c1-4)"; OP="newonly"; else NAME="dg-$(printf %s "$B" | cut -c1-5)"; OP="apply"; fi
python3 -I "$C/mirror-tools/mirror.py" drop "$NAME" >/dev/null 2>&1
python3 -I "$C/mirror-tools/mirror.py" create "$NAME" --batch "python3 -I '$T/stage.py' $OP $B --root {root} --backup-dir {root}/.tmp/bk --tolerate-missing" || exit 1
if [ "$B" = procstore ]; then ( cd "${TMPDIR:-/tmp}/sm/$NAME" && git init -q . && git add -A . >/dev/null 2>&1 && git -c user.email=a@b -c user.name=n commit -qm base >/dev/null 2>&1 ); fi
sh "$T/runtests.sh" "$NAME" "$T/tests-$B.list"
python3 -I "$C/mirror-tools/mirror.py" drop "$NAME" >/dev/null 2>&1
