#!/bin/zsh
# 🚰️ Waits until no process of this fixer still queues on the first lock file (the one inside the shared worktree, replaced by the lock in the temp folder): drain.sh
until ! pgrep -f "matrix/MH-5/[.]lock" > /dev/null; do sleep 10; done
