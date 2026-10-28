#!/bin/sh
# usage: learn.sh <profile-base> <rounds> -- cmd...   learns denied package.json reads, prints process names
base=$1; rounds=$2; shift 3
cp "$base" cur.sb
for i in $(seq 1 $rounds); do
  sandbox-exec -f cur.sb "$@" >out.$i.txt 2>&1; code=$?
  sleep 3
  /usr/bin/log show --last 40s --style compact --predicate 'eventMessage CONTAINS "Sandbox" AND eventMessage CONTAINS "deny"' 2>/dev/null | sed -n 's/.*Sandbox: //p' | grep "package.json\|Cargo.toml" | sort -u > den.$i.txt
  echo "round $i exit=$code denials=$(wc -l < den.$i.txt)"
  cat den.$i.txt | head -40
  [ -s den.$i.txt ] || break
  sed -n 's/^[^ ]* deny([0-9]*) file-read-data //p' den.$i.txt | while read -r p; do printf '(allow file-read-data (literal "%s"))\n' "$p" >> cur.sb; done
done
