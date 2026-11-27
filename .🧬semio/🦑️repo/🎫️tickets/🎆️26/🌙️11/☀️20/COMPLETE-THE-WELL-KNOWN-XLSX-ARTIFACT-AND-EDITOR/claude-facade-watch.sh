#!/bin/zsh
# [DEBUG] polls the job-context facade sources every 5 minutes and checks the facade after 10 quiet minutes
T=${0:A:h}; G="$T/🗑️generated/claude-test-support"; J=/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/⏱️context
start=$(date +%s)
newest() { find "$J" -name '*.rs' -not -path '*/🧪️tests/*' -exec stat -f %m {} + | sort -n | tail -1; }
while true; do
  age=$(( $(date +%s) - $(newest) ))
  echo "$(date +%T) facade sources unchanged for ${age}s" >> $G/watch.txt
  if [ $age -ge 600 ]; then
    MIN_DISK_GIB=20 $T/claude-resource-guard.sh $T/claude-test-support-cargo.sh -p semio-framework-job-context --lib > $G/facade.txt 2>&1; code=$?
    echo "facade check exit $code at $(date +%T)" >> $G/watch.txt
    [ $code -eq 0 ] && { echo PASS >> $G/watch.txt; exit 0; }
    [ $(( $(date +%s) - $start )) -ge 1800 ] && { echo "STALLED-FAILING" >> $G/watch.txt; exit 1; }
  fi
  sleep 300
done
