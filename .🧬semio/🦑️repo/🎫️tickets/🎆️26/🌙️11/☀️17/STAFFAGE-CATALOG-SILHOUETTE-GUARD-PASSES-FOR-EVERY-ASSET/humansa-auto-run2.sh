#!/bin/bash
. "$(dirname "$0")/humansa-env.sh"
slug="$1"; sc="$2"
rm -f "$G/humansa-ops-$slug.json"
"$B" "$N/humansa-auto.ts" "$slug" "$G/humansa-ops-$slug.json" "$sc" > "$G/humansa-auto-$slug.log" 2>&1
