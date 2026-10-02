#!/usr/bin/env bash
# 🧱️ Ticket tool of work package N: (re)starts or stops the private dev stack of `wp_j_private_stack.ts` under
# `🗑️generated/wp-n` (proctor 8921, site 6191). It stops only the processes whose ids that tool wrote down.
# Usage (from the repository root): bash ".../wp_n_stack.sh" <restart|stop>
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
scratch="$ticket/🗑️generated/wp-n"
mkdir -p "$scratch"
if [ -f "$scratch/stack-dev.pids" ]; then
  while read -r pid; do
    [ -n "$pid" ] && taskkill //PID "$pid" //T //F >/dev/null 2>&1
  done < "$scratch/stack-dev.pids"
  rm -f "$scratch/stack-dev.pids"
fi
[ "${1:-restart}" = "stop" ] && exit 0
WP_STACK_SCRATCH=wp-n nohup bun "$ticket/wp_j_private_stack.ts" dev > "$scratch/stack-dev.out" 2>&1 &
for _ in $(seq 1 120); do
  if curl -s -o /dev/null "http://127.0.0.1:6191/" && curl -s -o /dev/null "http://127.0.0.1:8921/"; then
    echo "stack up"
    exit 0
  fi
  sleep 0.5
done
echo "stack did not come up"
exit 1
