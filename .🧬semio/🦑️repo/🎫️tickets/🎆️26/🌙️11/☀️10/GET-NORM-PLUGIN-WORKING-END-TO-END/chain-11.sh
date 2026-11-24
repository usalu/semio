#!/usr/bin/env bash
# ⛓️ After chain 10, rebuild norm-eurocode-materials with the EN 1999 engine and fixture fixes and probe the three materials routes.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
while pgrep -f chain-10.sh > /dev/null; do sleep 60; done
while pgrep -f build-play-components > /dev/null; do sleep 60; done
REBUILD_ATTEMPTS=24 REBUILD_PAUSE_SECONDS=300 "$T/rebuild-and-probe.sh" materials11 norm-eurocode-materials en1995 en1996 en1999
