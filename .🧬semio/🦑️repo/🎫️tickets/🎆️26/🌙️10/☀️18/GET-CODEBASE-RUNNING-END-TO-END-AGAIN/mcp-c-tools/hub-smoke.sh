#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$PWD/$C/🗑️generated/mcp-c"
BIN="$G/os-hub"
PORT="${HUB_SMOKE_PORT:-8797}"
OUT="$G/hub-smoke.out"
: > "$OUT"
if [ ! -x "$BIN" ] || [ -n "$HUB_SMOKE_REBUILD" ]; then
  env -u SEMIO_SCRIPT_STALL_MS nice -n 10 repo cargo --store=shared --configuration=prototype -- build --manifest-path 🌎️hub/Cargo.toml -p semio-hub --bin os-hub --message-format=json-render-diagnostics > "$G/hub-build.log" 2>&1
  rc=$?
  exe=$(/usr/bin/grep -a '"reason":"compiler-artifact"' "$G/hub-build.log" | /usr/bin/grep -a '"name":"os-hub"' | tail -1 | python3 -c "import sys,json
for l in sys.stdin:
    d=json.loads(l); print(d.get('executable') or '')")
  [ -n "$exe" ] && cp "$exe" "$BIN" && echo "copied $exe rc=$rc" >> "$OUT"
  [ -x "$BIN" ] || { echo "build failed rc=$rc" >> "$OUT"; exit 1; }
fi
DATA=$(mktemp -d "$G/hub-data.XXXXXX")
chmod 700 "$DATA"
PW=$(python3 -c "import secrets;print(secrets.token_urlsafe(18))")
EMAIL="smoke@example.com"
export OS_HUB_DATA="$DATA" OS_HUB_PASSWORD_ITERATIONS=2000
printf '%s\n' "$PW" | "$BIN" credential set --email "$EMAIL" --display-name "Smoke" >> "$OUT" 2>&1
echo "credential set rc=$?" >> "$OUT"
OS_HUB_MODE=production OS_HUB_BIND=127.0.0.1 OS_HUB_PORT="$PORT" OS_HUB_CREDENTIAL_SIGN_IN=true OS_HUB_ADMIN_SUBJECTS="credential.password.v1:$EMAIL" "$BIN" > "$G/hub-serve.log" 2>&1 &
pid=$!
echo "serve pid=$pid" >> "$OUT"
ok=no
for i in $(seq 1 60); do
  code=$(curl -s -m 2 -o /dev/null -w "%{http_code}" "http://127.0.0.1:$PORT/healthz") && [ "$code" = "200" ] && { ok=yes; break; }
  kill -0 $pid 2>/dev/null || break
  sleep 2
done
echo "healthz ok=$ok" >> "$OUT"
echo "readyz: $(curl -s -m 5 -w ' http=%{http_code}' "http://127.0.0.1:$PORT/readyz" | head -c 1500)" >> "$OUT"
body=$(printf '{"schema":"semio.hub.auth.credential-sign-in/v1","email":"%s","password":"%s","deviceInstanceId":"hub-smoke-1","clientClass":"native"}' "$EMAIL" "$PW")
echo "signin: $(curl -s -m 10 -H 'content-type: application/json' -d "$body" -w ' http=%{http_code}' "http://127.0.0.1:$PORT/auth/sessions" | sed 's/"token":"[^"]*"/"token":"[REDACTED]"/; s/"[a-zA-Z]*[Tt]oken":"[^"]*"/"token":"[REDACTED]"/g' | head -c 600)" >> "$OUT"
badbody=$(printf '{"schema":"semio.hub.auth.credential-sign-in/v1","email":"%s","password":"wrong-password","deviceInstanceId":"hub-smoke-2","clientClass":"native"}' "$EMAIL")
echo "wrong-password: $(curl -s -m 10 -H 'content-type: application/json' -d "$badbody" -w ' http=%{http_code}' "http://127.0.0.1:$PORT/auth/sessions" | head -c 300)" >> "$OUT"
kill -TERM $pid 2>/dev/null
for i in 1 2 3 4 5 6 7 8 9 10; do kill -0 $pid 2>/dev/null || break; sleep 1; done
kill -0 $pid 2>/dev/null && kill -KILL $pid 2>/dev/null
wait $pid 2>/dev/null; echo "serve exit=$?" >> "$OUT"
rm -rf "$DATA"
exit 0
