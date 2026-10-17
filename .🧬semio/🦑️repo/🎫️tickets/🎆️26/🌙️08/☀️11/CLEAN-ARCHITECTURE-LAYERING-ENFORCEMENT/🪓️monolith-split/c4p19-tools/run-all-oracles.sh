#!/bin/zsh
# usage: run-all-oracles.sh <label> [cfgs]   runs the eight families one after the other
T=${0:A:h}
for f in a b c d e f g h; do "$T/run-oracle.sh" $f $1 ${2:-lib,test}; done
