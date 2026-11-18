#!/bin/sh
cd "$1"; out="$2"; shift 2; : > "$out"
for f in "$@"; do echo "### $f" >> "$out"; timeout 900 repo tools exec bun -- test "$f" 2>&1 | grep -v "capacity\|still working" >> "$out"; done
echo "### done" >> "$out"
