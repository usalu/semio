#!/bin/zsh
# usage: fwcheck.sh <logname> <packages...>
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated"
log="$G/rg-core-$1.log"; shift
args=()
for p in "$@"; do args+=(-p "$p"); done
ALARM=1200 "$C/rg-core-tools/pl.sh" update --workspace --offline --manifest-path 🧰️framework/Cargo.toml > "$log.lock" 2>&1
for attempt in 1 2 3 4 5 6; do
  ALARM=7000 "$C/rg-core-tools/pl.sh" check --manifest-path ${MANIFEST:-🧰️framework/Cargo.toml} "${args[@]}" --lib --tests --keep-going ${FMT:---message-format=short} > "$log" 2>&1
  rc=$?
  if grep -q "System image plist required\|Bounded unique parent kernel\|SyntaxError: Export named\|Cannot find module\|cannot update the lock file\|database or disk is full" "$log"; then sleep 20; ALARM=1200 "$C/rg-core-tools/pl.sh" update --workspace --offline --manifest-path 🧰️framework/Cargo.toml > "$log.lock" 2>&1; continue; fi
  break
done
echo "exit=$rc" >> "$log"
