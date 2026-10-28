#!/bin/sh
# 📋 [backlog] repo-relative files that differ between two mirrors (second = baseline, default blbase). usage: changed.sh <mirror> [base]
exec python3 -I "$(dirname "$0")/changed.py" "$@"
