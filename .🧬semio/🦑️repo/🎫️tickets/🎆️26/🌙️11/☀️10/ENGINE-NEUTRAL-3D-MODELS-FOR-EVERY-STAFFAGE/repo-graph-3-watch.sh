#!/bin/sh
# usage: repo-graph-3-watch.sh NAME  appends "time progress rssMB records" every 60 s until NAME.end exists
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/🗑️generated/repo-graph-3"
while [ ! -f "$G/$1.end" ]; do
  P=$(grep -o "Inferring projects: [0-9]* of 4754" "$G/$1.log" | tail -1 | grep -o "[0-9]* of")
  R=$(ps -axo rss,command | grep "[b]un" | grep -v "service host\|repo-watch\|resident" | sort -n -r | head -1 | awk '{print int($1/1024)}')
  N=$(ls .cache/b/g/m/inference/selections/*/np 2>/dev/null | grep -c "\.json$")
  echo "$(date +%T) progress=$P rssMB=$R records=$N" >> "$G/$1.watch"
  sleep 60
done
