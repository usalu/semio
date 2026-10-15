#!/bin/zsh
# usage: cargo-retry.sh <logfile> <cargo args after --> ; retries infra/workspace-load failures (not real compile results)
LOG="$1"; shift
cd /Users/ueli/Documents/semio || exit 99
end=$(( $(date +%s) + 2400 ))
while true; do
  perl -e 'alarm 1750; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype -- "$@" > "$LOG" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && ! /usr/bin/grep -q "error\[E\|could not compile\|Finished \|aborting due to\|\"reason\":\"compiler-message\"" "$LOG" && [ $(date +%s) -lt $end ]; then
    echo "[retry] infra failure rc=$rc, sleeping 60s" >&2; sleep 60; continue
  fi
  echo "exit $rc"; exit $rc
done
