#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/mcp-b"
nice -n 10 repo cargo --store=shared --configuration=prototype -- build --manifest-path "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust/Cargo.toml" --bin semio-repo-coordinator --message-format=json-render-diagnostics > "$G/coordinator-build.log" 2>&1 || { echo "build failed" > "$G/coordinator-smoke.out"; exit 1; }
exe=$(/usr/bin/grep -a '"reason":"compiler-artifact"' "$G/coordinator-build.log" | /usr/bin/grep -a '"name":"semio-repo-coordinator"' | /usr/bin/grep -a '"executable":"' | tail -1 | python3 -c "import sys,json
for l in sys.stdin:
    print(json.loads(l).get('executable') or '')")
[ -n "$exe" ] || { echo "no executable" > "$G/coordinator-smoke.out"; exit 1; }
cp "$exe" "$G/semio-repo-coordinator"
D=$(mktemp -d)
COMPOSE_SERVER_ADDR=127.0.0.1:6198 COMPOSE_SERVER_DB="$D/c.db" COMPOSE_SERVER_REPO_ROOT="$D" COMPOSE_SERVER_TOKEN=smoke "$G/semio-repo-coordinator" > "$G/coordinator-run.log" 2>&1 &
pid=$!
ok=no
for i in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20; do
  out=$(curl -s -m 2 -o /dev/null -w "%{http_code}" http://127.0.0.1:6198/healthz) && [ "$out" = "200" ] && { ok=yes; break; }
  sleep 1
done
body=$(curl -s -m 2 http://127.0.0.1:6198/healthz)
echo "healthz ok=$ok body=$body" > "$G/coordinator-smoke.out"
kill $pid 2>/dev/null; wait $pid 2>/dev/null
rm -rf "$D"
