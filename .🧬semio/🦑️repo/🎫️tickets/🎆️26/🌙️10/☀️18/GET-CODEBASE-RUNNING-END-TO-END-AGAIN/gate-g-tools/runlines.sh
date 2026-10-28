#!/bin/sh
# 🧪 Runs a verify script detached-friendly and prints only summary lines: `runlines.sh <script> <log>`.
sh "$1" > "$2" 2>&1; echo "exit $?" >> "$2"
