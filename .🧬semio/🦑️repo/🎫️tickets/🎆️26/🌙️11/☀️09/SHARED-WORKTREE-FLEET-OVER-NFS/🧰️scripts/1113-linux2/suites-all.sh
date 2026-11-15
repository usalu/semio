#!/bin/bash
# runs inside the container: /mir is a writable copy of the module's ancestor mirror, live tree mounted ro at its host path. usage: suites-all.sh [filter]
M=$(cd /mir && ls -d 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree)
cd "/mir/$M" || exit 2
mkdir -p /tmp/home
echo "platform $(uname -sm) bun $(bun --version) uid $(id -u)"
for f in $(ls */🧪️tests/*.ts */*/🧪️tests/*.ts 2>/dev/null); do
  case "$f" in *harness.ts|*sweep.ts|*killpoints.ts|*store.ts) ;; esac
  [ -n "$1" ] && case "$f" in *$1*) ;; *) continue;; esac
  d=$(dirname "$f"); b=$(basename "$f")
  echo "=== $f"
  ( cd "$d" && env -i PATH="$PATH" HOME=/tmp/home TMPDIR=/tmp DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1 POWERSHELL_TELEMETRY_OPTOUT=1 bun test "./$b" --timeout 180000 2>&1 | grep -E "^\s*(\(fail\)|\(skip\)|[0-9]+ (pass|fail|skip|todo)|Ran [0-9]+)|error:|^error" | head -40 )
done
