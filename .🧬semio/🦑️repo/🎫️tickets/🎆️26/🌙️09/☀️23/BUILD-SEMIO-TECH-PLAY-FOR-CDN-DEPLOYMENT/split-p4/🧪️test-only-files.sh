#!/bin/bash
# 🧪️ Prints the source files a crate root's test closure adds over its lib closure (repository-relative).
# usage: 🧪️test-only-files.sh <crate root 🦀️.rs>
L="$(cd "$(dirname "$0")" && pwd)"
cd "$L/../../../../../../../.." || exit 1
comm -13 <(bun "$L/📐️closure-files.ts" "$1" lib | awk -F'\t' '$2=="source"{print $1}' | sort) <(bun "$L/📐️closure-files.ts" "$1" test | awk -F'\t' '$2=="source"{print $1}' | sort)
