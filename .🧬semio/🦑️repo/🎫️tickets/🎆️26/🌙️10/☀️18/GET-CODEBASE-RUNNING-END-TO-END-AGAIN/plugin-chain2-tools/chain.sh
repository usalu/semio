#!/bin/sh
# usage: chain.sh TAG [--update] cargo-args...   (optional lock update first, then run.sh retry wrapper)
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
G="$C/🗑️generated/plugin-chain2-1022"
TAG=$1; shift
if [ "$1" = "--update" ]; then
  shift
  sh "$C/plugin-chain2-tools/run.sh" "$G/$TAG-update.log" update --workspace --offline
  echo "update rc=$?" > "$G/$TAG.rc"
fi
sh "$C/plugin-chain2-tools/run.sh" "$G/$TAG.log" "$@"
echo "check rc=$?" >> "$G/$TAG.rc"
