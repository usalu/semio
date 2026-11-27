#!/usr/bin/env bash
# 🔒️ Refreshes the Cargo.lock of every workspace given on stdin (one workspace dir per line), one cargo at a time.
set -uo pipefail
out="${1:?log dir}"
while IFS= read -r ws; do
  [ -z "$ws" ] && continue
  name=$(printf '%s' "$ws" | tr '/' '_' | tr -cd '[:alnum:]_.-')
  if SEMIO_LANGUAGE=en repo cargo --configuration=prototype --parallelism=1 --store=shared -- tree --manifest-path "$ws/Cargo.toml" --workspace --depth 0 --offline -e normal > "$out/lock-$name.txt" 2>&1; then
    echo "ok $ws"
  else
    echo "FAILED $ws"; tail -5 "$out/lock-$name.txt"
  fi
done
