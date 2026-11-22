#!/bin/zsh
W="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122"
cd /Users/ueli/Documents/semio
name=$1
perl -MTime::HiRes=time -e '$|=1;$s=time;while(<STDIN>){printf("%8.1f %s",time-$s,$_)}' < <(repo tools setup nx 2>&1) > "$W/🗑️generated/E22/$name.txt"
echo "$name total $(tail -1 "$W/🗑️generated/E22/$name.txt" | cut -c1-9) gens $(ls .cache/b/t/m/nx | wc -l) load $(uptime | sed 's/.*load averages: //')"
python3 "$W/../🧰️scripts/E22/phases.py" "$W/🗑️generated/E22/$name.txt" | awk '/nx:|namespace/ {printf "%s ", $2 "s:" $3}'; echo
