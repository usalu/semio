#!/bin/sh
# Applies the staged fixes that sit under 🏗️build/ and 🏃️process/ (hold until the coordinator announces chain17 green):
#   1. source-demand test + graph fixture (stage-source-demand.py --apply, re-derived from the live files, asserts on drift)
#   2. mode 0644 for the 80 appliers' outputs left at 0600 under those trees
set -e
D=$(cd "$(dirname "$0")" && pwd)
python3 -I "$D/stage-source-demand.py" --apply
python3 -I "$D/chmod-held.py"
