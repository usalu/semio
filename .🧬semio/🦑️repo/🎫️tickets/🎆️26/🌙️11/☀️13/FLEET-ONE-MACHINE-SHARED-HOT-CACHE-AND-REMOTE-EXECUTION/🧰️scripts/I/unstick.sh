#!/bin/zsh
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🐕️stall"
end=$(( $(date +%s) + ${1:-170} ))
while [ $(date +%s) -lt $end ]; do
  for p in $(pgrep -f 'bun install --frozen-lockfile'); do
    et=$(ps -o etime= -p $p | tr -d ' '); cpu=$(ps -o pcpu= -p $p | cut -d. -f1 | tr -d ' ')
    dir=$(lsof -p $p 2>/dev/null | awk '$4=="cwd"{ $1=$2=$3=$4=$5=$6=$7=$8=""; print }' | sed 's/^ *//')
    case "$dir" in */.cache/b/d/m/j/*)
      s=$(echo $et | awk -F: '{ if (NF==2) print $1*60+$2; else print 999 }')
      if [ "$s" -ge 40 ] && [ "${cpu:-0}" -ge 90 ]; then echo "$(date +%T) unstick $p $dir"; (cd "$D" && $B -e "import {releaseInstallerBindingsV1} from './🟦️.ts';console.log(releaseInstallerBindingsV1(process.argv[1]))" "$dir"); fi;;
    esac
  done
  sleep 5
done
