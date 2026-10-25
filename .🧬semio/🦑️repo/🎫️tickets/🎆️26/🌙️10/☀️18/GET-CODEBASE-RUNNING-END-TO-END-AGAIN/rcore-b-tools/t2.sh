#!/bin/sh
# usage: t2.sh NAME cargo-args... : headless detached cargo (no watchdog)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
N=$1; shift
python3 $C/chain-tools/detach.py $C/🗑️generated/rcore-b/$N.log sh $C/rcore-b-tools/run.sh $C/🗑️generated/rcore-b/$N.out "$@"
