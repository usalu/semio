#!/bin/zsh
# ⏱️ ST2 lane hold body: runs one command (its own process group, nice 15) inside a fleet-mutex hold and ends it at the
# deadline — SIGINT to the whole group (cargo forwards it to rustc), SIGKILL 20 s later — so an overlay hold never exceeds
# its budget; a cut cargo resumes from its completed units on the next hold (private build-dir).
# The command leads its own process group (python `setpgrp` + exec; zsh job control is off without a tty).
# usage (from st2-lane.zsh only): zsh st2-hold.zsh <deadline-minutes> <command…>
setopt no_bg_nice
minutes="$1"; shift
echo "[st2-hold] START $(date '+%F %T') deadline ${minutes}m: $*"
nice -n 15 python3 -c 'import os, sys; os.setpgrp(); os.execvp(sys.argv[1], sys.argv[1:])' "$@" &
pid=$!
deadline=$(( $(date +%s) + minutes * 60 ))
while kill -0 $pid 2>/dev/null; do
  if [ $(date +%s) -ge $deadline ]; then
    echo "[st2-hold] DEADLINE $(date '+%F %T') — SIGINT group $pid"
    kill -INT -- -$pid 2>/dev/null
    sleep 20
    kill -0 $pid 2>/dev/null && kill -KILL -- -$pid 2>/dev/null
    break
  fi
  sleep 5
done
wait $pid
rc=$?
echo "[st2-hold] rc=$rc $(date '+%F %T')"
exit $rc
