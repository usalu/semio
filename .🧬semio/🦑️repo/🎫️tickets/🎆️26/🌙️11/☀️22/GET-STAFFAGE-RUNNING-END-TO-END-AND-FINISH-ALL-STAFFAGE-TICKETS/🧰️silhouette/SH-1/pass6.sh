#!/bin/zsh
# 6️⃣ Sixth pass of the batch: the declared dimensions the bodies have, after the limbs whose drawing is wider or narrower than their body are drawn as plates (so that every drawn extent stays within the 15 mm of `validateAsset`).
source "${0:A:h}/env.sh"
step() { print -r -- "$3" | "$W/hand.sh" "$1" "$2" | sed "s/^/$1 1:$2 /"; }
arms() { local out=""; for v in ${=1}; do for b in ${=2}; do out+="plate|$v|$b-l"$'\n'"plate|$v|$b-r"$'\n'; done; done; print -rn -- "$out"; }
(cd "$W" && "$B" dimset.ts person-carrying-ladder depthM=0.511 && "$B" dimset.ts person-stretching widthM=0.702 && "$B" dimset.ts leaning-person depthM=0.536 && "$B" dimset.ts person-drinking-coffee depthM=0.556 && "$B" dimset.ts person-signing widthM=0.554 depthM=0.53)
for s in 100 200 500; do step person-carrying-ladder $s "$(arms 'front plan' 'upper-arm')"; done
step person-stretching 500 $'plate|elevation|pelvis/*\nplate|plan|pelvis/*'
step leaning-person 100 "$(arms front 'upper-arm forearm')"
step person-drinking-coffee 100 "$(arms front 'arm forearm')"
step person-drinking-coffee 500 "$(arms front 'arm forearm hand')"
step person-signing 100 "$(arms front 'arm forearm')"
step person-signing 200 "$(arms 'front plan' 'arm forearm')"$'\nplate|elevation|hand-l/*\nplate|elevation|hand-r/*\nplate|plan|hand-l/*\nplate|plan|hand-r/*'
step person-signing 500 "$(arms front 'arm forearm')"$'\nplate|elevation|hand-l/*\nplate|elevation|hand-r/*\nplate|plan|hand-l/*\nplate|plan|hand-r/*'
