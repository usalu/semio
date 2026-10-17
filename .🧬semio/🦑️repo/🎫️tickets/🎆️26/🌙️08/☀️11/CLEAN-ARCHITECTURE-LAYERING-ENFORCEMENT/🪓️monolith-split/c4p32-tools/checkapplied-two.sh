#!/bin/zsh
# usage: checkapplied-two.sh <label> <name>...   runs checkapplied-one.sh for the named packages one after the other
T=${0:A:h}
label=$1; shift
for n in "$@"; do "$T/checkapplied-one.sh" $label $n; done
echo done >> "${T:h}/🗑️generated/c4p32-checkapplied-$label.exit"
