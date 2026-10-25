#!/bin/sh
# usage: family.sh <tag-prefix> <package-suffix>...   (one graph-free cargo test per package, so one compile fault never masks the others)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
W="🧰️framework/🔨️modules/🧵️job/👷️worker/📦️packages/🦀️rust/Cargo.toml"
prefix="$1"; shift
for n in "$@"; do
  case "$n" in
    job) F=semio-framework-job/testkit;;
    job-worker) F=semio-framework-job-worker/testkit;;
    job-worker-authority) F=semio-framework-job-worker-authority/testkit;;
    *) F=;;
  esac
  if [ -n "$F" ]; then
    sh "$C/jobw-d-tools/cargo.sh" "$prefix-$n" test --manifest-path "$W" -p "semio-framework-$n" --features "$F" --no-fail-fast
  else
    sh "$C/jobw-d-tools/cargo.sh" "$prefix-$n" test --manifest-path "$W" -p "semio-framework-$n" --no-fail-fast
  fi
done
echo done > "$C/🗑️generated/jobw-d/$prefix.family.done"
