#!/bin/sh
# 🚀️ H1: boots the real `os-hub:dev` Nx route (the `🛠️dev🗄️os-hub` launch.json row) against a brand
# new OS_HUB_DATA with no SEMIO_BUILD_BUDGET_MS, waits for a 200 on /readyz, lists what the data root
# persisted, stops the run by pid, then boots the SAME data root a second time to prove the state
# survives a restart. Usage: 📜️h1-dev-boot.sh <capture-dir> <port> [wait-seconds] [data-dir]
set -eu
capture=$1
port=$2
waits=${3:-1800}
repo=$(cd "$(dirname "$0")/../../../../../../.." && pwd)
data=${4:-$(mktemp -d /private/tmp/h1-dev-data-XXXXXX)}
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

boot() {
  phase=$1
  log="$capture/h1-dev-$phase.txt"
  echo "=== $phase: OS_HUB_DATA=$data OS_HUB_PORT=$port SEMIO_BUILD_BUDGET_MS=${SEMIO_BUILD_BUDGET_MS:-<unset>} ===" > "$log"
  cd "$repo"
  OS_HUB_DATA="$data" OS_HUB_PORT="$port" nohup bun nx run os-hub:dev >> "$log" 2>&1 &
  pid=$!
  echo "$phase pid=$pid" >> "$log"
  elapsed=0
  status=000
  while [ "$elapsed" -lt "$waits" ]; do
    status=$(curl -s -o "$capture/h1-dev-$phase-readyz.json" -w "%{http_code}" "http://127.0.0.1:$port/readyz" || echo 000)
    [ "$status" = "200" ] && break
    kill -0 "$pid" 2>/dev/null || { echo "$phase: dev route exited before readiness" >> "$log"; break; }
    sleep 5
    elapsed=$((elapsed + 5))
  done
  echo "$phase readyz http=$status after ${elapsed}s" | tee -a "$log"
  if [ "$status" = "200" ]; then
    echo "--- $phase /readyz body ---" >> "$log"
    cat "$capture/h1-dev-$phase-readyz.json" >> "$log"
    echo "" >> "$log"
    echo "--- $phase /directory/spaces ---" >> "$log"
    curl -s -o /dev/stdout -w " <status %{http_code}>\n" "http://127.0.0.1:$port/directory/spaces" >> "$log" 2>&1 || true
  fi
  echo "--- $phase data root ---" >> "$log"
  find "$data" -maxdepth 3 -print >> "$log"
  descendants=$(ps -eo pid=,ppid= | awk -v root="$pid" 'BEGIN { n = 1; keep[root] = 1 } { parent[$1] = $2 } END { for (p in parent) { c = p; d = 0; while (c != 1 && d < 32) { if (c == root) { print p; break } c = parent[c]; d = d + 1 } } }')
  kill -TERM "$pid" $descendants 2>/dev/null || true
  sleep 3
  kill -9 "$pid" $descendants 2>/dev/null || true
  sleep 2
  [ "$status" = "200" ] || return 1
}

boot first || { echo "first boot did not reach /readyz"; exit 1; }
find "$data" -type f | sort > "$capture/h1-dev-data-after-first.txt"
boot restart || { echo "restart boot did not reach /readyz"; exit 1; }
find "$data" -type f | sort > "$capture/h1-dev-data-after-restart.txt"
echo "data dir retained: $data"
diff "$capture/h1-dev-data-after-first.txt" "$capture/h1-dev-data-after-restart.txt" > "$capture/h1-dev-data-diff.txt" 2>&1 || true
echo "both boots reached /readyz 200"
