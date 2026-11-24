#!/bin/zsh
# 4️⃣ Fourth pass of the batch: whole views redrawn as plates where a coarse scale misses the area (IoU), the two forearms of the signer in one step, then the greedy repair with pairs of limbs where the third pass stalled.
source "${0:A:h}/env.sh"
step() { print -r -- "$3" | "$W/try.sh" "$1" "$2" && "$W/keep.sh" "$1"; }
step person-stretching 500 'plate|plan|pelvis/*'
step person-signing 50 $'plate|elevation|forearm-l\nplate|elevation|forearm-r'
step cyclist 500 'plate|elevation|frame/*'
step person-carrying-ladder 200 'plate|elevation|pelvis/*'
step person-carrying-ladder 500 'plate|elevation|pelvis/*'
export ANYBONE=1
while read -r a scales; do "$W/fix.sh" "$a" ${=scales}; done <<'LIST'
person-signing 50
person-carrying-shopping-bags 50
person-cooking 50
person-eating-standing 50
painter-at-easel 50
person-pushing-bicycle 50
cyclist 50
LIST
