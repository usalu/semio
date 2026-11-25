#!/bin/zsh
# Runs checkout test files (paths under the build library) one at a time against the staged files; prints one summary line each and keeps the tail of a failure in 🗑️generated/<name>.log.
here=${0:A:h}; cd /Users/ueli/Documents/semio; mkdir -p "$here/../../🗑️generated/W14"
for rel in "$@"; do
  name=$(print -r -- "$rel" | shasum -a 1 | cut -c1-10)
  "$here/mk.sh" "$rel" "$name"
  out=$(repo test-file "${here#/Users/ueli/Documents/semio/}/run/$name.test.ts" 2>&1)
  print -r -- "$out" > "$here/../../🗑️generated/W14/$name.log"
  print -r -- "$(print -r -- "$out" | grep -E '^(passed|FAILED)' | sed "s|.*run/$name.test.ts|$rel|")"
done
