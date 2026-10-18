#!/usr/bin/env bash
# usage: all.sh <before|after> [slug...]
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
phase=$1; shift
while read -r dir slug; do
  if [ $# -gt 0 ]; then case " $* " in *" $slug "*) ;; *) continue;; esac; fi
  "$C/gocanon/run.sh" "$dir" "$slug" "$phase" 2>&1 | tail -1
done < "$C/gocanon/modules.txt"
