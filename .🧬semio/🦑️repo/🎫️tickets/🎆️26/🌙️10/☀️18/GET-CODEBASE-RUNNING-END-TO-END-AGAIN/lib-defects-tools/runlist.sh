#!/bin/sh
# usage: runlist.sh <root> <list-file> <log-dir>   runs every suite of the list (two at a time) and prints one summary line each
T=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$3"
xargs -P2 -I{} sh -c '"$0/run.sh" "$1" "$2" "$3" >/dev/null; f="$3/$(printf "%s" "$2" | python3 -c "import re,sys;print(re.sub(r\"[^A-Za-z0-9]+\",\"_\",sys.stdin.read().strip())[-110:])").log"; printf "%s %s\n" "$(grep -E "^ *[0-9]+ (pass|fail)$" "$f" | tr -s " \n" " ")" "$2"' "$T" "$1" {} "$3" < "$2"
