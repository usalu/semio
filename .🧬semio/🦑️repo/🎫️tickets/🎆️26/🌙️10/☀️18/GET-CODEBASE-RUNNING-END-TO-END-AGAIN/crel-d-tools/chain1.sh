#!/bin/sh
# Ticket input script: typecheck of the course site, then the release build the entry chunk is measured from.
here=$(cd "$(dirname "$0")" && pwd)
sh "$here/run.sh" t4 --tool=bazel run @teaching/architecture-course:typecheck
sh "$here/run.sh" t5 --tool=bazel run @teaching/architecture-course:build --configuration=release
