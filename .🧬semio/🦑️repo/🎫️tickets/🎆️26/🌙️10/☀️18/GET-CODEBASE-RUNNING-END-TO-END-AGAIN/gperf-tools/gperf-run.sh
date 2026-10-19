#!/bin/bash
# usage: gperf-run.sh LOG cmd...
log="$1"; shift
s=$(date +%s)
echo "$(date +%T) START $*" > "$log"
( "$@" 2>&1 ) | perl -MPOSIX -MTime::HiRes=time -e '$|=1; while(<STDIN>){ print POSIX::strftime("%H:%M:%S ",localtime), $_ }' >> "$log"
echo "$(date +%T) END elapsed=$(( $(date +%s) - s ))s" >> "$log"
