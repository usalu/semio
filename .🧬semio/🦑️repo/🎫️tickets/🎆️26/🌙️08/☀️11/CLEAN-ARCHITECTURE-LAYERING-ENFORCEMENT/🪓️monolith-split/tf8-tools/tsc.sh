#!/bin/zsh
# usage: tsc.sh <tsconfig> <log>
cd /Users/ueli/Documents/semio
perl -e 'alarm 2400; exec @ARGV' -- node node_modules/typescript/bin/tsc --noEmit -p "$1" --listFilesOnly > "$2.files" 2>&1
perl -e 'alarm 2400; exec @ARGV' -- node node_modules/typescript/bin/tsc --noEmit -p "$1" > "$2" 2>&1
echo "exit=$?" >> "$2"
