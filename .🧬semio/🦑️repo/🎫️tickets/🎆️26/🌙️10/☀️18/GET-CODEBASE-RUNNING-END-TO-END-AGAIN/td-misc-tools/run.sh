#!/bin/sh
# usage: run.sh <mirror> <mode: plain|art|a1|env> <list-file> <outdir>  -> per file: full log + one summary line
C="$(cd "$(dirname "$0")/.." && pwd)"; m="$1"; mode="$2"; lst="$3"; out="$4"; mkdir -p "$out"
R="${TMPDIR%/}/sm/$m"; i=0
while IFS= read -r f; do
  i=$((i+1)); log="$out/$mode-$i.log"
  case "$mode" in
    env) sh "$C/red-g-tools/mrun.sh" "$m" env "$f" --timeout 170000 > "$log" 2>&1;;
    art) A="$C/🗑️generated/td-misc/a$i"; mkdir -p "$A"; (cd "$R" && SEMIO_TEST_ARTIFACT_DIR="$A" nice -n 5 bun test "./$f" --timeout 170000) > "$log" 2>&1;;
    a1) A="$C/🗑️generated/td-misc/a$i"; mkdir -p "$A"; (cd "$R/$(dirname "$f")" && SEMIO_TEST_ARTIFACT_DIR="$A" nice -n 5 bun test "./$(basename "$f")" --timeout 170000) > "$log" 2>&1;;
    *) (cd "$R" && nice -n 5 bun test "./$f" --timeout 170000) > "$log" 2>&1;;
  esac
  printf '%s %s %s | %s\n' "$i" "$mode" "$(grep -E '^ *[0-9]+ (pass|fail)$' "$log" | tr -s ' \n' ' ')" "$(echo "$f" | awk -F/ '{print $(NF-2)"/"$(NF-1)}')"
done < "$lst"
