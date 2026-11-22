#!/bin/bash
# usage: head.sh <path-relative-to-caching> [lines]
cd /Users/ueli/Documents/semio || exit 1
C="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
repo test-file "$C/$1" 2>&1 | grep -vE "^\[script\]|^\[test-file\]|^bun test" | grep -E "^error|^\(fail\)|Expected|Received|RepoError|^ *[0-9]+ (pass|fail)|passed|FAILED|^\s+at <anonymous>" | cut -c1-${3:-260} | head -${2:-30}
