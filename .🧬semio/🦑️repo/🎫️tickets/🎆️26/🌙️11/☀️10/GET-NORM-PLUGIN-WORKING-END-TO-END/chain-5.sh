#!/usr/bin/env bash
# ⛓️ Wait for chain 4, then rebuild norm with the new VDI 3805 fail assets and probe the four base routes.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
while pgrep -f chain-4.sh > /dev/null; do sleep 60; done
"$T/rebuild-and-probe.sh" base norm din4108 din16798 iso16757 vdi3805
