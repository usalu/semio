#!/usr/bin/env bash
# 🧱️ Ticket tool of work package R: (re)starts or stops the private stacks of `wp_j_private_stack.ts` under
# `🗑️generated/wp-r` — `dev` (proctor 8927, site 6197) or `rehearsal` (proctor 8928, the release build in
# `🗑️generated/wp-r/site-rehearsal` on 6198). It stops only the processes whose ids that tool wrote down. The dev
# server runs with TEACHING_ARCHITECTURE_QUIZ_WATCH=off, as in the gate: the stack serves the sources as they were when
# it first read them (restart to refresh).
# Usage (from the repository root): bash ".../wp_r_stack.sh" <restart|stop> [dev|rehearsal]
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
scratch="$ticket/🗑️generated/wp-r"
topology="${2:-dev}"
if [ "$topology" = "rehearsal" ]; then site=6198; proctor=8928; else site=6197; proctor=8927; fi
mkdir -p "$scratch"
if [ -f "$scratch/stack-$topology.pids" ]; then
  while read -r pid; do
    [ -n "$pid" ] && taskkill //PID "$pid" //T //F >/dev/null 2>&1
  done < "$scratch/stack-$topology.pids"
  rm -f "$scratch/stack-$topology.pids"
fi
[ "${1:-restart}" = "stop" ] && exit 0
TEACHING_ARCHITECTURE_QUIZ_WATCH=off WP_STACK_SCRATCH=wp-r WP_STACK_SITE_PORT=$site WP_STACK_PROCTOR_PORT=$proctor nohup bun "$ticket/wp_j_private_stack.ts" "$topology" > "$scratch/stack-$topology.out" 2>&1 &
for _ in $(seq 1 240); do
  if curl -s -o /dev/null "http://127.0.0.1:$site/" && curl -s -o /dev/null "http://127.0.0.1:$proctor/"; then
    echo "stack up: site $site, proctor $proctor"
    exit 0
  fi
  sleep 0.5
done
echo "stack did not come up"
exit 1
