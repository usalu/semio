#!/bin/sh
# 🧪️ Waits for the peer queue to idle and memory >=45% free, then runs the scene suite; retries warden kills and infrastructure refusals.
cd "$(dirname "$0")/../../../../../../.."
G="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/🗑️generated"
cd "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing" || exit 1
for i in $(seq 40); do
  while pgrep -f "repo cargo" >/dev/null; do sleep 30; done
  for w in $(seq 30); do [ "$(memory_pressure | awk '/free percentage/{gsub("%","",$NF);print $NF}')" -ge 45 ] && break; sleep 30; done
  pgrep -f "repo cargo" >/dev/null && continue
  SEMIO_LANGUAGE=en repo capacity run --memory=3500 --cpu=1 --jobs=1 -- repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --no-fail-fast -p semio-s-artifact-draw-drawing-scene-mounted -- --test-threads=1 > "$G/r27-scene-mounted.txt" 2>&1
  echo "attempt $i exit $? $(date +%T)" >> "$G/r27m2.attempts"
  grep -aq "^test result" "$G/r27-scene-mounted.txt" && exit 0
  grep -aqE "^error(\[|:) |could not compile" "$G/r27-scene-mounted.txt" && ! grep -aqE "warden|Wächter|Closed .* required" "$G/r27-scene-mounted.txt" && exit 1
  sleep 60
done
