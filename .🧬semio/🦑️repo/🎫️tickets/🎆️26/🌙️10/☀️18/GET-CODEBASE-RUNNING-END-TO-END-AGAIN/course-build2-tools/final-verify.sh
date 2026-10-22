#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
export SEMIO_BUILD_GRAPH_POLICY=current
nice -n 10 bun repo --tool=bazel run @teaching/proctor:build --configuration=release
echo "EXIT proctor=$?"
nice -n 10 bun repo --tool=bazel run @teaching/architecture-course:build
echo "EXIT course=$?"
