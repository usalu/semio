#!/bin/bash
# 🧰 Step 0 of round 16: runs `repo tools setup all` once from the worktree root into ~/tools-setup-round-16.log, with one SIGINT when the dependency store wait stands for more than 2 minutes. Usage: tools-setup-round-16.sh
set -u
M="$(cd "$(dirname "$0")" && pwd)"; log="$HOME/tools-setup-round-16.log"; note="$M/round-17.tools-setup.txt"
export PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" SEMIO_LANGUAGE=en
cd "$HOME/Documents/semio" || exit 1
start="$(date +%s)"; echo "start $(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$note"
set -m; ( repo tools setup all 2>&1 | tee "$log"; exit "${PIPESTATUS[0]}" ) & pid=$!; set +m
waits=0
while kill -0 "$pid" 2>/dev/null; do
  sleep 10
  if tail -5 "$log" 2>/dev/null | grep -q 'Another process holds the dependency store'; then waits=$((waits+1)); else waits=0; fi
  if [ "$waits" -ge 13 ]; then
    { echo "## dependency store wait for more than 2 minutes at $(date -u +%Y-%m-%dT%H:%M:%SZ)"; grep 'Another process holds the dependency store' "$log" | tail -1; ps -axo pid,ppid,etime,command | grep -E "bun|repo" | grep -v grep | cut -c1-400; echo "one SIGINT to process group $pid"; } >> "$note"
    kill -INT -- "-$pid" 2>/dev/null; break
  fi
done
wait "$pid"; rc=$?
echo "end $(date -u +%Y-%m-%dT%H:%M:%SZ) exit_code=$rc wall_seconds=$(( $(date +%s) - start ))" >> "$note"
