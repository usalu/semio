#!/bin/zsh
T=$(dirname $0)
for i in 1 2 3 4; do
  $T/native-cargo.sh full4 "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation" test -p semio-s-artifact-sourcing-curation --lib -- --test-threads=1
  grep -q "^test result" $T/🗑️generated/full4.txt && break
  sleep 120
done
