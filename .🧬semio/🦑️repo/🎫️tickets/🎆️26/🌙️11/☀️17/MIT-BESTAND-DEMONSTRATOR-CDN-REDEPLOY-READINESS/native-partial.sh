#!/usr/bin/env bash
# Regenerates the catalog and runs the session and lane preparation steps for the given variants. Usage: native-partial.sh <variant>...
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
rm -f "$G/done/catalog-generate"; "$T/native-lanes.sh" catalog || exit 2
rc=0
for v in "$@"; do "$T/native-lanes.sh" session "$v" || rc=1; done
for v in "$@"; do "$T/native-lanes.sh" prepare "$v" || rc=1; done
exit $rc
