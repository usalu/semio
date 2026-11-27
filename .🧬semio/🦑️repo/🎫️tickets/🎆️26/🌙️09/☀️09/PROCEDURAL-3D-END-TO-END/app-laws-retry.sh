#!/bin/sh
# 🔁️ Repins composition law hashes and runs the generation3d app laws, retrying graph reads that met a concurrent mid-edit source.
cd "$(dirname "$0")/../../../../../../.." || exit 1
T="$(pwd)/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
TARGET="${1:-test-generation3d-app-laws}"
for attempt in 1 2 3 4 5; do
  (cd "✏️s/🧑‍💻dev/🧩️composition" && SEMIO_LANGUAGE=en repo tools exec bun -- "$T/ownership-hash-drift.ts" --write 2>&1 | grep "^suite\|^unit\|^missing")
  LOG="$T/🗑️generated/$TARGET-$attempt.log"
  SEMIO_LANGUAGE=en repo run "@semio-tech/s-composition-laws-rs:$TARGET" > "$LOG" 2>&1; code=$?
  echo "attempt=$attempt exit=$code"
  grep -qE "BUILD_SOURCE_UNPARSEABLE|BUILD_SOURCE_CONFLICT|ownership/🟦️.ts" "$LOG" || break
  sleep 120
done
grep -hE "^test result|FAILED|panicked|^error" "$LOG" | sort | uniq -c | head -30 | cut -c1-300
tail -3 "$LOG" | cut -c1-300
