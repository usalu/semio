#!/bin/zsh
# usage: batch.sh <stage|apply> <tag> [plugin-dir…] : runs run.ts once per plugin group of /Users/ueli/.cache/mwu/all-artifacts.txt (sequential; log F/🗑️generated/mwu-batch-<tag>.log)
cd /Users/ueli/Documents/semio || exit 99
ulimit -n 65536
FD=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
mode=$1; tag=$2; shift 2
if [ $# -gt 0 ]; then plugins=("$@"); else plugins=("${(@f)$(cut -d/ -f1-3 /Users/ueli/.cache/mwu/all-artifacts.txt | sort -u)}"); fi
for p in "${plugins[@]}"; do
  arts=("${(@f)$(awk -v p="$p/" 'index($0,p)==1' /Users/ueli/.cache/mwu/all-artifacts.txt)}")
  echo "== $p (${#arts[@]} artifacts)" >> "$FD/🗑️generated/mwu-batch-$tag.log"
  nice -n 10 bun "$FD/mwu-tools/run.ts" $mode "$tag-$(basename "$p")" "${arts[@]}" >> "$FD/🗑️generated/mwu-batch-$tag.log" 2>&1
  echo "exit $?" >> "$FD/🗑️generated/mwu-batch-$tag.log"
done
echo ALLDONE >> "$FD/🗑️generated/mwu-batch-$tag.log"
