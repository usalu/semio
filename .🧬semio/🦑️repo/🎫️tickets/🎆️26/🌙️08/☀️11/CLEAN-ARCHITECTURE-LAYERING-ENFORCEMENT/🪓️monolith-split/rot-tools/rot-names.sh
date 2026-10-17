#!/bin/sh
# usage: rot-names.sh <log> ; prints per-file test names with verbose pass/fail by re-listing is not available, so print failing names
/usr/bin/grep -E '^\(fail\)' "$1" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort
