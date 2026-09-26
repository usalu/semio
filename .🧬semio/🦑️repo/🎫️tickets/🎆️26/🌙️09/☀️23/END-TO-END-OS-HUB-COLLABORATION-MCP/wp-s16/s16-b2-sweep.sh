#!/bin/zsh
# 🧹️ S15 session 12: the hub-document sweep on catalog B2 inside `s` — for every creatable kind of the hub's creation catalog (index order
# of `generated/s15-b2-kinds.txt`) and each locale, the person-driven journey (`s16-hub-journey.mjs`): sign in → open the sweep space →
# create the kind → the creation saga opens it (its program installed by catalog generation) → rail verb → undo → redo.
# One persistent browser profile per locale, so a plugin's first kind installs from the hub and later ones from the device's store.
# usage: [S15_KINDS_FILE=<kinds list>] [S15_B2_SPACE=<space name>] zsh s15-b2-sweep.sh <serveOrigin> <tagPrefix> <locale en|de> [firstIndex] [lastIndex]
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-s16
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-s16-logs"
P="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-s16-profiles"
ORIGIN="$1"; TAG="$2"; LOC="$3"; FIRST="${4:-0}"; LAST="${5:-15}"
KINDS="${S15_KINDS_FILE:-$L/kinds-b2-7800.txt}"
PLUGINS=($(tail -n +2 "$KINDS" | awk '{ split($4, part, "."); print part[2] }'))
mkdir -p "$P/$TAG-$LOC"
cd "$W" || exit 1
for i in $(seq "$FIRST" "$LAST"); do
  plugin=${PLUGINS[$((i + 1))]}
  run="$TAG-$LOC-$i-$plugin"
  echo "[s16-b2] $(date '+%T') start $run"
  S15_KINDS_FILE="$KINDS" S12_SPACE="${S15_B2_SPACE:-S15 B2 Sweep}" S12_KIND_INDEX=$i S15_LOCALE=$([ "$LOC" = de ] && echo de-DE || echo en-US) S15_PROFILE_DIR="$P/$TAG-$LOC" S15_CONSOLE_ALL=1 S15_CONSOLE_FILTER="hub program|loaded from|stale|refused|error" \
    bun s16-hub-journey.mjs "$ORIGIN" "$run" "$plugin" none > "$L/journey-$run.txt" 2>&1
  echo "[s16-b2] $(date '+%T') done $run rc=$? $(/usr/bin/grep -o 'createArtifact → [^"]*' "$L/journey-$run.txt" | head -1) | $(/usr/bin/grep -o '"verb":"[^"]*","detail":[^,]*,"edits":\[[^]]*\]' "$L/journey-$run.txt" | head -1)"
done
echo "[s16-b2] $(date '+%T') finished $TAG $LOC"
