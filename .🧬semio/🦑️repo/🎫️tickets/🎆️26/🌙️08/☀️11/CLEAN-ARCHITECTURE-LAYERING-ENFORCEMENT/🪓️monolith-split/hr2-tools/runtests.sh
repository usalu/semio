#!/bin/zsh
# usage: runtests.sh <list> <out.tsv>
cd /Users/ueli/Documents/semio
LIST="$1"; OUT="$2"; : > "$OUT"
run_one() {
  f="$1"
  log=$(perl -e 'alarm 300; exec @ARGV' -- bun test "./$f" 2>&1); code=$?
  pass=$(printf '%s' "$log" | /usr/bin/grep -E '^ *[0-9]+ pass' | tail -1 | awk '{print $1}')
  fail=$(printf '%s' "$log" | /usr/bin/grep -E '^ *[0-9]+ fail' | tail -1 | awk '{print $1}')
  failing=$(printf '%s' "$log" | /usr/bin/grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]//' | tr '\n' ';' | cut -c1-600)
  printf '%s\t%s\t%s\t%s\t%s\n' "$f" "$code" "${pass:-?}" "${fail:-?}" "$failing"
}
export -f run_one 2>/dev/null
while IFS= read -r f; do
  [ -z "$f" ] && continue
  run_one "$f" >> "$OUT" &
  while [ "$(jobs -r | wc -l)" -ge 3 ]; do sleep 1; done
done < "$LIST"
wait
echo done > "$OUT.done"
