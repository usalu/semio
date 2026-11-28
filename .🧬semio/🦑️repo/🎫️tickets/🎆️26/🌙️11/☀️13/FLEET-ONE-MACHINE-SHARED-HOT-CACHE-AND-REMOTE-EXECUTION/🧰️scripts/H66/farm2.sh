#!/bin/zsh
# Scratch tree for the pool image module: the growth farm (🔨️modules) plus a real copy of the pool image dir with the given patched files; every other entry is a symlink to the checkout.
F=${1:?farm2 dir}; FARM=${2:?farm dir}; PATCHED=${3:?pool 🟦️.ts}; TESTS=${4:?pool test}
RF=/Users/ueli/Documents/semio/🧰️framework
chain=(🛍️products 🦑️repo 🔨️modules 📚️library ⚡️caching 🦀️cargo 🗃️unit-cache 🧵️lane 🏊️pool 🧩️image)
rm -rf $F; mkdir -p $F/🧰️framework
ln -s $FARM/🔨️modules $F/🧰️framework/🔨️modules
cur=$F/🧰️framework; real=$RF
for part in $chain; do
  for e in $real/*(N) $real/.[!.]*(N); do n=${e:t}; [ $n = $part ] || [ -e $cur/$n ] || ln -s $e $cur/$n; done
  mkdir -p $cur/$part; cur=$cur/$part; real=$real/$part
done
for e in $real/*(N); do n=${e:t}; [ $n = 🟦️.ts ] || [ $n = 🧪️tests ] || ln -s $e $cur/$n; done
mkdir -p $cur/🧪️tests; cp $PATCHED $cur/🟦️.ts; cp $TESTS $cur/🧪️tests/🟦️.ts
echo $cur
