#!/usr/bin/env bash
cd "$(dirname "$0")"
n=0
while :; do bash "$1"; l="🗑️generated/${1%.sh}.log"; grep -q 'could not compile' "$l" && grep -q 'tool-machine' "$l" && [ $n -lt 3 ] && { n=$((n+1)); sleep 300; continue; }; break; done
