#!/bin/bash
shopt -s nullglob dotglob
W=$1
R=/Users/Shared/semio
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
case "$W" in */res1113/work|*/res1113/work2|*/res1113/work3) ;; *) echo bad; exit 1;; esac
rm -rf "$W"; mkdir -p "$W"
link_level(){ local dir=$1 keep=$2 base="$R/$1"
  mkdir -p "$W/$dir"
  for e in "$base"/*; do n=$(basename "$e"); [ "$n" = "$keep" ] && continue; [ "$n" = ".git" ] && continue; [ "$n" = ".cache" ] && continue; [ "$n" = node_modules ] && continue; ln -s "$e" "$W/$dir/$n"; done; }
link_level "" "🧰️framework"
link_level "🧰️framework" "🛍️products"
link_level "🧰️framework/🛍️products" "🦑️repo"
link_level "🧰️framework/🛍️products/🦑️repo" "🔨️modules"
link_level "🧰️framework/🛍️products/🦑️repo/🔨️modules" "🤝️shared-worktree"
cp -R "$R/$M" "$W/$M"
mkdir "$W/node_modules"; for e in "$R"/node_modules/*; do ln -s "$e" "$W/node_modules/$(basename "$e")"; done; ln -s /Users/ueli/Documents/semio-niloufar-wip/node_modules/ajv "$W/node_modules/ajv"
