#!/bin/zsh
# usage: run-post.sh <label> <fam>...   runs the POS oracle for the given families one after the other
T=${0:A:h}
l=$1; shift
for f in "$@"; do "$T/run-oracle.sh" $f $l lib,test; done
