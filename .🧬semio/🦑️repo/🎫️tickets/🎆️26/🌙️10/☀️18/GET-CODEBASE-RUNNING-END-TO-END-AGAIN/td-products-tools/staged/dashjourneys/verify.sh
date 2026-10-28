#!/bin/sh
# 🧭 usage: verify.sh <mirror-root>   (mirror with the batch applied: stage.py apply --root <mirror-root>)
# laws: (1) the TypeScript host-scale law (shared vectors + TS model + SQLite oracle) and the fixture schema, (2) the Rust host-scale law, (3) the two formerly racy journeys behind the slow terminal, (4) the whole PTY journeys suite.
M="$1"; T="$(cd "$(dirname "$0")/../.." && pwd)"
D="$M/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys"
(cd "$D" && nice -n 10 bun test ./🟦️.ts -t "Host time scale|satisfy their schema") || exit 1
sh "$T/journeys-mirror.sh" "$M" host_time_scale || exit 1
SLOW=1 sh "$T/journeys-mirror.sh" "$M" native_files_browse maximum_native || exit 1
sh "$T/journeys-mirror.sh" "$M"
