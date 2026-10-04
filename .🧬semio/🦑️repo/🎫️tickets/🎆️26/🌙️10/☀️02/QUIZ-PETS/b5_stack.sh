#!/usr/bin/env bash
# 🧱️ Ticket tool of work package B5: (re)starts or stops the private stack of `wp_j_private_stack.ts` under
# `🗑️generated/b5` on the ports of this package — site 6247, proctor 8947 — in either topology, one at a time: `dev`
# (development proctor, the site's dev server with TEACHING_ARCHITECTURE_QUIZ_WATCH=off) or `rehearsal` (production-mode
# proctor, the release build in `🗑️generated/b5/site-rehearsal`, built by `build`, served under its real policy). The
# proctor's data is a scratch folder under `🗑️generated/b5/stack-<topology>`. It stops only the processes whose ids that
# tool wrote down, and a stop of one topology also stops the other (they share the ports).
# Usage (from the repository root): bash ".../b5_stack.sh" <restart|stop|build> [dev|rehearsal]
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
scratch="$ticket/🗑️generated/b5"
topology="${2:-dev}"
site=6247
proctor=8947
mkdir -p "$scratch"
if [ "${1:-restart}" = "build" ]; then
  cd "$ticket/../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript" || exit 1
  NX_PLUGIN_NO_TIMEOUTS=true PROCTOR_URL="http://127.0.0.1:$proctor" bun ./📜️script.ts build --outDir "$scratch/site-rehearsal" --emptyOutDir > "$scratch/build-rehearsal.txt" 2>&1
  code=$?
  tail -5 "$scratch/build-rehearsal.txt"
  echo "build exit=$code"
  exit $code
fi
for known in dev rehearsal; do
  if [ -f "$scratch/stack-$known.pids" ]; then
    while read -r pid; do
      [ -n "$pid" ] && taskkill //PID "$pid" //T //F >/dev/null 2>&1
    done < "$scratch/stack-$known.pids"
    rm -f "$scratch/stack-$known.pids"
  fi
done
[ "${1:-restart}" = "stop" ] && exit 0
TEACHING_ARCHITECTURE_QUIZ_WATCH=off WP_STACK_SCRATCH=b5 WP_STACK_SITE_PORT=$site WP_STACK_PROCTOR_PORT=$proctor nohup bun "$ticket/wp_j_private_stack.ts" "$topology" > "$scratch/stack-$topology.out" 2>&1 &
for _ in $(seq 1 240); do
  if curl -s -o /dev/null "http://127.0.0.1:$site/" && curl -s -o /dev/null "http://127.0.0.1:$proctor/"; then
    echo "stack up ($topology): site $site, proctor $proctor"
    exit 0
  fi
  sleep 0.5
done
echo "stack did not come up"
exit 1
