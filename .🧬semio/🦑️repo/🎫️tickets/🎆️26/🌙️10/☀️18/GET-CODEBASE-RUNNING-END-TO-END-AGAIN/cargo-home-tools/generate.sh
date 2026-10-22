#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
SEMIO_BUILD_GRAPH_POLICY=current exec nice -n 10 bun repo --tool=bazel run @semio-tech/framework-rs:generate --configuration=prototype
