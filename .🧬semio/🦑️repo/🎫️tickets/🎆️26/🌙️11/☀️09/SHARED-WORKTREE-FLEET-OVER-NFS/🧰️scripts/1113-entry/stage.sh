#!/bin/bash
# builds the stage for linux-suite.sh: the module copy of the working copy, every other ancestor entry a symlink to /w (the live tree, mounted read only in the container)
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/stage
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS"
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
rm -rf "$S"; mkdir -p "$S"
cur=""
IFS=/ read -ra parts <<< "$M"
for p in "${parts[@]}"; do
  mkdir -p "$S/$cur"
  for e in /Users/Shared/semio/$cur*; do n=$(basename "$e"); [ "$n" = "$p" ] && continue; ln -s "/w/$cur$n" "$S/$cur$n"; done
  cur="$cur$p/"
done
mkdir -p "$S/$M"
rsync -a --exclude node_modules "$T/🧰️scripts/1113-entry/work/$M/" "$S/$M/"
