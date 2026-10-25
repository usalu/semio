#!/bin/sh
# usage: chain.sh TAG [--update] cargo-args...
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
G="$C/🗑️generated/plug-al-b"
TAG=$1; shift
if [ "$1" = "--update" ]; then
  shift
  M=""; for a in "$@"; do case "$a" in --manifest-path=*) M="$a";; esac; done
  sh "$C/plug-al-b-tools/run.sh" "$G/$TAG-update.log" update --workspace --offline $M
  echo "update rc=$?" > "$G/$TAG.rc"
fi
sh "$C/plug-al-b-tools/run.sh" "$G/$TAG.log" "$@"
echo "check rc=$?" >> "$G/$TAG.rc"
