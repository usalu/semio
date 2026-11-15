#!/bin/zsh
# usage: parse.sh <overlay name>; parse proof of every file the landing changes or adds: .mjs/.ts with bun build --no-bundle, .json with python, .py with ast
HERE=${0:A:h};S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
python3 -I - "$HERE" <<'P' | while IFS= read -r f; do
import json,os,sys
sys.path.insert(0,sys.argv[1]);import edits
for rel,*_ in edits.EDITS:print(rel)
for dest,_ in edits.NEWFILES:print(dest)
P
  src=$HERE/new/$f; [[ -f $src ]] || src=$S/$1/$f
  case $f in
    *.json) python3 -I -c 'import json,sys;json.load(open(sys.argv[1],encoding="utf8"))' "$src" && echo "json ok  ${f:t}" ;;
    *.py) python3 -I -c 'import ast,sys;ast.parse(open(sys.argv[1],encoding="utf8").read())' "$src" && echo "py ok    ${f:t}" ;;
    *.mjs|*.ts) $BUN build --no-bundle "$src" > /dev/null && echo "bun ok   ${f:t}" ;;
  esac
done
