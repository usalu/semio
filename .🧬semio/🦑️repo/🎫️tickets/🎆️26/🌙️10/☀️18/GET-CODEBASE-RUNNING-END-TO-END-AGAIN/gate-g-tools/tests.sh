#!/bin/sh
# 🧪 Runs a list of test files one at a time: `tests.sh <list> <out-dir>`; prints "exit pass fail file" per file.
cd "$(dirname "$0")/../../../../../../../.." || exit 1
mkdir -p "$2"; n=0
while IFS= read -r f; do
  [ -z "$f" ] && continue
  n=$((n+1))
  nice -n 10 bun test "./$f" > "$2/$n.txt" 2>&1
  echo "$? $(grep -E '^ *[0-9]+ (pass|fail)' "$2/$n.txt" | tr '\n' ' ') $f"
done < "$1"
