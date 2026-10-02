#!/usr/bin/env bash
# 🧱️ Ticket tool of work package P: (re)starts or stops the private stacks of `wp_j_private_stack.ts` under
# `🗑️generated/wp-p` — `dev` (proctor 8923, site 6193) or `rehearsal` (proctor 8924, the release build in
# `🗑️generated/wp-p/site-rehearsal` on 6194). It stops only the processes whose ids that tool wrote down. The dev
# server runs with TEACHING_ARCHITECTURE_QUIZ_WATCH=off, as in the gate: an edit of somebody else never reloads a page
# under a running spec, and the stack serves the sources as they were when it first read them (restart to refresh).
# Usage (from the repository root): bash ".../wp_p_stack.sh" <restart|stop> [dev|rehearsal]
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
scratch="$ticket/🗑️generated/wp-p"
topology="${2:-dev}"
if [ "$topology" = "rehearsal" ]; then site=6194; proctor=8924; else site=6193; proctor=8923; fi
mkdir -p "$scratch"
if [ -f "$scratch/stack-$topology.pids" ]; then
  while read -r pid; do
    [ -n "$pid" ] && taskkill //PID "$pid" //T //F >/dev/null 2>&1
  done < "$scratch/stack-$topology.pids"
  rm -f "$scratch/stack-$topology.pids"
fi
[ "${1:-restart}" = "stop" ] && exit 0
TEACHING_ARCHITECTURE_QUIZ_WATCH=off WP_STACK_SCRATCH=wp-p WP_STACK_SITE_PORT=$site WP_STACK_PROCTOR_PORT=$proctor nohup bun "$ticket/wp_j_private_stack.ts" "$topology" > "$scratch/stack-$topology.out" 2>&1 &
for _ in $(seq 1 240); do
  if curl -s -o /dev/null "http://127.0.0.1:$site/" && curl -s -o /dev/null "http://127.0.0.1:$proctor/"; then
    echo "stack up: site $site, proctor $proctor"
    exit 0
  fi
  sleep 0.5
done
echo "stack did not come up"
exit 1
