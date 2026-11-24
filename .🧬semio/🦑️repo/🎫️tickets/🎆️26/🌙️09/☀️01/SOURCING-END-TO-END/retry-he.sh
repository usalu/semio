#!/bin/zsh
T=$(dirname $0)
for i in 1 2 3 4 5 6; do
  $T/native-cargo.sh he9 "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation" test -p semio-s-artifact-sourcing-curation --lib history_edits_end_to_end -- --test-threads=1 --nocapture
  grep -q "^test result" $T/🗑️generated/he9.txt && break
  sleep 120
done
