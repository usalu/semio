#!/usr/bin/env zsh
# 👥️ Slice C5 — provision outcome 3's two humans, their shared space and ONE gis Map document on a
# hub data root, then make BOTH of them AUTHOR members of that space.
#
# The three product routes, in the order the hub requires them (GM1 §5):
#   1. `os-hub credential set --email … --display-name …`, password on stdin, against OS_HUB_DATA —
#      the only credential verb the binary has (`🌎️hub/🔐️auth/📤️command/🦀️.rs:41`).
#   2. `🐍️gm1-live-open-plan.ts` — sign-in → create-space → `POST /spaces/{s}/artifact-creations`
#      polled to `ready` → `open-plan`. A bare `announce-document` is NOT enough (GM1 §4b).
#   3. `🐍️c3-add-member.ts` — the author invites the second human through the product's own sealed
#      `upsert-member` directory command (C3 §3.1).
#
# Prints `C5-PROVISION space=<id> document=<id>` on success; those two ids parameterise every probe.
# Usage: 📜️c5-provision.sh <port> <dataRoot> <binary>
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7621}
DATA=${2:-"$ROOT/.🧬semio/🌐hub/jc1-boot"}
BIN=${3:-"$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-jc1/debug/os-hub"}
ORIGIN="http://127.0.0.1:$PORT"

cd "$ROOT" || exit 1
for row in "user1@semio.dev:User One:gm1-local-dev-pass-1" "user2@semio.dev:User Two:gm1-local-dev-pass-2"; do
  email="${row%%:*}"; rest="${row#*:}"; name="${rest%%:*}"; pass="${rest#*:}"
  printf '%s' "$pass" | OS_HUB_DATA="$DATA" "$BIN" credential set --email "$email" --display-name "$name"
  echo "credential set $email exit=$?"
done

bun "$TICKET/🐍️gm1-live-open-plan.ts" "$ORIGIN" user1@semio.dev gm1-local-dev-pass-1 | tee "$GEN/c5-open-plan.txt"
line=$(grep -m1 "^GM1-LIVE-OPEN-PLAN ok " "$GEN/c5-open-plan.txt")
[ -n "$line" ] || { echo "C5-PROVISION FAILED: no open-plan line"; exit 1; }
SPACE="${${line##*space=}%% *}"
DOCUMENT="${line##*document=}"

bun "$TICKET/🐍️c3-add-member.ts" "$ORIGIN" "$SPACE" user2@semio.dev author | tee "$GEN/c5-add-member.txt"
echo "C5-PROVISION space=$SPACE document=$DOCUMENT" | tee "$GEN/c5-provision.txt"
