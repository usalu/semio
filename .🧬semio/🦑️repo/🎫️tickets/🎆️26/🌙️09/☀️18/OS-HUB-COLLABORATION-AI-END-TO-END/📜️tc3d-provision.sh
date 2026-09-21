#!/usr/bin/env zsh
# 👥️ Slice TC3d — provision the two humans on the 7651 root, then run TC3c's create-and-attach probe
# once per kind the three-package generation offers. Same shape as 📜️tc3c-provision.sh with TC3d's
# data root and binary, and with `s.stdio.*` added: stdio is the package the hub DOES link a Rust
# codec for, so running it beside the two unlinked kinds is the regression half of the proof —
# a guest-resolver fix must not have moved the natively linked creation path.
# Usage: 📜️tc3d-provision.sh [port] [dataRoot] [binary] [kinds…]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7651}
DATA=${2:-"$ROOT/.🧬semio/🌐hub/tc3d-boot"}
BIN=${3:-"$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3d/debug/os-hub"}
shift 3 2>/dev/null || true
KINDS=("$@")
[ ${#KINDS[@]} -gt 0 ] || KINDS=(s.note.note s.gis.gismap)
ORIGIN="http://127.0.0.1:$PORT"
cd "$ROOT" || exit 1

for row in "user1@semio.dev:User One:gm1-local-dev-pass-1" "user2@semio.dev:User Two:gm1-local-dev-pass-2"; do
  email="${row%%:*}"; rest="${row#*:}"; name="${rest%%:*}"; pass="${rest#*:}"
  printf '%s' "$pass" | OS_HUB_DATA="$DATA" "$BIN" credential set --email "$email" --display-name "$name"
  echo "credential set $email exit=$?"
done

for kind in "${KINDS[@]}"; do
  echo "=========== $kind ==========="
  bun "$TICKET/🐍️tc3c-create-and-attach.ts" "$ORIGIN" "$kind" 2>&1 | tee "$GEN/tc3d-attach-${kind//./-}.txt"
  echo "probe exit=$?"
done
