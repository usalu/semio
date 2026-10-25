#!/bin/sh
# Ticket input script: typecheck + unit suites of the course client and the course site, retried over peer churn.
here=$(cd "$(dirname "$0")" && pwd)
sh "$here/retry.sh" u1 --tool=bazel run @semio-tech/course-react:typecheck
sh "$here/retry.sh" u2 --tool=bazel run @semio-tech/course-react:test -- quick
sh "$here/retry.sh" u3 --tool=bazel run @teaching/architecture-course:typecheck
sh "$here/retry.sh" u4 --tool=bazel run @teaching/architecture-course:test -- quick
