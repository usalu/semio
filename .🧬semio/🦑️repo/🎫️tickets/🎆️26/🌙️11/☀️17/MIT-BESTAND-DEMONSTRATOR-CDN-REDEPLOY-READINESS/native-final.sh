#!/usr/bin/env bash
# Verify successful queue inputs, prepare, build and test the demonstrator release.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; ROOT="$(cd "$T/../../../../../../.." && pwd)"; M="$ROOT/.cache/b/p/m/browser/release/modules"
repo tools exec --light python -- -I "$T/release-receipts.py" verify "$M" "$G/confirmed" || exit 1
repo tools exec --light python -- -I "$T/release-sessions.py" || exit 2
FORCE=1 "$T/native-lanes.sh" step browser-support "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript" repo ./📜️script.ts support || exit 2
rm -f "$G/done/demonstrator-prepare"
"$T/native-lanes.sh" step demonstrator-prepare "♻️mit-bestand/🧺️demonstrator" repo ./🔨️modules/🧩️runtime/📜️script.ts prepare || exit 2
exec bash "$T/native-site-verify.sh"
