#!/usr/bin/env bash
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
cd "$R"
for i in 1 2 3 4 5 6; do
  repo run @semio-tech/assets:build > "$T/🗑️generated/normcomp-assets-try$i.log" 2>&1
  [ -f "$R/🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs" ] && { echo ok > "$T/🗑️generated/normcomp-assets-done.log"; exit 0; }
  sleep 60
done
echo fail > "$T/🗑️generated/normcomp-assets-done.log"
