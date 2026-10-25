#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=bazel --workers=off run @semio-tech/os-plugin-describe-rs:build --configuration=prototype
echo "[describe] rc=$?"
