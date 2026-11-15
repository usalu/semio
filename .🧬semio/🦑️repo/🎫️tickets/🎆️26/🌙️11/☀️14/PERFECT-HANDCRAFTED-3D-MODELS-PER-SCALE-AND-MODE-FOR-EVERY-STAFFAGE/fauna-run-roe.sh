#!/bin/bash
S=$1; B=./author-29-bun.sh; A=animals/roe-deer
$B fauna-roe.ts $S --write || exit 1
python3 fauna-addth.py $A $S "coat-trunk=spine-hind+spine-fore,coat-hindlimbs=near-hock+far-hock,coat-forelimbs=near-carpus+far-carpus+near-elbow+far-elbow" >/dev/null
$B kit-structure.ts $A --scale $S --write 2>&1 | grep -E "outside" | cut -c1-120
$B volumes-check.ts $A --product --scale $S --quiet 2>&1 | grep -v "^\[check" | tr ' ' '\n' | grep -E "^!|^FAIL" | tr '\n' ' '; echo
