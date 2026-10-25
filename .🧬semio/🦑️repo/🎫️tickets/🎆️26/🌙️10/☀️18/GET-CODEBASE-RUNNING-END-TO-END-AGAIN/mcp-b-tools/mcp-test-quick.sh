#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
SEMIO_TEST_LEVEL=quick exec nice -n 10 repo --tool=bazel run repo-mcp:test
