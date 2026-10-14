#!/bin/bash
# 🔒️ Resyncs the lockfiles of the given workspaces offline (first-party additions only) and refuses any registry or checksum change.
# usage: 🔒️resync.sh <workspace Cargo.toml>...
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
G="$T/🗑️generated/l2/locks"; mkdir -p "$G"
for manifest in "$@"; do
  lock="$(dirname "$manifest")/Cargo.lock"; key=$(echo "$lock" | shasum | cut -c1-12)
  [ -f "$lock" ] && cp "$lock" "$G/$key.before" || : > "$G/$key.before"
  out=$(bash "$T/l2-feature-free/🦀️cargo.sh" 300 metadata --offline --format-version 1 --manifest-path "$manifest" 2>&1 >/dev/null); rc=$?
  third=$(diff "$G/$key.before" "$lock" | rg -c '^[<>].*(source = "registry|checksum = )')
  echo "rc=$rc third-party-lines=${third:-0} changed=$(diff "$G/$key.before" "$lock" | rg -c '^[<>]') $manifest"
  [ $rc -ne 0 ] && echo "$out" | tail -3
done
