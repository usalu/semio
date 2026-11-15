#!/bin/bash
cd "$(dirname "$0")"
for a in "$@"; do
  python3 areareview3-dump.py "${a%%:*}" 50 "${a#*:}" | cut -c1-165 | grep -v "^poses\|^   pose\|SUM all"
done
