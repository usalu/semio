#!/bin/zsh
# 🧪️ WG11 session 14d: the canonical per-process run of one lib test binary, in bounded chunks (one Bash call per chunk). Lists the
# binary's tests, keeps those matching <regex>, and runs chunk <index> of <size> through wg11-isolated.sh.
# usage: zsh wg11-run-lib.sh <test binary> <regex> <chunk size> <chunk index> <out prefix>
set -u
bin="$1"; regex="$2"; size="$3"; index="$4"; out="$5"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg11
names="$out.names"
"$bin" --list --format terse 2>/dev/null | /usr/bin/grep ': test$' | sed 's/: test$//' | /usr/bin/grep -E "$regex" > "$names.all"
start=$(( index * size + 1 ))
sed -n "${start},$(( start + size - 1 ))p" "$names.all" > "$names"
echo "[wg11-lib] chunk $index: $(wc -l < "$names" | tr -d ' ') of $(wc -l < "$names.all" | tr -d ' ') tests $(date '+%F %T')"
zsh "$W/wg11-isolated.sh" "$bin" "$names" "$out.tsv" "$out-details.txt"
