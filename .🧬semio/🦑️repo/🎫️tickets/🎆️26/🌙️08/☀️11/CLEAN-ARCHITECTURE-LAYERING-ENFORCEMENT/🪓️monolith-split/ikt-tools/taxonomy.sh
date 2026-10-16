#!/bin/zsh
# usage: taxonomy.sh <label> <scope-relative-path>
. "$(dirname "$0")/env.sh"
cd "$R"
perl -e 'alarm 1500; exec @ARGV' -- bun ./📜️script.ts verify taxonomy report --scope "$2" > "$G/ikt-taxonomy-$1.log" 2>&1
echo "exit $?" >> "$G/ikt-taxonomy-$1.log"
