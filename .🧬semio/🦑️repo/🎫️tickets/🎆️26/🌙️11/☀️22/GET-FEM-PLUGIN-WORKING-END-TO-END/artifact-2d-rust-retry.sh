#!/usr/bin/env bash
# usage: artifact-2d-rust-retry.sh <log> <cargo args...>
log="$1"; shift
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" || exit 2
for i in 1 2 3 4 5 6 7 8 9 10 11 12; do
  repo cargo --store=shared --configuration=prototype -- "$@" > "$log" 2>&1
  code=$?
  if grep -q "Internal error: EBUSY" "$log"; then
    cp "$log" "$log.ebusy$i"
    sleep 20
    continue
  fi
  echo "exit $code" >> "$log"
  exit $code
done
echo "exit gave-up-ebusy" >> "$log"
