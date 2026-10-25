#!/bin/sh
# usage: then.sh <T> <G>: wfc-2d recheck, then the test chain
T=$1; G=$2
sh "$T/chk2.sh" "$T/checks4.tsv" "$G/chk4-verdicts.tsv"
sh "$T/tst.sh" "$T/test-groups.tsv" "$G/tst-verdicts.tsv"
