#!/bin/zsh
prev=""
while true; do
  cur=""
  for p in ${(f)"$(ps -axo pid=,command= | awk '$2 ~ /\/bin\/cargo$/ && ($3=="build"||$3=="check"||$3=="test"||$3=="rustc"||$3=="nextest") {print $1}')"}; do
    [ -z "$p" ] && continue
    k=$(pgrep -P "$p" 2>/dev/null | wc -l | tr -d ' ')
    if [ "$k" = "0" ] && sample "$p" 1 2>/dev/null | /usr/bin/grep -qE 'prebuild_lock_exclusive|open_rw_exclusive'; then cur="$cur $p"; fi
  done
  both=""
  for p in ${=cur}; do [[ " $prev " == *" $p "* ]] && both="$both $p"; done
  if [ -n "$both" ]; then date; for p in ${=both}; do ps -o pid=,ppid=,etime=,command= -p "$p" | cut -c1-220; done; exit 0; fi
  prev="$cur"
  sleep 300
done
