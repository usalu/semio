#!/usr/bin/env zsh
# 👥️ Slice TC3c — provision the two humans on the 7651 root, then run the create-and-attach probe
# once per kind the three-package generation should offer. Shape copied from 📜️c5-provision.sh:
# `os-hub credential set` is the only credential verb the binary has, and it runs against OS_HUB_DATA.
# Usage: 📜️tc3c-provision.sh [port] [dataRoot] [binary]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7651}
DATA=${2:-"$ROOT/.🧬semio/🌐hub/tc3c-boot"}
BIN=${3:-"$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3c/debug/os-hub"}
ORIGIN="http://127.0.0.1:$PORT"
cd "$ROOT" || exit 1

for row in "user1@semio.dev:User One:gm1-local-dev-pass-1" "user2@semio.dev:User Two:gm1-local-dev-pass-2"; do
  email="${row%%:*}"; rest="${row#*:}"; name="${rest%%:*}"; pass="${rest#*:}"
  printf '%s' "$pass" | OS_HUB_DATA="$DATA" "$BIN" credential set --email "$email" --display-name "$name"
  echo "credential set $email exit=$?"
done

for kind in s.note.note s.gis.gismap; do
  echo "=========== $kind ==========="
  bun "$TICKET/🐍️tc3c-create-and-attach.ts" "$ORIGIN" "$kind" 2>&1 | tee "$GEN/tc3c-attach-${kind//./-}.txt"
  echo "probe exit=$?"
done
