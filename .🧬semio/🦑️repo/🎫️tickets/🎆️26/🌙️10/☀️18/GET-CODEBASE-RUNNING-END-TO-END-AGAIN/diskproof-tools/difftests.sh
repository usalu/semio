#!/bin/sh
# [lane diskproof] run a list of single test files one at a time under ROOT and print "<file>\t<pass>\t<fail>\t<failing test names>"
root="$1"; art="$2"; list="$3"
cd "$root" || exit 2
export SEMIO_TEST_ARTIFACT_DIR="$art"; mkdir -p "$art"
while IFS= read -r f; do
  out=$(bun test "./$f" 2>&1)
  p=$(printf '%s\n' "$out" | /usr/bin/grep -E '^ *[0-9]+ pass$' | tail -1 | tr -dc 0-9)
  x=$(printf '%s\n' "$out" | /usr/bin/grep -E '^ *[0-9]+ fail$' | tail -1 | tr -dc 0-9)
  n=$(printf '%s\n' "$out" | /usr/bin/grep -E '^\(fail\)' | sed 's/ \[[0-9.]*m\?s\]$//' | sort | tr '\n' ';')
  printf '%s\t%s\t%s\t%s\n' "$f" "${p:-?}" "${x:-?}" "$n"
done < "$list"
