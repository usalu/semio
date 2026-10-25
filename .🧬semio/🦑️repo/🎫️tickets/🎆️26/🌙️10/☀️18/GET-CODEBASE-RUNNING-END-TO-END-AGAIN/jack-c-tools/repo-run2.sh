#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
exec nice -n 10 bun repo --tool=bazel run "$@"
