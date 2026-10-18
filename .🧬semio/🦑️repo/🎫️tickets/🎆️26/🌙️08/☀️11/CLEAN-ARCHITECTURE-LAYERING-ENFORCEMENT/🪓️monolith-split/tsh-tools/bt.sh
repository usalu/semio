#!/bin/zsh
# usage: bt.sh <phase> <file-rel-to-repo>...  runs `bun test <file>` each (retry once on segfault), logs to $G/tsh-bt-<phase>.log, appends result line to $G/tsh-bt-<phase>.tsv
. "$(dirname "$0")/env.sh"
phase="$1"; shift
cd "$R" || exit 9
export SEMIO_TEST_ARTIFACT_DIR="/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/tsh-art"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
for f in "$@"; do
  out="$G/tsh-bt-$phase-$(echo "$f" | md5 -q).log"
  for attempt in 1 2; do
    perl -e 'alarm 420; exec @ARGV' -- nice -n 10 bun test --timeout 120000 "./$f" > "$out" 2>&1; ec=$?
    if /usr/bin/grep -q -E 'Segmentation fault|panic\(|oh no: Bun has crashed' "$out"; then seg=1; else seg=0; break; fi
  done
  pass=$(/usr/bin/grep -E '^ *[0-9]+ pass' "$out" | tail -1 | awk '{print $1}')
  fail=$(/usr/bin/grep -E '^ *[0-9]+ fail' "$out" | tail -1 | awk '{print $1}')
  printf '%s\texit=%s\tpass=%s\tfail=%s\tseg=%s\t%s\n' "$f" "$ec" "${pass:-?}" "${fail:-?}" "$seg" "$out" >> "$G/tsh-bt-$phase.tsv"
done
