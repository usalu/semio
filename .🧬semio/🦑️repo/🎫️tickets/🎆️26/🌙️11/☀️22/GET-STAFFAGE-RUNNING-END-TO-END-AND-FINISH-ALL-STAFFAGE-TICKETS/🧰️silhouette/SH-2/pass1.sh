#!/bin/zsh
# 🔁️ First pass of the greedy silhouette repair of SH-2 (2026-11-23): every person at every scale whose guard fails in the baseline `gate0`, in the order of the batch file; the result of each person is measured right after into `gate1`.
source "${0:A:h}/env.sh"; cd "$W"
export MAXSEC=${MAXSEC:-150}
run() { ./fix.sh "$@"; ./gate.sh gate1 "$1"; }
run standing-adult 50 100 200 500
run person-taking-selfie 50
run person-with-guide-dog 50
run person-with-rollator 50 100 200
run person-with-umbrella 50 100
run person-with-white-cane 50 100 500
run photographer-with-camera 50
run pregnant-person 50 100
run runner-with-blade-prosthesis 50 100 200
run running-child 50 500
run seated-guitarist 50 100
run seated-office-worker 50 100 200 500
run seated-reader 100
run skateboarder 50 100 200
run sleeping-person 50
run standing-phone-user 50 100
run standing-violinist 100
run tennis-player 50 500
run tour-guide 50 100
run walking-commuter 50 100
run wheelchair-user 50 200
run worker-carrying-box 50
run worker-with-pallet-jack 50
echo PASS1-DONE
