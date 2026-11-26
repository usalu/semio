#!/bin/zsh
# 🌿️ `weeping-willow` under one grant (`grant.sh 2000 <timeout> chain-w.sh`): chain E (write, compare, prove; the strands `mvt-*` are what `visible.ts` looks at), then its matrix.
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
PREFIX=mvt- zsh "$W/chain-e.sh" weeping-willow; zsh "$W/chain-s.sh" weeping-willow 21600
