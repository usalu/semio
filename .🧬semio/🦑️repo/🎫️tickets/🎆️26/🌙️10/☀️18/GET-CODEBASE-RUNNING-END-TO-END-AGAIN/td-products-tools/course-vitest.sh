#!/bin/sh
# 🎓 usage: course-vitest.sh <mirror-root> [comma-separated include paths relative to the course package]
# Runs the course Vitest suite of a MIRROR with the OFFICIAL configuration (workspace aliases, provide table): the mirror must hold 🧰️framework + 🎓️teaching (mirror.py create <name> --only framework 🎓️teaching --no-closure) and be seeded with the gitignored generated inputs (seed-mirror.py). No include list = the official include list.
M="$1"; T="$(cd "$(dirname "$0")" && pwd)"
python3 -I "$T/seed-mirror.py" "$M" >/dev/null || exit 2
cp "$T/vitest.all.config.ts" "$M/tp-vitest.config.ts"
COURSE="$M/🎓️teaching/🏛️architecture/📚️course"
cd "$COURSE" || exit 2
TP_INCLUDE="${2:-}" exec nice -n 10 node "$M/node_modules/vitest/vitest.mjs" run --config "$M/tp-vitest.config.ts" --configLoader native --testTimeout 170000 --hookTimeout 170000
