#!/bin/sh
# usage: run-gate.sh <tag> <package-dir> [extra flags...]
export LC_ALL=en_US.UTF-8
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pgg-sbx
tag="$1"; pkg="$2"; shift 2
mkdir -p "$S"
cd "$R" || exit 9
start=$(date +%s)
tries=0
while [ $tries -lt 20 ]; do
  tries=$((tries + 1))
  rm -rf "$S/$tag-base" "$S/$tag-combined"
  nice bun ./📜️script.ts new crates "$pkg" --check-applied --base="${BASE:-f856dff0813}" --samples=400 --sandbox="$S/$tag" "$@" > "$F/🗑️generated/pgg-$tag.log" 2> "$F/🗑️generated/pgg-$tag.err"
  code=$?
  if [ -s "$F/🗑️generated/pgg-$tag.log" ]; then break; fi
  sleep 45
done
echo "exit=$code seconds=$(( $(date +%s) - start )) tries=$tries" > "$F/🗑️generated/pgg-$tag.exit"
