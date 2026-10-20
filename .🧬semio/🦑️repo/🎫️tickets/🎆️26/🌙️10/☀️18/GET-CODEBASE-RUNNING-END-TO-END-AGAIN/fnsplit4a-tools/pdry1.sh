#!/bin/zsh
# usage: pdry1.sh "<file>|<name>|<line>|<lines>" : dry run, prints one compact line
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio
IFS='|' read -r f n l ln <<< "$1"
out=$(perl -e 'alarm 280; exec @ARGV' -- $T/pgo.sh "$f" "$n" 150 2>&1 | tr '\n' ' ' | cut -c1-420)
echo "[$n $ln] $out"
