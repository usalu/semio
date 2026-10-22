#!/bin/sh
# usage: chain-bazel.sh <tag> <project:target>...   runs each sequentially on --tool=bazel
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag=$1; shift
for t in "$@"; do
  n="$tag-$(echo "$t" | sed 's/[^A-Za-z0-9]/_/g')"
  sh "$C/demo-verify-tools/run.sh" "$n" --tool=bazel run "$t"
  echo "$t bazel rc=$(cat "/Users/ueli/Documents/semio/$C/🗑️generated/demo-verify/$n.rc")" >> "/Users/ueli/Documents/semio/$C/🗑️generated/demo-verify/$tag-summary.txt"
done
echo done >> "/Users/ueli/Documents/semio/$C/🗑️generated/demo-verify/$tag-summary.txt"
