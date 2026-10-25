#!/bin/sh
# runs inside one `repo exec` envelope: usage loop.sh <outdir> <listfile> [<tag>]; one `repo tools exec bun -- test <file>` per line
cd /Users/ueli/Documents/semio || exit 1
out="$1"; list="$2"; n=0
mkdir -p "$out"; : > "$out/summary.txt"
while IFS= read -r f; do
  [ -z "$f" ] && continue
  n=$((n+1)); mkdir -p "$out/art$n"
  SEMIO_TEST_ARTIFACT_DIR="${ART:-$out/art$n}" repo tools exec bun -- test --timeout 600000 "/Users/ueli/Documents/semio/$f" > "$out/f$n.log" 2>&1
  echo "$n rc=$? $(/usr/bin/grep -E '^ *[0-9]+ (pass|fail)$' "$out/f$n.log" | tr -s ' ' | tr '\n' ' ') $f" >> "$out/summary.txt"
done < "$list"
echo done >> "$out/summary.txt"
