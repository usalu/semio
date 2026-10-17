#!/bin/zsh
# usage: checkapplied.sh <label>   the three artifact packages one after the other
T=${0:A:h}
for n in shooting fem3d bim; do "$T/checkapplied-one.sh" $1 $n; done
echo done >> "${T:h}/🗑️generated/c4p32-checkapplied-$1.exit"
