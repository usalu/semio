#!/bin/sh
# usage: web-ui-vitest.sh <comma separated ui-relative test paths>; one run at a time through the shared lock
D="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$D/🗑️generated"
WEB_UI_INCLUDE="$1" COMPOSITOR_CONFIG="$D/web-ui-vitest.config.ts" lockf -k /private/tmp/claude-501/semio-vitest.lock sh "$D/compositor-vitest.sh" 2>&1 | tail -${2:-60}
