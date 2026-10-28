#!/bin/sh
# usage: both.sh <repo-relative test file>  -> plain and official-envelope summaries + failing test names
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
f="$1"; shift
echo "### $f"
for mode in plain env; do
  if [ $mode = plain ]; then out=$(bun test "./$f" "$@" 2>&1); else out=$(VF_WALL=300 bun "$C/ver-f-tools/proc-run.ts" file "$PWD/$f" "$@" 2>&1); fi
  printf '%s: %s\n' $mode "$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' | tr -s ' \n' ' ')"
  echo "$out" | /usr/bin/grep -E '^\(fail\)' | cut -c1-260 | sed "s/^/  $mode /"
done
