#!/usr/bin/env bash
# Verify successful queue inputs, prepare, build and test the demonstrator release.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; ROOT="$(cd "$T/../../../../../../.." && pwd)"; M="$ROOT/.cache/b/p/m/browser/release/modules"
python3 "$T/release-receipts.py" verify "$M" "$G/confirmed" || exit 1
rm -f "$G/done/demonstrator-prepare"
"$T/native-lanes.sh" step demonstrator-prepare "♻️mit-bestand/🧺️demonstrator" repo ./🔨️modules/🧩️runtime/📜️script.ts prepare || exit 2
VERIFY_QUEUE_RECEIPTS=1 exec bash "$T/native-site-verify.sh"
