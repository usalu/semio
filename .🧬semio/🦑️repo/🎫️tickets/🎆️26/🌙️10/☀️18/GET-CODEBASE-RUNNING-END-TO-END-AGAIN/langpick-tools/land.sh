#!/bin/sh
# 🗣️ land.sh check|apply|revert [ROOT] [batch...]: the staged batches of lane langpick in land order (revert runs in reverse). Default batches: core copies imports dashboard routing (launcher and launcher2 are separate steps: they need a launcher rebuild).
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
verb=${1:?check|apply|revert}; shift
root=""; [ $# -gt 0 ] && [ -d "$1" ] && { root=$1; shift; }
batches=${*:-core copies imports dashboard routing}
[ "$verb" = revert ] && batches=$(echo $batches | tr ' ' '\n' | sed '1!G;h;$!d' | tr '\n' ' ')
for batch in $batches; do echo "== $batch"; python3 -I "$HERE/staged/$batch/stage.py" "$verb" $root; done
