#!/bin/sh
# Ticket input script: the site's unit tests with the pinned budgets, then the rehearsal topology of the course end-to-end gate.
#   chain2.sh <run id>
here=$(cd "$(dirname "$0")" && pwd)
ticket=$(cd "$here/.." && pwd)
sh "$here/run.sh" t6 --tool=bazel run @teaching/architecture-course:test -- quick || exit $?
exec sh "$ticket/e2e-orch-tools/run.sh" course --phase e2e --course-topology rehearsal --go --run-id "$1"
