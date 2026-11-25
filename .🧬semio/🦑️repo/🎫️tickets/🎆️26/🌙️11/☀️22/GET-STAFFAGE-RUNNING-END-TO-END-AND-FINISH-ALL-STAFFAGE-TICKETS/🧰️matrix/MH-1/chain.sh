#!/bin/zsh
# ⛓️ Several steps of the 2500 MB class under ONE grant of the memory queue, one after the other (the queue is crowded: one admission costs up to an hour): chain.sh '<shell commands>' runs the commands in this folder inside the grant; the scripts of this folder are called with their INSIDE variable so they do not queue again, for example chain.sh 'INSIDE=1 ./solve.sh a b; INSIDE=light STEPS="measure tests" ./proof.sh c'   (CHAIN_MB and CHAIN_TIMEOUT change the reservation and the limit)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
exec "$W/q.sh" ${CHAIN_MB:-2500} ${CHAIN_TIMEOUT:-21600} "$W" /bin/zsh -c "$1"
