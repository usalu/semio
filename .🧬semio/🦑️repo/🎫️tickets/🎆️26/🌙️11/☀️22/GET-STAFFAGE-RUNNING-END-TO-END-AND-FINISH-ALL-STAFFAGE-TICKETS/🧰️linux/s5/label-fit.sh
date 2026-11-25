#!/usr/bin/env bash
# 🏷️ Runs the site's `label-fit` case (Chromium, both languages, 320, 768 and 1440 px) under a capacity grant; screenshots and findings go to `🗑️generated/linux-s5/<name>`. Usage: label-fit.sh <name> [release|prototype|http://running-site/]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
NAME="$1" SITE="${2:-release}"
OUT="$HERE/../../🗑️generated/linux-s5/$NAME"
mkdir -p "$OUT"
case "$SITE" in
  http*) export TEACHING_ARCHITECTURE_STAFFAGE_URL="$SITE" ;;
  *) export SEMIO_BUILD_MODE="$SITE" ;;
esac
TEACHING_ARCHITECTURE_STAFFAGE_LABEL_FIT_OUT="$OUT" SEMIO_TEST_LEVEL=long TEST_MEMORY_MB="${TEST_MEMORY_MB:-2500}" "$HERE/../s1/site-test.sh" label-fit
