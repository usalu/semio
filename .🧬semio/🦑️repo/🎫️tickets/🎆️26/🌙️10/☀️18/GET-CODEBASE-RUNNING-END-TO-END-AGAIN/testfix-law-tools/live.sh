#!/usr/bin/env bash
# After GO: python3 apply.py (dry run), python3 apply.py --write, then this script runs the edited suites live (2 jobs) into ../🗑️generated/testfix-law/live.jsonl.
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$T/../🗑️generated/testfix-law"
LIVE=1 CONC=2 python3 "$T/run.py" "$T/suites-edited.txt" "$T/../🗑️generated/testfix-law/live.jsonl" 300
