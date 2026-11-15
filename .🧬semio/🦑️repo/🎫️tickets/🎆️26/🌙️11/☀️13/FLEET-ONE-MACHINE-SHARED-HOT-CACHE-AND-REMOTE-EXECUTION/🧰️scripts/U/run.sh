#!/bin/zsh
# usage: run.sh label cmd...
label=$1; shift
cd /Users/Shared/semio
unset SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_TOKEN SEMIO_REMOTE_CACHE_ACCESS SEMIO_REMOTE_CACHE_INSTANCE
D=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/U
SEMIO_REPO_PROFILE=1 /usr/bin/time -l -o $D/$label.time "$@" >$D/$label.out 2>$D/$label.err &
tp=$!
sleep 0.5
rp=$(pgrep -P $tp | head -1); [ -z "$rp" ] && rp=$tp
python3 $D/mon.py $rp $D/$label.mon &
python3 $D/cat.py $rp $D/$label.cat &
wait $tp
echo rc=$? >> $D/$label.time
wait
