#!/bin/zsh
# SF3: runs 'new crates --check-applied --base=<ref>' for every artifact of a list slice and retries (60 s apart, at most 20 times) while the run dies before producing a report. usage: check-all.sh <base> <summary-file> <worker> <artifact-facade-dir...>
cd /Users/ueli/Documents/semio || exit 2
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
SB=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad
base=$1; summary=$2; worker=$3; shift 3
for facade in "$@"; do
  art=${facade%/📦️packages/🦀️rust}
  name=$(print -r -- "$art" | /usr/bin/awk -F/ '{print $3"-"$5}' | /usr/bin/sed 's/[^[:alnum:]-]/_/g')
  key=$(print -r -- "$art" | /usr/bin/shasum | /usr/bin/cut -c1-8)

  tries=0
  while true; do
    rm -rf "$SB/sf3-ck-$key" "$SB/sf3-ck-$key-combined" "$SB/sf3-ck-$key-base"
    start=$(date +%s)
    nice -n 10 perl -e 'alarm 3300; exec @ARGV' -- bun ./📜️script.ts new crates "$facade" --check-applied --base=$base --sandbox="$SB/sf3-ck-$key" > "$G/sf3-ck-$key.log" 2>&1
    rc=$?
    if /usr/bin/grep -q '"onlyCombined"' "$G/sf3-ck-$key.log"; then break; fi
    if /usr/bin/grep -q 'Missing Rust source input' "$G/sf3-ck-$key.log"; then break; fi
    tries=$((tries+1)); [ $tries -gt 20 ] && break
    sleep 60
  done
  echo "$key $art exit=$rc seconds=$(($(date +%s)-start)) tries=$tries worker=$worker" >> "$G/$summary"
  rm -rf "$SB/sf3-ck-$key" "$SB/sf3-ck-$key-combined" "$SB/sf3-ck-$key-base"
done
echo "done worker=$worker" >> "$G/$summary"
