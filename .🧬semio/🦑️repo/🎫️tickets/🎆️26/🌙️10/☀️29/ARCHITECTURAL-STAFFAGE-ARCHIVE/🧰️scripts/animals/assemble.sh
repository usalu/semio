#!/usr/bin/env bash
# [scratch] concatenates the handcrafted fragments of one asset (and optionally the category) in file-name order
set -euo pipefail
D="$(cd "$(dirname "$0")" && pwd)"
asset="$1"
cat "$D/$asset"/[0-9]*.json
