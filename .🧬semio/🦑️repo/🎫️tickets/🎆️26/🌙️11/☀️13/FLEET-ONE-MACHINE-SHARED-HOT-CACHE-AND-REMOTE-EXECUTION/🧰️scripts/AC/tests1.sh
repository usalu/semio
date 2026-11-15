#!/bin/zsh
. /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/AC/env.sh
L="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G="$L/🕸️graph/🕰️generations"
run(){ d="$1"; f="$2"; ( cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test ./$f --timeout 180000 2>&1 | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)|error:" | tr '\n' ' '; echo " <- ${d#$L/}/$f" ); }
run "$L/🪪️graph-state/📍️cursor/🧪️tests" "🟦️.ts"
run "$G/🧪️tests/🔎️doubt" "🟦️.ts"
run "$G/🧪️tests/🌊️stream-patch" "🟦️.ts"
run "$G/🧪️tests/🌊️patch-drift" "🟦️.ts"
run "$G/🧪️tests/🛡️stamp-soundness" "🟦️.ts"
run "$G/🧪️tests" "🟦️.ts"
run "$G/🌐️remote-exchange/🧪️tests" "🟦️.ts"
run "$G/🪞️plan-once" "🟦️.ts" 2>/dev/null
run "$G/🧪️tests/🪞️plan-once" "🟦️.ts"
run "$G/🔌️load/🧪️tests" "🟦️.ts" 2>/dev/null
run "$L/🏡️resident/🕰️graph-hooks/🧪️tests" "🟦️.ts"
run "$L/🏡️resident/🕰️graph-hooks/🧪️tests/🗂️file-rows" "🟦️.ts"
