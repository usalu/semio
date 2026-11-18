#!/bin/bash
. "$(dirname "$0")/humansa-env.sh"
slug="$1"; shift
sc="${1:-500,200,100,50}"
[ "$slug" = kneeling-gardener ] && sc=100,50
rm -f "$G/humansa-ops-$slug.json"
"$B" "$N/humansa-auto.ts" "$slug" "$G/humansa-ops-$slug.json" "$sc" > "$G/humansa-auto-$slug.log" 2>&1
