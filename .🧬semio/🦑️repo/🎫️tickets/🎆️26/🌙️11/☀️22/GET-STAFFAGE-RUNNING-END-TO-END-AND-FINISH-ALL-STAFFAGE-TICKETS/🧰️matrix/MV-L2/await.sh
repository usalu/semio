#!/bin/zsh
# ⏳️ Starts the last worker of round MV-L2 when a grant is free: `await.sh <megabytes>` waits until at most one worker of this round is left, then asks the queue for one grant of that size and runs `worker.sh` in it (the suites of a tree peak above 2500 MB: `🧊️three-views` of `norway-maple` 3026 MB).
source "${0:A:h}/env.sh"
until (( $(ps -axww -o command | tr -cd '[:print:]\n' | grep -c "^timeout 36000 zsh .*matrix/MV-L2/worker.sh") <= 1 )); do sleep 10; done
exec "$W/q.sh" "$1" "$W" 36000 zsh "$W/worker.sh" "$1"
