#!/bin/zsh
# 🔬️ RB1 §3 — the production posture nobody has ever attempted: the RELEASE `os-hub` binary, as a
# plain process (no cargo, no launcher, no fd 3), bound to a REAL NETWORK ADDRESS of this machine,
# under the three statements `validate_auth_startup` demands for a network bind.
#
# Differences from P4's loopback `📜️p4-production-runtime.sh`, which this replaces for the network case:
#   · the binary is `dist/build/os-hub` (release), not a debug coordinator build;
#   · the bind is `ipconfig getifaddr en0`, not 127.0.0.1, so OS_HUB_ALLOWED_ORIGINS +
#     OS_HUB_TRUSTED_FORWARDING=proxy are mandatory and their refusals are exercised;
#   · `X-Forwarded-Proto` enforcement is live, so every request has to speak as the proxy would;
#   · the data root is a copy of a published catalog root, so trusted-catalog reuse across a
#     different binary is observed rather than assumed;
#   · a socket is held open across the SIGTERM to watch the drain close it.
set -u

REPO="/Users/ueli/Documents/semio"
TICKET="$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
SRC="$REPO/🌎️hub/📦️packages/🦀️rust/dist/build/os-hub"
RUN="$TICKET/🗑️generated/rb1-runtime"
BIN="$RUN/os-hub"
DATA="$REPO/.🧬semio/🌐hub/rb1-prod"
# 🧬️ The seed root must carry a catalog published under the SAME codegen policy this binary
# compiles: `hs1-boot`'s generation is jco-1.27.0 and this release binary is jco-1.34, which it
# refuses outright at boot with
# `ArtifactAuthority(Catalog("trusted browser actor identity differs from its package or renderer"))`
# — measured 11:10 on 2026-09-21. `jc1-boot` holds generation 8086b61f… (jco-1.34.0).
SEED="${RB1_SEED:-$REPO/.🧬semio/🌐hub/jc1-boot}"
PORT="${RB1_PORT:-7661}"
ADDR="$(ipconfig getifaddr en0)"
EMAIL="ada@example.com"
PASSWORD="correct horse battery staple"
PROVIDER="credential.password.v1"
ORIGIN="http://$ADDR:6081"

say() { printf '\n=== %s ===\n' "$*"; }

rm -rf "$RUN"; mkdir -p "$RUN"
[ -f "$SRC" ] || { echo "FAIL: no release binary at $SRC"; exit 1; }
[ -n "$ADDR" ] || { echo "FAIL: en0 has no address"; exit 1; }

say "0. the release binary — rm + cp + codesign, never an in-place overwrite"
rm -f "$BIN"; cp "$SRC" "$BIN"; chmod +x "$BIN"
codesign --force --sign - "$BIN" 2>&1 | sed 's/^/codesign: /'
codesign --verify --verbose=2 "$BIN" 2>&1 | sed 's/^/codesign: /'
ls -l "$BIN" | awk '{print "size:", $5, "bytes"}'
file "$BIN" | sed 's/^/file: /'
echo "bind address: $ADDR  port: $PORT  origin: $ORIGIN"

say "1. data root — a reused published catalog root, operator-private"
if [ ! -d "$DATA" ]; then
  echo "copying $SEED → $DATA"
  cp -R "$SEED" "$DATA"
fi
chmod 700 "$DATA"
ls -la "$DATA" | sed 's/^/data: /'
stat -f "%Sp %N" "$DATA" | sed 's/^/mode: /'

export OS_HUB_DATA="$DATA"

say "2. the three refusals a network bind must produce before it is allowed to boot"
refuse() {
  local label="$1"; shift
  local out
  out=$(env "$@" "$BIN" 2>&1 | tail -3)
  echo "$label -> ${out:-(no output)}"
}
refuse "no OS_HUB_ALLOWED_ORIGINS   " OS_HUB_MODE=production OS_HUB_BIND="$ADDR" OS_HUB_PORT="$PORT" OS_HUB_CREDENTIAL_SIGN_IN=true OS_HUB_ADMIN_SUBJECTS="$PROVIDER:$EMAIL" OS_HUB_TRUSTED_FORWARDING=proxy
refuse "no OS_HUB_TRUSTED_FORWARDING" OS_HUB_MODE=production OS_HUB_BIND="$ADDR" OS_HUB_PORT="$PORT" OS_HUB_CREDENTIAL_SIGN_IN=true OS_HUB_ADMIN_SUBJECTS="$PROVIDER:$EMAIL" OS_HUB_ALLOWED_ORIGINS="$ORIGIN"
refuse "no OS_HUB_CREDENTIAL_SIGN_IN" OS_HUB_MODE=production OS_HUB_BIND="$ADDR" OS_HUB_PORT="$PORT" OS_HUB_ADMIN_SUBJECTS="$PROVIDER:$EMAIL" OS_HUB_ALLOWED_ORIGINS="$ORIGIN" OS_HUB_TRUSTED_FORWARDING=proxy

say "3. first user through the operator verb — no server running, no fd 3"
printf '%s' "$PASSWORD" | "$BIN" credential set --email "$EMAIL" --display-name "Ada" 2>&1 | sed 's/^/credential-set: /'

boot() {
  OS_HUB_MODE=production \
  OS_HUB_BIND="$ADDR" \
  OS_HUB_PORT="$PORT" \
  OS_HUB_CREDENTIAL_SIGN_IN=true \
  OS_HUB_ADMIN_SUBJECTS="$PROVIDER:$EMAIL" \
  OS_HUB_ALLOWED_ORIGINS="$ORIGIN" \
  OS_HUB_TRUSTED_FORWARDING=proxy \
  OS_HUB_STORAGE_BACKEND=fs \
  OS_HUB_DIRECTORY_BACKEND=sqlite \
  "$BIN" > "$1" 2>&1 &
  HUB_PID=$!
}

# 🧭️ Every request speaks as the TLS-terminating proxy would, because OS_HUB_TRUSTED_FORWARDING=proxy
# makes X-Forwarded-Proto enforced rather than advisory.
PX=(-H "X-Forwarded-Proto: https" -H "X-Forwarded-Host: hub.example.com")

wait_ready() {
  for _ in $(seq 1 180); do
    curl -fsS "${PX[@]}" -m 3 "http://$ADDR:$PORT/healthz" >/dev/null 2>&1 && return 0
    kill -0 "$1" 2>/dev/null || return 1
    sleep 1
  done
  return 1
}

say "4. boot — production, NETWORK bind, plain process"
boot "$RUN/boot1.txt"; PID1=$HUB_PID
if wait_ready "$PID1"; then
  echo "booted pid $PID1 on $ADDR:$PORT"
  grep -aiE "bind scope|cross-origin|forwarding|listening|ready" "$RUN/boot1.txt" | head -6 | sed 's/^/boot: /'
else
  echo "FAIL: never became live"; tail -25 "$RUN/boot1.txt" | sed 's/^/boot: /'; exit 1
fi

say "5. the transport statement is enforced, not advisory"
echo -n "GET /healthz WITHOUT X-Forwarded-Proto  -> "; curl -s -o /dev/null -m 5 -w '%{http_code}\n' "http://$ADDR:$PORT/healthz"
echo -n "GET /healthz with X-Forwarded-Proto:http -> "; curl -s -o /dev/null -m 5 -H 'X-Forwarded-Proto: http' -w '%{http_code}\n' "http://$ADDR:$PORT/healthz"
echo -n "GET /healthz as the proxy                -> "; curl -s -o /dev/null -m 5 "${PX[@]}" -w '%{http_code}\n' "http://$ADDR:$PORT/healthz"
echo -n "GET /readyz  as the proxy                -> "; curl -s -m 10 "${PX[@]}" -w ' [%{http_code}]\n' "http://$ADDR:$PORT/readyz" | head -c 900; echo

say "6. the cross-origin allowlist"
echo -n "preflight from the allowed origin $ORIGIN -> "
curl -s -o /dev/null -m 5 -X OPTIONS "${PX[@]}" -H "Origin: $ORIGIN" -H 'Access-Control-Request-Method: POST' -w '%{http_code}\n' "http://$ADDR:$PORT/auth/sessions"
echo "  allow-origin header: $(curl -s -D- -o /dev/null -m 5 -X OPTIONS "${PX[@]}" -H "Origin: $ORIGIN" -H 'Access-Control-Request-Method: POST' "http://$ADDR:$PORT/auth/sessions" | grep -i 'access-control-allow-origin' | tr -d '\r')"
echo "  a foreign origin:    $(curl -s -D- -o /dev/null -m 5 -X OPTIONS "${PX[@]}" -H 'Origin: https://evil.example.com' -H 'Access-Control-Request-Method: POST' "http://$ADDR:$PORT/auth/sessions" | grep -ic 'access-control-allow-origin') allow-origin header(s)"

say "7. credential sign-in over the network bind"
sign_in() {
  curl -sS -o "$RUN/signin.json" -w '%{http_code}' -m 20 "${PX[@]}" \
    -H 'content-type: application/json' -H "Origin: $ORIGIN" \
    -X POST "http://$ADDR:$PORT/auth/sessions" \
    --data "{\"schema\":\"semio.hub.auth.credential-sign-in/v1\",\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\",\"deviceInstanceId\":\"rb1-probe\",\"clientClass\":\"browser\"}"
}
CODE=$(sign_in); echo "POST /auth/sessions -> $CODE"
TOKEN=$(sed -n 's/.*"token":"\([^"]*\)".*/\1/p' "$RUN/signin.json")
USER=$(sed -n 's/.*"user_id":"\([^"]*\)".*/\1/p' "$RUN/signin.json")
echo "user_id: $USER  token: ${TOKEN:0:16}… (${#TOKEN} chars)"
echo -n "GET /auth/sessions/me -> "; curl -sS -m 10 "${PX[@]}" -H "Authorization: Bearer $TOKEN" "http://$ADDR:$PORT/auth/sessions/me" | head -c 300; echo
echo -n "a wrong password      -> "
curl -s -o /dev/null -m 10 "${PX[@]}" -H 'content-type: application/json' -X POST "http://$ADDR:$PORT/auth/sessions" \
  --data "{\"schema\":\"semio.hub.auth.credential-sign-in/v1\",\"email\":\"$EMAIL\",\"password\":\"wrong\",\"deviceInstanceId\":\"rb1-probe\",\"clientClass\":\"browser\"}" -w '%{http_code}\n'

say "8. pool-worker stack — HS1's 64 MiB fix in a binary launched WITHOUT cargo"
echo "RUST_MIN_STACK in this environment: '${RUST_MIN_STACK:-(unset)}' (cargo's .cargo/config.toml floor is NOT in play here)"
# 🧨️ `wait` with no arguments waits for EVERY background job, and the hub is one of them — so it
# blocks until the hub exits, which is never. Wait on exactly these eight pids.
FANOUT=()
for i in 1 2 3 4 5 6 7 8; do
  curl -s -o /dev/null -m 10 "${PX[@]}" -H "Authorization: Bearer $TOKEN" "http://$ADDR:$PORT/directory/spaces" &
  FANOUT+=($!)
done
for pid in "${FANOUT[@]}"; do wait "$pid"; done
echo "stack overflow lines in the hub log so far: $(grep -ac 'stack overflow' "$RUN/boot1.txt")"
echo "SIGABRT/panic lines:                        $(grep -acE 'panicked at|SIGABRT|fatal runtime' "$RUN/boot1.txt")"

say "9. SIGTERM with a socket held open — the drain, the socket, the exit code"
python3 - "$ADDR" "$PORT" "$RUN/socket.txt" <<'PY' &
import socket, sys, time
host, port, out = sys.argv[1], int(sys.argv[2]), sys.argv[3]
s = socket.create_connection((host, port), timeout=10)
opened = time.time()
with open(out, "w") as f:
    f.write("socket opened\n"); f.flush()
    s.settimeout(60)
    try:
        data = s.recv(1024)
        f.write("server closed the socket after %.2fs (recv returned %d bytes)\n" % (time.time()-opened, len(data)))
    except socket.timeout:
        f.write("socket STILL OPEN after 60s — no clean close\n")
    except OSError as e:
        f.write("socket error after %.2fs: %s\n" % (time.time()-opened, e))
PY
SOCK_PID=$!
sleep 2
BEFORE=$(wc -l < "$RUN/boot1.txt" | tr -d " ")
echo "sending SIGTERM to $PID1 at $(date '+%T')"
kill -TERM "$PID1"
wait "$PID1"; STATUS=$?
echo "exit status after SIGTERM: $STATUS"
wait "$SOCK_PID" 2>/dev/null
cat "$RUN/socket.txt" | sed 's/^/held socket: /'
tail -n "+$BEFORE" "$RUN/boot1.txt" | grep -aiE 'readiness|drain|shut|cancel|stop' | head -8 | sed 's/^/drain: /'
echo -n "the port after exit -> "; curl -s -o /dev/null -m 3 "${PX[@]}" -w '%{http_code}\n' "http://$ADDR:$PORT/healthz"

say "10. restart — the same sqlite root, the same user"
boot "$RUN/boot2.txt"; PID2=$HUB_PID
if wait_ready "$PID2"; then
  CODE2=$(sign_in); echo "POST /auth/sessions after restart -> $CODE2"
  USER2=$(sed -n 's/.*"user_id":"\([^"]*\)".*/\1/p' "$RUN/signin.json")
  echo "same user_id across restart: $([ "$USER" = "$USER2" ] && echo "yes ($USER)" || echo "NO ($USER vs $USER2)")"
  echo "stack overflow lines across both boots: $(cat "$RUN/boot1.txt" "$RUN/boot2.txt" | grep -ac 'stack overflow')"
else
  echo "FAIL: restart never became live"; tail -20 "$RUN/boot2.txt" | sed 's/^/boot2: /'
fi

say "11. the durable store format stamps, and the reused trusted catalog"
for role in authority projections blobs sessions; do
  printf '%s: ' "$role"; cat "$DATA/instance/$role/format.json" 2>/dev/null || echo "(missing)"; echo
done
echo "trusted-catalog pointer: $(cat "$DATA/trusted-catalog/current.json" 2>/dev/null | head -c 300)"
grep -aiE 'catalog|codegen|policy|refus' "$RUN/boot2.txt" | head -8 | sed 's/^/catalog: /'

echo "$PID2" > "$RUN/hub.pid"
say "hub $PID2 left running on $ADDR:$PORT for the s-bundle probe; stop it with: kill -TERM \$(cat $RUN/hub.pid)"
