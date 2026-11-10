#!/usr/bin/env bash
# Private-workspace snapshot (copies: hard links were tried and refused by the build, "Bounded physical Source revision required", because a source file must have one link) of the owners lane (about 2 minutes, about 10 GB below /home/deck/pw): the sources of the live checkout, the first-party node_modules links pointed at the copy, and the fact index and generation store as a snapshot that owners-run.sh restores before every run. Nothing is linked to the live store.
set -euo pipefail
LIVE=/home/deck/semio
R=${OWNERS_ROOT:-/home/deck/pw/o1}
HERE="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$R/.cache/b/g" "$(dirname "$R")/snap"
cd "$LIVE"
rsync -a --exclude=/.cache --exclude=/.git --exclude=/.🧬semio --exclude=/.venv --exclude=/.nx --exclude=/.pytest_cache --exclude=/node_modules "$LIVE"/ "$R"/
mkdir -p "$R/.🧬semio"
rm -rf "$(dirname "$R")/snap/m"
cp -a "$LIVE/.cache/b/g/m" "$(dirname "$R")/snap/m"
rm -f "$(dirname "$R")/snap/m/format.json" "$(dirname "$R")"/snap/m/optional*
"$LIVE/.venv/bin/python" -I "$HERE/owners-modules.py" "$R"
du -sh "$R" "$(dirname "$R")/snap" | tail -2
