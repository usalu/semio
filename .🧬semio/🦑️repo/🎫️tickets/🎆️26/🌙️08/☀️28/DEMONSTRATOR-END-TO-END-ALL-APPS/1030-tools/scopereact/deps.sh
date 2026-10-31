#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
export SEMIO_LANGUAGE=en
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1030"
for step in "$@"; do
  repo dependencies $step --all > "$G/scopereact-deps-${step}.txt" 2>&1
  echo $? > "$G/scopereact-deps-${step}.rc"
done
