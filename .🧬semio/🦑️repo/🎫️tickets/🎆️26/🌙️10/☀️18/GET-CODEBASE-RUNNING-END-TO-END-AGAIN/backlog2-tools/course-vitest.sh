#!/bin/sh
# 🎓 [backlog2] runs the course vitest files of a MIRROR with its own node_modules, native config loader, no workspace aliases (the mirror holds only part of the workspace). usage: course-vitest.sh <mirror-name> [comma-separated include globs relative to the course package]
M="${TMPDIR%/}/sm/$1"; T="$(cd "$(dirname "$0")" && pwd)"
V=$(ls -d "$M"/node_modules/.bun/vitest@*/node_modules/vitest | head -1)
COURSE="$M/🎓️teaching/🏛️architecture/📚️course"
cp "$T/vitest.config.mjs" "$M/b2-vitest.config.mjs"
cd "$COURSE" || exit 2
COURSE_ROOT="$COURSE" COURSE_INCLUDE="${2:-}" exec node "$V/vitest.mjs" run --config "$M/b2-vitest.config.mjs" --configLoader native --testTimeout 170000 --hookTimeout 170000
