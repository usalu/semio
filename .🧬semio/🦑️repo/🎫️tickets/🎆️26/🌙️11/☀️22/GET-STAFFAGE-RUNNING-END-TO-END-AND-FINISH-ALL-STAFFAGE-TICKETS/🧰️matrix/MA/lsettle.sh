#!/bin/zsh
# ⚖️ `lsettle.py` with the paths of the batch, made to be started through the memory queue (800 MB): lsettle.sh <id>…
BATCH=MA source "${0:A:h}/../MR/env.sh"; export B S T M GEN
python3 "$W/lsettle.py" "$@"
