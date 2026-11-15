#!/bin/bash
# usage: mirror.sh <ro-root> <dest>; builds <dest> as a writable mirror of <ro-root> where the shared-worktree module is a real copy and every other entry on its ancestor path is a symlink into <ro-root>
set -e
R="$1"; D="$2"
M=$(cd "$R" && ls -d 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree)
mkdir -p "$D"
cur=""
IFS=/ read -ra parts <<< "$M"
for p in "${parts[@]}"; do
  mkdir -p "$D/$cur"
  for e in "$R/$cur"*; do n=$(basename "$e"); [ "$n" = "$p" ] && continue; [ -e "$D/$cur$n" ] || ln -s "$e" "$D/$cur$n"; done
  cur="$cur$p/"
done
rm -rf "$D/$M"; cp -a "$R/$M" "$D/$M"
echo "$D/$M"
