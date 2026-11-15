#!/bin/bash
# Package T: builds a scratch mirror of the framework tree outside the worktree. Members in REAL are copies; every other entry is a symlink into the live tree.
set -e
LIVE=/Users/Shared/semio
DEST=$1
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
REAL=("$L/⚡️caching/🦀️cargo/🗃️unit-cache" "$L/🏗️build/🌐️remote-execution/🚀️offload")
rm -rf "$DEST"; mkdir -p "$DEST"
DIRS=()
for member in "${REAL[@]}"; do
  IFS=/ read -ra parts <<< "$member"; cur=""
  for part in "${parts[@]}"; do cur="${cur:+$cur/}$part"; [ "$cur" = "$member" ] || DIRS+=("$cur"); done
done
is_dir(){ for d in "${DIRS[@]}"; do [ "$d" = "$1" ] && return 0; done; return 1; }
is_real(){ for d in "${REAL[@]}"; do [ "$d" = "$1" ] && return 0; done; return 1; }
for rel in "${DIRS[@]}" ""; do
  [ -z "$rel" ] && [ "${#DIRS[@]}" -gt 0 ] || true
done
DIRS+=("")
for rel in "${DIRS[@]}"; do
  mkdir -p "$DEST/$rel"
  for entry in "$LIVE/$rel"/* "$LIVE/$rel"/.[!.]*; do
    [ -e "$entry" ] || continue
    name=$(basename "$entry"); child="${rel:+$rel/}$name"
    if is_dir "$child" || is_real "$child"; then continue; fi
    [ -L "$DEST/$child" ] || [ -e "$DEST/$child" ] || ln -s "$entry" "$DEST/$child"
  done
done
for member in "${REAL[@]}"; do mkdir -p "$DEST/$member"; rsync -a --exclude node_modules "$LIVE/$member/" "$DEST/$member/"; done
