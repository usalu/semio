#!/bin/sh
# Runs every batch after and before on fresh mirrors, one at a time; log lines go to stdout.
T="$(cd "$(dirname "$0")" && pwd)"
for B in unitcache rootguard sweep ledger procstore; do
  echo "##### $B after $(date +%H:%M:%S)"; sh "$T/verify.sh" $B 2>&1 | /usr/bin/grep -E '^(==|\(fail\)|mirror: guard|stage:|GUARD)'
  echo "##### $B before $(date +%H:%M:%S)"; sh "$T/verify.sh" $B before 2>&1 | /usr/bin/grep -E '^(==|\(fail\)|mirror: guard|stage:|GUARD)'
done
echo "##### done $(date +%H:%M:%S)"
