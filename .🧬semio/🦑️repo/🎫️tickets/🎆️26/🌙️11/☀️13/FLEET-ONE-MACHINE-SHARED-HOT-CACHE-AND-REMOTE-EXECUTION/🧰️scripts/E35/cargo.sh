#!/bin/zsh
W="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122"
cd /Users/ueli/Documents/semio
name=$1
echo "start $(date +%T) load $(uptime | sed 's/.*load averages: //')" > "$W/🗑️generated/E35/$name.txt"
perl -MTime::HiRes=time -e '$|=1;$s=time;while(<STDIN>){printf("%8.1f %s",time-$s,substr($_,0,400)."\n")}' < <(SEMIO_CARGO_TIMING=1 repo cargo --store=shared --configuration=prototype -- metadata --format-version=1 --no-deps --offline 2>&1 >/dev/null) >> "$W/🗑️generated/E35/$name.txt"
echo "end $(date +%T) load $(uptime | sed 's/.*load averages: //')" >> "$W/🗑️generated/E35/$name.txt"
