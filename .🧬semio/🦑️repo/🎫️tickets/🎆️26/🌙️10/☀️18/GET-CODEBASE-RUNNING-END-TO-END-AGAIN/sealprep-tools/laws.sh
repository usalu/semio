#!/bin/sh
# 🧾️ [sealprep] runs every law file of laws.txt (or LIST) once in a mirror, plain first and through the graph-free official envelope when the plain run asks for an invocation; one summary line + failing test names per file into C/🗑️generated/sealprep/<tag>.txt. usage: sh laws.sh <mirror-name> <tag> [LIST]
N="$1"; TAG="$2"; C="$(cd "$(dirname "$0")/.." && pwd)"; L="${3:-$C/sealprep-tools/laws.txt}"; O="$C/🗑️generated/sealprep/$TAG.txt"; mkdir -p "$(dirname "$O")"; : > "$O"
while IFS= read -r f; do
 [ -n "$f" ] || continue
 out=$(nice -n 5 sh "$C/red-g-tools/mrun.sh" "$N" plain "$f" --timeout 170000 2>&1)
 case "$out" in *"invocation required"*|*"Original parent"*) out=$(nice -n 5 sh "$C/red-g-tools/mrun.sh" "$N" env "$f" --timeout 170000 2>&1);; esac
 { echo "## $f"; echo "$out"; } >> "$O"
 echo "$(echo "$out" | head -1 | cut -c1-120)  $(echo "$f" | awk -F/ '{print $(NF-2)"/"$(NF-1)}')"
done < "$L"
