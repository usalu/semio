#!/bin/sh
export LC_ALL=en_US.UTF-8
ps -axo pid,ppid,etime,%cpu,rss,stat,command | awk -v pat="$1" 'NR==1 || $0 ~ pat' | cut -c1-${2:-330}
