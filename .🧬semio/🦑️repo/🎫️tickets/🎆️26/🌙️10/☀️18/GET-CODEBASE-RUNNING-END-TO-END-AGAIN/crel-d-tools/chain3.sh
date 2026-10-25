#!/bin/sh
# Ticket input script: typecheck + full unit suites of the course client and the course site after the language chunks.
here=$(cd "$(dirname "$0")" && pwd)
sh "$here/retry.sh" t8 --tool=bazel run @semio-tech/course-react:typecheck
sh "$here/retry.sh" t9 --tool=bazel run @semio-tech/course-react:test -- quick
sh "$here/retry.sh" t10 --tool=bazel run @teaching/architecture-course:typecheck
sh "$here/retry.sh" t11 --tool=bazel run @teaching/architecture-course:test -- quick
