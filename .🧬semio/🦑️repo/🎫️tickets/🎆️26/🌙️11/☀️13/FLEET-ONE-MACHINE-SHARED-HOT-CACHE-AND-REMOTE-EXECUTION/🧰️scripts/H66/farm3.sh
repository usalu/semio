#!/bin/zsh
# Scratch tree for the doctor storage probe: growth farm + real copies of the probe and its test, everything else symlinked.
F=${1:?dir}; FARM=${2:?farm}; PROBE=${3:?probe 🟦️.ts}; TEST=${4:?test 🟦️.ts}
RF=/Users/ueli/Documents/semio/🧰️framework
chain=(🛍️products 🦑️repo 🔨️modules 📚️library 🏗️build 🩺️doctor)
rm -rf $F; mkdir -p $F/🧰️framework; ln -s $FARM/🔨️modules $F/🧰️framework/🔨️modules
cur=$F/🧰️framework; real=$RF
mirror(){ # mirror $1(real) into $2(cur) except names in $3...
  local r=$1 c=$2; shift 2
  for e in $r/*(N); do n=${e:t}; [[ ${@[(Ie)$n]} -gt 0 ]] || [ -e $c/$n ] || ln -s $e $c/$n; done
}
for part in $chain; do mirror $real $cur $part; mkdir -p $cur/$part; cur=$cur/$part; real=$real/$part; done
mirror $real $cur 🔎️probes 🧪️tests
mkdir -p $cur/🔎️probes/💾️storage $cur/🧪️tests/🔎️probes
mirror $real/🔎️probes $cur/🔎️probes 💾️storage
mirror $real/🔎️probes/💾️storage $cur/🔎️probes/💾️storage 🟦️.ts
cp $PROBE $cur/🔎️probes/💾️storage/🟦️.ts
mirror $real/🧪️tests $cur/🧪️tests 🔎️probes
mirror $real/🧪️tests/🔎️probes $cur/🧪️tests/🔎️probes 🟦️.ts
cp $TEST $cur/🧪️tests/🔎️probes/🟦️.ts
echo $cur
