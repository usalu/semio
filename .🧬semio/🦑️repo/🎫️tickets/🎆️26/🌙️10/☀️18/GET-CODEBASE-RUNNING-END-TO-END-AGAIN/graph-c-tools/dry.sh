#!/bin/sh
# usage: dry.sh <name> <isolate on|off>   the 8-project bazel typecheck dry-run of graph-b (many.sh m8): graph load + planning only
cd /Users/ueli/Documents/semio || exit 1
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/graph-c"
N="$1"; I="$2"; S=$(date +%s)
SEMIO_BUILD_GRAPH_ISOLATE="$I" SEMIO_GRAPH_PHASES=1 nice -n 10 bun repo --tool=bazel run-many --targets=typecheck --projects=@semio-tech/framework-os,@semio-tech/s-dev-composition,@semio-tech/repo-lib,@semio-tech/pets,@semio-tech/pets-react,@semio-tech/ui-react,@semio-tech/framework-renderer-react,@semio-tech/course-react --parallel=1 --dry-run 2>&1 | perl -MPOSIX -ne 'BEGIN{$|=1} print strftime("%H:%M:%S ",localtime).$_' > "$G/$N.err"
echo "name=$N rc=$? wall=$(( $(date +%s)-S ))s" > "$G/$N.time"
