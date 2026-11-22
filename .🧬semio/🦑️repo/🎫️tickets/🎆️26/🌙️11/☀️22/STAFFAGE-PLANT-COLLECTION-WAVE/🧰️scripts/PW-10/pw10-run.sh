#!/bin/sh
# 🏃 Builds the given PW-10 plants (default all three), validates each candidate with the round-3 validator (`--volumes`, add NOPRINT=1 to skip the print checks) and writes the logs to 🗑️generated/PW-10.
set -u
D=$(cd "$(dirname "$0")"; pwd)
T2="$D/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
G="$D/../../🗑️generated/PW-10"
mkdir -p "$G"
for id in ${@:-windmill-palm banana-plant tree-fern}; do
  (cd "$D" && repo tools exec bun -- "pw10-$id.ts" ${SHIP:+--ship}) 2>&1 | grep -v "^\[" | grep -v "^2D" > "$G/$id.build.txt"
  (cd "$T2" && repo tools exec bun -- r3-validate.ts --file "$G/$id/🔣️.json" --volumes ${NOPRINT:+--no-print}) 2>&1 | grep -v "^\[" | cut -c1-600 > "$G/$id.validate.txt"
  echo "== $id"; cat "$G/$id.build.txt" | head -2; cat "$G/$id.validate.txt"
done
