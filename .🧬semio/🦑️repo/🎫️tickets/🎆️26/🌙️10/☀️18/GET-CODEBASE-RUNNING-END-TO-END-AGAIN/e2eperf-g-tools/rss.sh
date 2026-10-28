#!/bin/sh
# runs a command and prints its peak RSS (macOS /usr/bin/time -l)
/usr/bin/time -l "$@" 2>&1 | awk '/maximum resident set size/ {printf "PEAK_RSS_MB %d\n", $1/1048576} !/maximum resident|page reclaims|page faults|swaps|block|messages|signals|context|involuntary|voluntary/ {print}'
