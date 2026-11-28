#!/bin/sh
# 🧪️ Runs the named Draw packages (default viewer) once peer cargo builds are idle (not gated on the final pipeline: the coordinator allows small runs beside it) and the host has 45 percent free memory; retries after infrastructure failures.
cd "$(dirname "$0")/../../../../../../.."
T="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
tag="$1"; shift
cd "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing" || exit 1
for pkg in "$@"; do
  out="$T/🗑️generated/r27-$pkg$tag.txt"
  for attempt in 1 2 3 4 5 6; do
    while pgrep -f "script.ts repo [c]a" >/dev/null; do sleep 10; done
    until [ "$(memory_pressure | awk '/free percentage/{gsub("%","",$NF);print $NF}')" -ge 45 ]; do sleep 30; done
    SEMIO_LANGUAGE=en repo capacity run --memory=3500 --cpu=1 --jobs=1 -- repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --no-fail-fast -p semio-s-artifact-draw-drawing-$pkg > "$out" 2>&1
    grep -aq "^test result" "$out" && break
    sleep 60
  done
done
echo RUN-DONE
