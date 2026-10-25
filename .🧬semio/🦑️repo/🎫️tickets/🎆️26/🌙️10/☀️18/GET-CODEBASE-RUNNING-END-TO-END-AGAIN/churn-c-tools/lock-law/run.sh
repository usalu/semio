#!/bin/sh
# Runs outside the repo tree (scratchpad) so no repository .cargo/config.toml applies.
# usage: run.sh <name> <variants...>   Runs one cargo per variant CONCURRENTLY on one shared build dir (copy of this crate under 🗑️generated/churn-c/lock-<name>) and judges the interval log with oracle.py.
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-pSPkwj/ad7098a3-86cf-42c9-85e1-65386eec4277/scratchpad
G="$PWD/$C/🗑️generated/churn-c"
TC=/Users/ueli/Documents/semio/.cache/b/t/m/r/toolchains/nightly-2026-07-20-aarch64-apple-darwin # the repository pinned nightly; a scratch crate outside the repository, so no repository build is bypassed
name=$1; shift
W="$S/lock-$name"
rm -rf "$W"; mkdir -p "$W"; cp -R "$C/churn-c-tools/lock-law/." "$W/"; rm -f "$W/run.sh"
: > "$W/log.txt"
pids=""
for variant in "$@"; do
  ( CHURN_LOG="$W/log.txt" CHURN_VARIANT="$variant" PATH="$TC/bin:$PATH" RUSTUP_TOOLCHAIN= nice -n 10 "$TC/bin/cargo" -Zunstable-options -C "$W" build > "$W/cargo-$variant.out" 2>&1; echo "rc=$?" >> "$W/cargo-$variant.out" ) &
  pids="$pids $!"
  sleep 2
done
wait
python3 -I "$W/oracle.py" "$W/log.txt" "$#" > "$W/oracle.json" 2>&1; echo "oracle rc=$?" >> "$W/oracle.json"
