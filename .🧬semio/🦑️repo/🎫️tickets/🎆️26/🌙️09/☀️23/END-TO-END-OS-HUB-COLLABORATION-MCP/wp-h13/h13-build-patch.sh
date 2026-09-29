#!/bin/zsh
# 🩹️ H13: rebuilds `<set>.patch` from a work set's base/edit copies (every file under edit/, new files diffed from /dev/null).
# usage: zsh h13-build-patch.sh <set-dir> <patch-name>
W=$1
cd "$W" || exit 1
: > "$2"
(cd edit && find . -type f | sed 's|^\./||' | sort) > files.txt
while IFS= read -r f; do
  if [ -f "base/$f" ]; then diff -u --label "a/$f" --label "b/$f" "base/$f" "edit/$f" >> "$2"; else diff -u --label "a/$f" --label "b/$f" /dev/null "edit/$f" >> "$2"; fi
done < files.txt
wc -l "$2"
