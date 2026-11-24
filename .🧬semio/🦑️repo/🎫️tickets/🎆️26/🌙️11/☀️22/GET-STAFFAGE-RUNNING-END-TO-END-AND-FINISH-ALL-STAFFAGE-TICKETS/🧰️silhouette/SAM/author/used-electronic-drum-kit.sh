#!/bin/zsh
# 🥁️ Repair ops of `used-electronic-drum-kit`, in order: the rims lose `slender` at 1:50 (used-electronic-drum-kit.py), then the snare pad, whose drawn outline stood 14 mm off its tilted solid in the side and front elevation, is drawn as its shell, rim and head project.
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 used-electronic-drum-kit.py && cd "$W"
for v in elevation front; do "$B" trace.ts used-electronic-drum-kit 50 $v snare-shell,snare-rim,snare-head --id snare --bone rack --like snare; done
