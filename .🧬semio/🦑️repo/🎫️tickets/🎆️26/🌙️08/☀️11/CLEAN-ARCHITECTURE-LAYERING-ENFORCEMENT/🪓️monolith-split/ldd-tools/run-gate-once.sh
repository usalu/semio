#!/bin/sh
export LC_ALL=en_US.UTF-8
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/ldd-sbx
tag="$1"; pkg="$2"; secs="${3:-3000}"; shift 3
mkdir -p "$S"
cd "$R" || exit 9
rm -rf "$S/$tag-base" "$S/$tag-combined"
start=$(date +%s)
perl -e "alarm $secs; exec @ARGV" -- nice bun ./📜️script.ts new crates "$pkg" --check-applied --base="${BASE:-f856dff0813}" --samples=400 --sandbox="$S/$tag" "$@" > "$F/🗑️generated/ldd-$tag.log" 2> "$F/🗑️generated/ldd-$tag.err"
echo "exit=$? seconds=$(( $(date +%s) - start ))" > "$F/🗑️generated/ldd-$tag.exit"
