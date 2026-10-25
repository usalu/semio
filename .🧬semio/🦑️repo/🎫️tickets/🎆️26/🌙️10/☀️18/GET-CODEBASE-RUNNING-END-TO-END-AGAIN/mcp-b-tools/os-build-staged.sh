#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
exec nice -n 10 repo --tool=bazel run @semio-tech/framework-os-mcp-rs:build --output=stream
