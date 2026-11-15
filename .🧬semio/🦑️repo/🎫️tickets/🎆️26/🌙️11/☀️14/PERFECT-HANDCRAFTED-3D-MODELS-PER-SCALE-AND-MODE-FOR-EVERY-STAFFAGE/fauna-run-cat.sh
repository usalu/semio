#!/bin/bash
S=$1; B=./author-29-bun.sh; A=animals/domestic-cat
$B fauna-cat.ts $S --write || exit 1
$B volumes-check.ts $A --product --scale $S --quiet 2>&1 | grep -v "^\[check" | cut -c1-500 | grep -v "^ok"
