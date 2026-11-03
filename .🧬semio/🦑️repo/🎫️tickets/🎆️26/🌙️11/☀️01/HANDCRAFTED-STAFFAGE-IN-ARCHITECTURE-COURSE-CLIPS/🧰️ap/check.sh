#!/usr/bin/env bash
cd /c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage || exit 1
repo tools exec bun -- "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$1" 2>&1 | grep -v '^\[capacity\]' | grep -E '✓|✗|fail|error|Error' | head -20
