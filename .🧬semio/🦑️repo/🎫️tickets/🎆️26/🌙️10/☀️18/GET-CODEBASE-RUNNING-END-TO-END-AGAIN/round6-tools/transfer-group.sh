#!/bin/sh
# usage: transfer-group.sh <tag> <step>...   transfers the steps in order onto the checkout (merging peer edits), then smoke-tests the CLI; stops at the first problem
TAG="$1"; shift
HERE="$(cd "$(dirname "$0")" && pwd)"
cd /Users/ueli/Documents/semio || exit 2
for s in "$@"; do
  python3 "$HERE/transfer.py" "$s" --tag "$TAG" || { echo "TRANSFER STOPPED at $s"; exit 3; }
done
echo "smoke $(date +%H:%M:%S)"
if bun repo --help > /private/tmp/i6/smoke-help.txt 2>&1 && /usr/bin/grep -q "^repo - " /private/tmp/i6/smoke-help.txt; then echo "smoke: bun repo --help ok"; else echo "SMOKE FAIL: bun repo --help"; tail -5 /private/tmp/i6/smoke-help.txt | cut -c1-300; exit 4; fi
if repo --help > /dev/null 2>&1; then echo "smoke: repo --help ok"; else echo "SMOKE FAIL: repo --help"; exit 4; fi
if bun repo cache status > /private/tmp/i6/smoke-cache.txt 2>&1 && /usr/bin/grep -q "script\] {\"owner\":\"repository:build\",\"command\":\"repo\",\"stage\":\"complete\"" /private/tmp/i6/smoke-cache.txt; then echo "smoke: bun repo cache status ok"; else echo "SMOKE FAIL: bun repo cache status"; /usr/bin/grep -v "source-admission\|^\[script\] {" /private/tmp/i6/smoke-cache.txt | tail -6 | cut -c1-300; exit 4; fi
echo "group ok"
