#!/usr/bin/env bash
# Runs strict static acceptance on the last built site. Usage: accept-site.sh [port]
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"
exec repo tools exec --light python -- -I "$T/accept-site.py" --port "${1:-0}"
