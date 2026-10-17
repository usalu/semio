#!/bin/zsh
# usage: oracle_number.sh  -> typechecks number lib+tests, base (HEAD) vs current + companion via `extern crate self`
cd /Users/ueli/Documents/semio || exit 1
S=/Users/ueli/.cache/tc3/number
M="🧰️framework/🔨️modules/🔢️number"
rm -rf $S && mkdir -p $S/base $S/cur
git -c core.quotepath=false archive HEAD "$M" | tar -x -C $S/base
cp -R "$M" $S/cur/ 2>/dev/null; mkdir -p $S/cur/"🧰️framework/🔨️modules"; rm -rf $S/cur/"🧰️framework/🔨️modules/🔢️number"; mkdir -p "$S/cur/🧰️framework/🔨️modules"; cp -R "$M" "$S/cur/🧰️framework/🔨️modules/"
ROOT="🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust/🦀️.rs"
python3 - "$S/cur/$ROOT" <<'PY'
import sys
p=sys.argv[1]; s=open(p,encoding="utf-8").read()
s+='\nextern crate self as semio_framework_number;\n#[cfg(test)]\n#[path = "../../🧪️tests/🥣️companion/🦀️.rs"]\nmod companion;\n'
open(p,"w",encoding="utf-8").write(s)
PY
for side in base cur; do
  (cd $S/$side && nice -n 10 rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name semio_framework_number --emit=metadata -o /dev/null --test "$ROOT" 2> $S/$side.err; echo "$side exit $?")
  /usr/bin/grep -c "^error" $S/$side.err
done
