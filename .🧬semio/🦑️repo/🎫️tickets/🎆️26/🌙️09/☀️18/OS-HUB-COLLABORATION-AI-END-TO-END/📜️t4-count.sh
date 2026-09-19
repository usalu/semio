#!/usr/bin/env bash
# 🧮️ Counts T4-owned diagnostics (os product, ✏️s plugins, root build script) in a tsc capture.
set -euo pipefail
f="$1"
grep "error TS" "$f" | sed 's/(.*//' | awk '
/^🧰️framework\/🛍️products\/💻️os/ { os++; next }
/^✏️s/ { s++; next }
/^📜️script\.ts/ { sc++; next }
{ other++ }
END { printf "os=%d s=%d script=%d owned=%d other=%d\n", os, s, sc, os+s+sc, other }'
