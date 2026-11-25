#!/usr/bin/env bash
set -uo pipefail
purge() { find /tmp/build -type f -perm -u+x ! -name "*.so" ! -name "*.rlib" ! -name "*.rmeta" ! -name "*build*script*" -delete 2>/dev/null || true; }
for p in "$@"; do
  extra=()
  if [[ "$p" == "semio-s-artifact-puzzle-5d" ]]; then extra=(--features component-app-assembly); fi
  echo "=== TEST $p ==="
  cargo test -p "$p" --all-targets "${extra[@]}" --no-fail-fast --jobs 2 --offline
  echo "TEST $p:$?"
  purge
done
