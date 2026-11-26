#!/usr/bin/env bash
# 🔁️ Reruns one xi-law until its log shows no compile error (peers break unrelated crates transiently). Usage: xi-retry.sh <filter> <tag> <target>
cd "$(dirname "$0")"
for i in 1 2 3 4 5 6 7 8; do
  bash xi-law.sh "$1" "$2" semio-s-artifact-fem-3d-editor "$3"
  grep -aq "^error" "🗑️generated/xi/law-$2.log" || break
  sleep 120
done
