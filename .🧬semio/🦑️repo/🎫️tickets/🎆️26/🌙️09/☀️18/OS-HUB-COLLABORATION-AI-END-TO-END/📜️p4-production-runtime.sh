#!/usr/bin/env bash
# 🔬️ P4 §8d — the production-posture runtime observation.
#
# Boots `os-hub` as a PLAIN PROCESS (no launcher, no fd 3) in production mode on loopback with the
# sqlite directory, then observes, in order: first user seeded by the operator verb → sign-in over
# HTTP → SIGTERM drain + exit code → restart persistence → a durable store stamped with a newer
# format version refuses the open. Own data root, own port, own copy of the binary.
set -u

REPO="/Users/ueli/Documents/semio"
TICKET="$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
SRC="$REPO/.🧬semio/🦑️repo/⚡️cache/cargo/target-coordinator-hub/debug/os-hub"
RUN="$TICKET/🗑️generated/p4-runtime"
BIN="$RUN/os-hub"
DATA="$RUN/data"
PORT="${P4_PORT:-8931}"
EMAIL="ada@example.com"
PASSWORD="correct horse battery staple"
PROVIDER="credential.password.v1"

say() { printf '\n=== %s ===\n' "$*"; }

rm -rf "$RUN"; mkdir -p "$RUN" "$DATA"

say "0. binary: rm + cp + codesign (never an in-place overwrite)"
[ -f "$SRC" ] || { echo "FAIL: no coordinator binary at $SRC"; exit 1; }
rm -f "$BIN"; cp "$SRC" "$BIN"; chmod +x "$BIN"
codesign --force --sign - "$BIN" 2>&1 | sed 's/^/codesign: /'
codesign --verify --verbose=2 "$BIN" 2>&1 | sed 's/^/codesign: /'
ls -l "$BIN" | awk '{print "size:", $5}'

export OS_HUB_DATA="$DATA"

say "1. first user through the operator verb — no server running, no fd 3"
printf '%s' "$PASSWORD" | "$BIN" credential set --email "$EMAIL" --display-name "Ada" 2>&1 | sed 's/^/credential-set: /'
echo "credential set exit: $?"

boot() {
  local log="$1"
  OS_HUB_MODE=production \
  OS_HUB_BIND=127.0.0.1 \
  OS_HUB_PORT="$PORT" \
  OS_HUB_CREDENTIAL_SIGN_IN=true \
  OS_HUB_ADMIN_SUBJECTS="$PROVIDER:$EMAIL" \
  OS_HUB_STORAGE_BACKEND=fs \
  OS_HUB_DIRECTORY_BACKEND=sqlite \
  "$BIN" > "$log" 2>&1 &
  HUB_PID=$!
}

wait_ready() {
  for _ in $(seq 1 120); do
    curl -fsS "http://127.0.0.1:$PORT/healthz" >/dev/null 2>&1 && return 0
    kill -0 "$1" 2>/dev/null || return 1
    sleep 1
  done
  return 1
}

sign_in() {
  curl -sS -o "$RUN/signin.json" -w '%{http_code}' \
    -H 'content-type: application/json' \
    -X POST "http://127.0.0.1:$PORT/auth/sessions" \
    --data "{\"schema\":\"semio.hub.auth.credential-sign-in/v1\",\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\",\"deviceInstanceId\":\"p4-probe\",\"clientClass\":\"browser\"}"
}

say "2. boot #1 — production, loopback, plain process"
boot "$RUN/boot1.txt"; PID1=$HUB_PID
if wait_ready "$PID1"; then
  echo "booted pid $PID1, /healthz answers"
  grep -E '^\[(INFO|WARN)\]|bind scope|ready' "$RUN/boot1.txt" | head -8 | sed 's/^/boot1: /'
  curl -sS "http://127.0.0.1:$PORT/readyz" | head -c 400 | sed 's/^/readyz: /'; echo
else
  echo "FAIL: boot #1 never became live"; sed -n '1,40p' "$RUN/boot1.txt" | sed 's/^/boot1: /'; exit 1
fi

say "3. sign in over HTTP"
CODE=$(sign_in); echo "POST /auth/sessions -> $CODE"
TOKEN=$(sed -n 's/.*"token":"\([^"]*\)".*/\1/p' "$RUN/signin.json")
USER=$(sed -n 's/.*"user_id":"\([^"]*\)".*/\1/p' "$RUN/signin.json")
echo "user_id: $USER  token: ${TOKEN:0:20}… (${#TOKEN} chars)"
echo -n "GET /auth/sessions/me -> "
curl -sS -H "Authorization: Bearer $TOKEN" "http://127.0.0.1:$PORT/auth/sessions/me" | head -c 300; echo

say "4. SIGTERM — the drain, and the exit code"
BEFORE=$(wc -l < "$RUN/boot1.txt")
kill -TERM "$PID1"
wait "$PID1"; STATUS=$?
echo "exit status after SIGTERM: $STATUS"
tail -n "+$((BEFORE))" "$RUN/boot1.txt" | sed 's/^/drain: /'

say "5. restart — the user is still there"
boot "$RUN/boot2.txt"; PID2=$HUB_PID
if wait_ready "$PID2"; then
  CODE2=$(sign_in); echo "POST /auth/sessions after restart -> $CODE2"
  USER2=$(sed -n 's/.*"user_id":"\([^"]*\)".*/\1/p' "$RUN/signin.json")
  echo "same user_id across restart: $([ "$USER" = "$USER2" ] && echo yes || echo "NO ($USER vs $USER2)")"
else
  echo "FAIL: restart never became live"; tail -20 "$RUN/boot2.txt" | sed 's/^/boot2: /'
fi
kill -TERM "$PID2" 2>/dev/null; wait "$PID2" 2>/dev/null

say "6. the durable store format stamps this build wrote"
for role in authority projections blobs sessions; do
  printf '%s: ' "$role"; cat "$DATA/instance/$role/format.json" 2>/dev/null || echo "(missing)"; echo
done

say "7. a store written by a newer build refuses the open"
python3 - "$DATA/instance/authority/format.json" <<'PY'
import json,sys
p=sys.argv[1]
d=json.load(open(p)); d["version"]=d["version"]+1
json.dump(d,open(p,"w"))
print("rewrote", p, "as", d)
PY
boot "$RUN/boot3.txt"; PID3=$HUB_PID
sleep 8
kill -0 "$PID3" 2>/dev/null && { echo "UNEXPECTED: hub still alive with a newer store format"; kill -TERM "$PID3"; } || echo "hub exited"
wait "$PID3" 2>/dev/null; echo "exit status with a newer store format: $?"
grep -iE "format|version|migration|Error" "$RUN/boot3.txt" | head -6 | sed 's/^/refusal: /'

say "done"
