#!/bin/sh
# usage: t.sh NAME cargo-args... : detached cargo + watchdog, prints wrapper pid
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
N=$1; shift
P=$(python3 $C/chain-tools/detach.py $C/🗑️generated/rcore-b/$N.log sh $C/rcore-b-tools/run.sh $C/🗑️generated/rcore-b/$N.out "$@")
python3 $C/chain-tools/detach.py $C/🗑️generated/rcore-b/$N.dog sh $C/rcore-b-tools/dog.sh $P $C/🗑️generated/rcore-b/$N.out 'out/semio_framework_|deps/semio_framework_' 240 >/dev/null
echo $P
