#!/bin/sh
# 🧮 Runs a suite list on a root with a junit report and prints the failure set: sh suites.sh <root> <suites.txt> <label>   (output: C/🗑️generated/edit30/<label>.{xml,out,set})
here=$(cd "$(dirname "$0")" && pwd); root=$1; list=$2; label=$3; out="$here/../🗑️generated/edit30"; mkdir -p "$out"
export SEMIO_TEST_BAZEL_BINARY=${SEMIO_TEST_BAZEL_BINARY:-$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/*/bazel 2>/dev/null | head -1)}
cd "$root" && bun test --timeout 600000 --reporter=junit --reporter-outfile="$out/$label.xml" $(tr '\n' ' ' < "$list") > "$out/$label.out" 2>&1
python3 -I "$here/staged/freshmtime/junit-set.py" "$out/$label.xml" > "$out/$label.set"; head -1 "$out/$label.set"
