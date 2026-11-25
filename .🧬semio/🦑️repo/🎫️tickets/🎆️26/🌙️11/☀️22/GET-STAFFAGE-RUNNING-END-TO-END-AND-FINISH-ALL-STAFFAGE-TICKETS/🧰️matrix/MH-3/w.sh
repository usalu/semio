#!/bin/zsh
# ⏳️ Blocks at most 170 s until one of the chain outputs of the fourth fixer in $GEN/t4 grows by a line that is no queue line, then prints the new lines of all of them (each call prints what the last one did not): w.sh
d="${0:A:h}/../../🗑️generated/matrix/MH-3/t4"; end=$(( SECONDS + ${1:-170} ))
while :; do
  new=0
  for f in "$d"/*.out; do
    n=$(grep -vc '^\[capacity\] waiting' "$f"); o=$(cat "$f.seen" 2>/dev/null || echo 0)
    if (( n > o )); then new=1; echo "## ${f:t} $(date +%H:%M)"; grep -v '^\[capacity\] waiting' "$f" | tail -n +$(( o + 1 )) | cut -c1-${WIDTH:-230} | grep -v 'page \|swaps\|block \|messages\|signals\|shared\|unshared\|context\|instructions\|cycles\|footprint'; echo $n > "$f.seen"; fi
  done
  (( new || SECONDS > end )) && break
  sleep 5
done
for f in "$d"/*.out; do tail -1 "$f" | grep '^\[capacity\] waiting' | sed -E "s|^.capacity. waiting ([0-9]+s).*, ([0-9]+) ahead.*|${f:t} waits \1, \2 ahead|" | cut -c1-120; done
