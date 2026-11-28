#!/bin/sh
# usage: finish-when-green.sh  (every 40 min reruns the interactive-grant MediaIn law with --nocapture; on the first green run rebuilds and stages the wasm, runs the full journey and the third-party oracle; markers in 🗑️generated/finish.log)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
G="$here/🗑️generated"; L="$G/media-in-hang"; mkdir -p "$L"
for try in $(seq 1 18); do
  sh "$here/native-cargo.sh" test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib reaches_its_mount_through_the_real_runtime_under_the_interactive_grant -- --nocapture > "$L/fin-$try.log" 2>&1
  if grep -aq "neither completed\|did not reach\|MediaIn faulted\|could not compile\|^error\[" "$L/fin-$try.log"; then echo "$(date +%T) law red ($try)" >> "$G/finish.log"; sleep 2400; continue; fi
  echo "$(date +%T) law green ($try)" >> "$G/finish.log"
  sh "$here/round6-chain.sh" r9x >> "$G/finish.log" 2>&1
  rm -rf "$G/pdf-journey-r9" "$G/pdf-journey-r9.done"; sh "$here/run-round4-journey.sh" pdf-journey-r9
  (cd "$here" && uv run --offline --with pymupdf==1.26.7 --with pypdf python -I journey-oracle.py all "$G/pdf-journey-r9") > "$G/pdf-journey-r9.oracle.log" 2>&1
  echo "$(date +%T) journey done $(cat "$G/pdf-journey-r9.done")" >> "$G/finish.log"; exit 0
done
echo "$(date +%T) gave up" >> "$G/finish.log"; exit 1
