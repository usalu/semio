#!/bin/sh
# usage: run-queue.sh <queue-file>   (lines: tag|package-dir|base)
export LC_ALL=en_US.UTF-8
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/pgr-sbx
mkdir -p "$S"
cd "$R" || exit 9
while IFS='|' read -r tag pkg base; do
  [ -z "$tag" ] && continue
  case "$tag" in \#*) continue;; esac
  if [ -f "$F/🗑️generated/pgr-$tag.exit" ]; then continue; fi
  start=$(date +%s)
  tries=0
  code=99
  while [ $tries -lt 12 ]; do
    tries=$((tries + 1))
    rm -rf "$S/$tag-base" "$S/$tag-combined" "$S/$tag"
    perl -e 'alarm 3600; exec @ARGV' -- nice bun ./📜️script.ts new crates "$pkg" --check-applied --base="$base" --samples=400 --sandbox="$S/$tag" > "$F/🗑️generated/pgr-$tag.log" 2> "$F/🗑️generated/pgr-$tag.err"
    code=$?
    if [ -s "$F/🗑️generated/pgr-$tag.log" ]; then break; fi
    sleep 45
  done
  echo "exit=$code seconds=$(( $(date +%s) - start )) tries=$tries" > "$F/🗑️generated/pgr-$tag.exit"
  rm -rf "$S/$tag-base" "$S/$tag-combined" "$S/$tag"
done < "$1"
echo done > "$F/🗑️generated/pgr-queue-$(basename "$1").done"
