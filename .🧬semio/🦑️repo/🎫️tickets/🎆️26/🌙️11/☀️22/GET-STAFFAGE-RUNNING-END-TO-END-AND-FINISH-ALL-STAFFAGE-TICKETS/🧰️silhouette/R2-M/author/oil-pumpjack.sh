#!/bin/zsh
# 🛢️ Repair ops of `oil-pumpjack` of the second round (after the ops of SAM): the hand repairs (oil-pumpjack.py), then drawn shapes that were fuller than their solids are redrawn as the solids project. Front elevation of every scale: the skid (two beams, cross members and deck with the open bays between them instead of one block); 1:50 front: motor feet, motor and the two pairs of legs; 1:50 and 1:100 front: the feet of the four legs where they pass the clearance between cross members and deck; side elevation: wellhead top (1:100), stuffing box and horsehead (1:50).
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 oil-pumpjack.py && cd "$W"
t() { "$B" trace.ts oil-pumpjack "$@" | tail -1 }
for s in 50 100 200; do t $s front "skid-beam-*,skid-cross-*,skid-deck" --id skid --bone skid --like skid --replace skid --holes; done
t 500 front "skid-beam-*,skid-deck" --id skid --bone skid --like skid --replace skid --holes
t 50 front "motor-feet,motor-foot-*" --id motor-feet --bone motor --like motor-feet --replace motor-feet
t 50 front "motor,motor-endshield" --id motor --bone motor --like motor --replace motor
t 50 front "leg-fl,leg-rl" --id leg-left --bone post --like leg-left --replace leg-left
t 50 front "leg-fr,leg-rr" --id leg-right --bone post --like leg-right --replace leg-right
for s in 50 100; do "$B" patch.ts oil-pumpjack $s front --only leg-fl,leg-rl,leg-fr,leg-rr --thin 0.004 --min 0.2 --grow 0.001 | tail -1; done
t 100 elevation wellhead-top --id wellhead-top --bone base --like wellhead-top --replace wellhead-top
t 50 elevation stuffing-box --id stuffing-box --bone base --like stuffing-box --replace stuffing-box
t 50 elevation "horsehead,horsehead-face" --id horsehead --bone horsehead --like horsehead --replace horsehead
"$B" drawable.ts oil-pumpjack --ops ops/oil-pumpjack.json
