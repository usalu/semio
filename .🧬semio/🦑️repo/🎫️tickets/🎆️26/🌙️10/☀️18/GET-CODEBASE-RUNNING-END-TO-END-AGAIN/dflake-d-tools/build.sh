#!/bin/sh
# usage: build.sh <tag> [crate]   graph-free build of the test binaries of a crate; prints the lib test executable into <tag>.bin
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/dflake-d"
crate=${2:-semio-framework-repo-dashboard-daemon}
try=1
while :; do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test -p "$crate" --lib --no-run --message-format=short > "$O/$1.log" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && [ $try -lt 4 ] && /usr/bin/grep -q 'SEMIO_STALE_IMPLEMENTATION\|Stale loaded Source revision\|Current shared Source-owner generation changed' "$O/$1.log"; then try=$((try+1)); continue; fi
  break
done
/usr/bin/grep -o "\.🧬semio[^ )]*${crate//-/_}-[0-9a-f]*" "$O/$1.log" | tail -1 > "$O/$1.bin"
echo "rc=$rc tries=$try bin=$(cat "$O/$1.bin")"
/usr/bin/grep -E "^error|error\[" "$O/$1.log" | head -20 | cut -c1-400
[ -n "$(cat "$O/$1.bin")" ] && cp "$(cat "$O/$1.bin")" "$O/$1.exe" && echo "copied to $O/$1.exe"
