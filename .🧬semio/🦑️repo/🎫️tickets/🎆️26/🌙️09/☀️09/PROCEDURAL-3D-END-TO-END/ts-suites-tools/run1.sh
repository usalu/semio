#!/bin/bash
export LC_ALL=en_US.UTF-8
f="$1"; G="$2"; id=$(printf '%s' "$f" | shasum | cut -c1-10)
s=$(date +%s); timeout 400 repo test-file "$f" > "$G/logs/$id.log" 2>&1; rc=$?
echo "$rc $(( $(date +%s)-s ))s $id $f" >> "$G/results.txt"
