#!/bin/zsh
. "$(dirname "$0")/env.sh"
cd "$R"
for n in 📤️file-open-import ⬇️media-export-encoding 🚫️reject-malformed-version-input ✅️satisfy-version-requirements 🧵️spawned-job-drive 🐢️ui-dirty-scope 🧵️video-render-job 🎞️video-render-program 📦️receipt 🧪️createturnoutcomebroadcast; do
  echo "=== $n"
  perl -e 'alarm 300; exec @ARGV' -- bun test "$KD/🧪️tests/$n/🟦️.ts" 2>&1 | tail -8
done
