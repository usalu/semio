#!/usr/bin/env bash
# Packages a verified site. Usage: package-site.sh <site-folder> <output-dir> <accepted-manifest>
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"
exec repo tools exec --light python -- -I "$T/package-site.py" "$@"
