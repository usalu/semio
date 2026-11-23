#!/usr/bin/env bash
# ⛓️ Wait for chain 2, then rebuild norm-energy and probe DIN V 18599.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
while pgrep -f chain-2.sh > /dev/null; do sleep 30; done
"$T/rebuild-and-probe.sh" energy norm-energy din18599
