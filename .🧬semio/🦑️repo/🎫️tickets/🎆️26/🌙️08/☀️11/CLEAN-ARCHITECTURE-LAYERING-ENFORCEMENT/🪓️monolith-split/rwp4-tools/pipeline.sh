#!/bin/zsh
# 🏭️ WP4 pipeline: plan -> S0 -> T1 -> external overlay, all in the sandbox (usage: pipeline.sh <tag>)
set -e
cd "$(dirname "$0")"
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad
TAG=$1
export RWP4_SRC=$SCR/orig-shell/🎯️targets/🧊️wgpu
export RWP4_EXT_BASE=$SCR/apply-state/ext-orig
python3 plan_t1.py > ../🗑️generated/rwp4-plan-$TAG.log
python3 s0.py build $SCR/s0-$TAG > ../🗑️generated/rwp4-s0-$TAG.log
python3 t1.py $SCR/s0-$TAG $SCR/t1-$TAG > ../🗑️generated/rwp4-t1-$TAG.log
rm -rf $SCR/ext-$TAG $SCR/ovc-$TAG
python3 carve_vocab.py $SCR/t1-$TAG $SCR/ovc-$TAG > ../🗑️generated/rwp4-vocab-$TAG.log
python3 ext_overlay.py $SCR/ext-$TAG > ../🗑️generated/rwp4-ext-$TAG.log
python3 verify_t1.py $SCR/t1-$TAG > ../🗑️generated/rwp4-verify-$TAG.log
python3 verify_calls.py $SCR/t1-$TAG $SCR/ext-$TAG >> ../🗑️generated/rwp4-verify-$TAG.log
tail -2 ../🗑️generated/rwp4-verify-$TAG.log
