#!/bin/zsh
# 🔁️ Second pass of the greedy silhouette repair of SH-2 (2026-11-23, with the tool that measures fresh copies and keeps an elevation change only when front and plan stay as good): every person and scale that still fails after `pass1.sh`; the result of each person is measured right after into `gate2`.
source "${0:A:h}/env.sh"; cd "$W"
export MAXSEC=${MAXSEC:-240}
run() { ./fix.sh "$@"; ./gate.sh gate2 "$1"; }
./gate.sh gate2 sleeping-person
run walking-commuter 50 100
run standing-adult 500
run seated-guitarist 50
run standing-phone-user 50
run pregnant-person 50
run runner-with-blade-prosthesis 50
run tour-guide 50
run skateboarder 50
run person-with-umbrella 50 100
run person-with-rollator 50 100
run person-with-white-cane 50 100
run seated-office-worker 50 200 500
run wheelchair-user 50 200
run worker-carrying-box 50
MAXSEC=150 run standing-violinist 100
MAXSEC=150 run worker-with-pallet-jack 50
MAXSEC=150 run person-taking-selfie 50
MAXSEC=150 run person-with-guide-dog 50
echo PASS2-DONE
