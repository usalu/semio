#!/bin/sh
# usage: watch-media-in-law.sh  (reruns the interactive-grant MediaIn law every 30 minutes until it passes, up to 8 tries; log 🗑️generated/media-in-hang/watch.log)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
L="$here/🗑️generated/media-in-hang"
mkdir -p "$L"
for try in 1 2 3 4 5 6 7 8; do
  sh "$here/native-cargo.sh" test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib reaches_its_mount_through_the_real_runtime_under_the_interactive_grant > "$L/watch-$try.log" 2>&1
  if grep -aq "test result: ok. 1 passed" "$L/watch-$try.log"; then echo "$(date +%T) try $try GREEN" >> "$L/watch.log"; exit 0; fi
  echo "$(date +%T) try $try red: $(grep -a -o 'MediaIn neither completed[^:]*\|error\[E[0-9]*\][^\n]*\|could not compile[^\n]*' "$L/watch-$try.log" | head -1)" >> "$L/watch.log"
  sleep 1800
done
exit 1
