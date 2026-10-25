#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
exec nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --no-fail-fast --message-format=short -p semio-framework-repo-codebase -p semio-framework-repo-identity -p semio-framework-repo-workspace
