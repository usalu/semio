#!/bin/zsh
# usage: docs.sh <file>... : prints the line number and first 110 chars of every doc comment line
for f in "$@"; do echo "--- ${f:t:h} ($f)"; grep -n -E '^[ 	]*(///|//!|/\*\*|// #region)' "$f" | cut -c1-130; done
