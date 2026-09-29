#!/bin/zsh
# ⛓️ LW1: runs law blocks one after another (each its own native-lane hold via lw1-law.sh); before each block waits while L1 is
# writing a train (`l1-run.sh` alive) so no block compiles a half-landed round. A FLEET_TICKET_STAMP holds for the first block only. usage: zsh lw1-chain.sh <capture>:<script> …
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-lw1
for pair in "$@"; do
  cap="${pair%%:*}"; script="${pair#*:}"
  while pgrep -f "wp-l1/l1-run.sh" > /dev/null; do sleep 20; done
  zsh "$W/lw1-law.sh" "$cap" /Users/ueli/Documents/semio -- zsh "$W/$script"
  unset FLEET_TICKET_STAMP
done
