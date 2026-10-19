#!/usr/bin/env bash
# usage: sync.sh -- syncs the repo modules (and the framework Go modules) into the throwaway copy
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/gofin6
REAL=/Users/ueli/Documents/semio; COPY="$S/repo-copy"
for rel in "🧰️framework/🛍️products/🦑️repo/🔨️modules" "🧰️framework/🔨️modules/📁️filesystem" "🧰️framework/🛍️products/🦑️repo/📜️script.ts"; do
  [ -d "$REAL/$rel" ] || continue
  rsync -a --delete --exclude='target/' --exclude='dist/' --exclude='.cache/' --exclude='node_modules/' --exclude='🗑️generated/' --exclude='.DS_Store' "$REAL/$rel/" "$COPY/$rel/" || exit 4
done
cp "$REAL/go.work" "$COPY/go.work"
