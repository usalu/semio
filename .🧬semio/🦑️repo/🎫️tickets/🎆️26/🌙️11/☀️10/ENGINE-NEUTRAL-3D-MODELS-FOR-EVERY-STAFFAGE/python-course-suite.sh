#!/usr/bin/env bash
# Runs the listed course unittest modules one by one with real manim and a per-module limit. Usage: python-course-suite.sh <modules-file> <result-file> [limit-seconds]
T="$(cd "$(dirname "$0")" && pwd)"; . "$T/python-manim-env.sh"; cd /Users/ueli/Documents/semio
export SEMIO_TEACHING_LANGUAGE="${SEMIO_TEACHING_LANGUAGE:-en}"
LIMIT="${3:-600}"; : > "$2"
G="$T/🗑️generated/python-course/suite"; mkdir -p "$G"
while IFS= read -r module; do
  start=$(date +%s)
  log="$G/$(echo "$module" | md5 -q).txt"
  perl -e 'alarm shift; exec @ARGV' "$LIMIT" "$PY" -m unittest "$module" > "$log" 2>&1; rc=$?
  echo "$rc	$(( $(date +%s) - start ))s	$(tail -n 3 "$log" | tr '\n' ' ' | cut -c1-100)	$module	$log" >> "$2"
done < "$1"
echo finished >> "$2"
