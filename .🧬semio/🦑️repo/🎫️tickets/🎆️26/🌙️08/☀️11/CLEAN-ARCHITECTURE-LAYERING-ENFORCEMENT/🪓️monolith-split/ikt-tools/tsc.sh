#!/bin/zsh
# usage: tsc.sh <label> <tsconfig>
. "$(dirname "$0")/env.sh"
cd "$R"
perl -e 'alarm 1500; exec @ARGV' -- node_modules/.bin/tsc --noEmit -p "$2" > "$G/ikt-tsc-$1.log" 2>&1
echo "exit $?"; grep -c 'error TS' "$G/ikt-tsc-$1.log"
