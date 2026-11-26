#!/bin/zsh
# 🧮️ The matrix of several assets of batch MV-3 one after the other under one grant (`grant.sh <MB> <timeout> chain-m.sh <id>…`), each as `chain-s.sh` runs it (limit LIMIT seconds per asset, default 14400).
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
for a in "$@"; do zsh "$W/chain-s.sh" "$a" "${LIMIT:-14400}"; done
