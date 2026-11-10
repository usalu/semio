#!/usr/bin/env bash
# waits until `repo tools exec --light bun -- --version` works, then runs the test file given
cd /home/deck/semio
for i in $(seq 1 90); do
  if repo tools exec --light bun -- --version >/dev/null 2>&1; then
    repo tools exec --light bun -- test "./$1" 2>&1 | grep -aE "^ *[0-9]+ (pass|fail)|^\(fail\)|^error:|Expected|Received" | head -20 | cut -c1-250
    exit 0
  fi
  sleep 20
done
echo "repo cli unavailable"
