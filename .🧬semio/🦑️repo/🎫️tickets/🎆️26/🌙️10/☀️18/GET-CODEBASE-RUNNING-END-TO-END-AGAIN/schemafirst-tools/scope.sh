#!/bin/sh
# Lists today's changed production TS/MJS sources in the schema-first scope (relative to the repo root).
C=$(cd "$(dirname "$0")/.." && pwd)
grep -E '/🏗️build/|/🌡️host-capacity/|/🧠️memo/|🔧️tools/🏗️bazel/' "$C/compliance-tools/today.txt" | grep -E '\.(ts|mjs)$' | grep -v '\.d\.mts$' | grep -v '/🧪️' | sort
