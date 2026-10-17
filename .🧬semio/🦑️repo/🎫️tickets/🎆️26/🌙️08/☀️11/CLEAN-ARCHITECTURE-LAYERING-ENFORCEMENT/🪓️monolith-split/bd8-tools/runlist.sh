#!/bin/zsh
# usage: runlist.sh <list> <out>; runs bun test per file (90 s cap), writes "file<TAB>pass<TAB>fail<TAB>firstError"
R=/Users/ueli/Documents/semio; cd $R; : > "$2"
while IFS= read -r f; do
  [ -z "$f" ] && continue
  case "$f" in *.test.*|*🟦️.ts|*.ts) ;; esac
  out=$( (ulimit -n 65536; nice -n 10 perl -e 'alarm 90; exec @ARGV' -- bun test "./$f" 2>&1) )
  p=$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ pass' | awk '{print $1}' | tail -1)
  fl=$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ fail' | awk '{print $1}' | tail -1)
  e=$(echo "$out" | /usr/bin/grep -m1 -E '^error|ENOENT|Cannot find|SEMIO_' | cut -c1-200)
  printf '%s\t%s\t%s\t%s\n' "$f" "${p:-?}" "${fl:-?}" "$e" >> "$2"
done < "$1"
