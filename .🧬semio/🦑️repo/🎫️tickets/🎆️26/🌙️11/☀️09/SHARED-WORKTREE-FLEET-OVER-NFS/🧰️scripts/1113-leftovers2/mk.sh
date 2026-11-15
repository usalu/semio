#!/bin/bash
# builds temp tree W with a copy of the module, symlinks elsewhere
set -e
L=/Users/Shared/semio
W=${1:?}
rm -rf "$W"; mkdir -p "$W"
for e in "$L"/* "$L"/.[!.]*; do b=$(basename "$e"); case "$b" in .git|🧰️framework|.🧬semio) continue;; esac; ln -s "$e" "$W/$b"; done
cp "$L/.gitignore" "$W/.gitignore" 2>/dev/null || true
ln -s "$L/.🧬semio" "$W/.🧬semio"
mkdir -p "$W/🧰️framework"
for e in "$L/🧰️framework"/*; do b=$(basename "$e"); [ "$b" = "🛍️products" ] && continue; ln -s "$e" "$W/🧰️framework/$b"; done
mkdir -p "$W/🧰️framework/🛍️products"
for e in "$L/🧰️framework/🛍️products"/*; do b=$(basename "$e"); [ "$b" = "🦑️repo" ] && continue; ln -s "$e" "$W/🧰️framework/🛍️products/$b"; done
R="$W/🧰️framework/🛍️products/🦑️repo"; mkdir -p "$R"
for e in "$L/🧰️framework/🛍️products/🦑️repo"/* "$L/🧰️framework/🛍️products/🦑️repo"/.[!.]*; do [ -e "$e" ] || continue; b=$(basename "$e"); [ "$b" = "🔨️modules" ] && continue; ln -s "$e" "$R/$b"; done
mkdir -p "$R/🔨️modules"
for e in "$L/🧰️framework/🛍️products/🦑️repo/🔨️modules"/*; do b=$(basename "$e"); [ "$b" = "🤝️shared-worktree" ] && continue; ln -s "$e" "$R/🔨️modules/$b"; done
rm -f "$W/node_modules"; ln -s /Users/ueli/.cache/c5c8/nm/node_modules "$W/node_modules"
cp -R "$L/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree" "$R/🔨️modules/🤝️shared-worktree"
