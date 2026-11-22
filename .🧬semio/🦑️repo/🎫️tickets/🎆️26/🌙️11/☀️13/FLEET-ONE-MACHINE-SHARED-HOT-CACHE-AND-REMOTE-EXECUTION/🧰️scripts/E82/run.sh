#!/bin/zsh
W="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122"
cd /Users/ueli/Documents/semio
name=$1
out="$W/🗑️generated/E82/$name.txt"
echo "start $(date +%T) load $(uptime | sed 's/.*load averages: //')" > "$out"
perl -MTime::HiRes=time -e '$|=1;$s=time;while(<STDIN>){printf("%8.1f %s",time-$s,$_)}' < <(SEMIO_REPO_PROFILE=1 repo run repo:test-cache-reset --dry-run --exclude-dependencies 2>&1) >> "$out"
echo "end $(date +%T) load $(uptime | sed 's/.*load averages: //')" >> "$out"
