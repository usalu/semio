#!/bin/zsh
# 🏁️ The last proofs of written assets inside the grant of its caller: the guard of the state before the repair (`before.sh`) and the real test files (`suites.sh`). final.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do zsh "$W/before.sh" "$a"; zsh "$W/suites.sh" "$a"; done
