#!/bin/zsh
# usage: changed14.sh > changed14.tsv ; touches per file in the last 14 days (read-only git, no optional locks)
cd /Users/Shared/semio && git --no-optional-locks -c core.quotepath=false log --since="14 days ago" --name-only --format='' | sort | uniq -c | sed 's/^ *\([0-9]*\) /\1\t/'
