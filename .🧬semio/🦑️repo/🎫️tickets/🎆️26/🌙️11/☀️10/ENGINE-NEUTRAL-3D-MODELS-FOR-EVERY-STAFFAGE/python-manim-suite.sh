#!/usr/bin/env bash
# Runs every unittest module of the course `test` command (📜️script.ts) one by one with the real manim, each with a time limit. Usage: python-manim-suite.sh <result-file> [limit-seconds]
T="$(cd "$(dirname "$0")" && pwd)"; . "$T/python-manim-env.sh"; cd /Users/ueli/Documents/semio
export SEMIO_TEACHING_LANGUAGE="${SEMIO_TEACHING_LANGUAGE:-en}"
LIMIT="${2:-240}"; : > "$1"
while IFS= read -r module; do
  start=$(date +%s)
  log="$T/🗑️generated/python-manim/suite/$(echo "$module" | md5 -q).txt"; mkdir -p "$(dirname "$log")"
  perl -e 'alarm shift; exec @ARGV' "$LIMIT" "$PY" -m unittest "$module" > "$log" 2>&1; rc=$?
  echo "$rc	$(( $(date +%s) - start ))s	$(tail -n 3 "$log" | tr '\n' ' ' | cut -c1-120)	$module	$log" >> "$1"
done < "$T/🗑️generated/python-manim/all-modules.txt"
echo finished >> "$1"
