#!/bin/zsh
# usage: wait.sh [seconds<=570] [tail lines]  blocks until the newest runner window stops/ends or the timeout
cd /Users/ueli/Documents/semio || exit 1
W=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/cargo-fix/window"
t=0; max=${1:-570}
done_() { [ -e $W/STOPPED ] && return 0; local s; s=$(grep -n 'window start' $W/runner.log | tail -1 | cut -d: -f1); [ -n "$s" ] && tail -n +$s $W/runner.log | grep -q 'window end'; }
until done_ || [ $t -ge $max ]; do sleep 10; t=$((t+10)); done
tail -${2:-6} $W/runner.log | cut -c1-260; cat $W/STOPPED 2>/dev/null; uptime
