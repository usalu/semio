#!/bin/bash
# usage: run-f1.sh <list-file-with-paths> <result-tsv> <scratch-log>
list="$1"; out="$2"; log="$3"; : > "$out"
while IFS= read -r fp; do
  case "$fp" in
    *.py) kind=pysyntax; python3 -I -c 'import ast,sys; ast.parse(open(sys.argv[1],encoding="utf-8").read(), sys.argv[1])' "$fp" > "$log" 2>&1; rc=$? ;;
    *.mjs) kind=mjssyntax; node --check "$fp" > "$log" 2>&1; rc=$? ;;
    *fixtures*|*echo.ts|*relaunch.ts|*child.ts|*trampoline.ts) kind=tssyntax; bun build "$fp" --no-bundle --target bun > /dev/null 2> "$log"; rc=$? ;;
    *) kind=bun-test; bun test --timeout 120000 "./$fp" > "$log" 2>&1; rc=$? ;;
  esac
  printf '%s\t%s\t%s\t%s\n' "$rc" "$kind" "$fp" "$(tail -n 3 "$log" | tr '\n\t' '  ' | tail -c 300)" >> "$out"
done < "$list"
echo FINISHED >> "$out"
