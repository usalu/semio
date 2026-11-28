#!/bin/zsh
# Builds a scratch tree where the staged growth module sits at its final relative place (real copy), everything else symlinked to the checkout.
R=/Users/ueli/Documents/semio/🧰️framework/🔨️modules
F=${1:?farm dir}
HERE=${0:A:h}
P=🏃️process/📦️artifacts/🏗️native-build/💾️storage
mkdir -p $F/🔨️modules/$P/🍎️image
[ -e $F/🔨️modules/🧬️schema ] || ln -s $R/🧬️schema $F/🔨️modules/🧬️schema
for d in $R/🏃️process/*(N); do n=${d:t}; [ $n = 📦️artifacts ] || [ -e $F/🔨️modules/🏃️process/$n ] || ln -s $d $F/🔨️modules/🏃️process/$n; done
for d in $R/🏃️process/📦️artifacts/*(N); do n=${d:t}; [ $n = 🏗️native-build ] || [ -e $F/🔨️modules/🏃️process/📦️artifacts/$n ] || ln -s $d $F/🔨️modules/🏃️process/📦️artifacts/$n; done
for d in $R/$P:h/*(N); do n=${d:t}; [ $n = 💾️storage ] || [ -e $F/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/$n ] || ln -s $d $F/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/$n; done
for d in $R/$P/*(N); do n=${d:t}; [ $n = 🍎️image ] || [ -e $F/🔨️modules/$P/$n ] || ln -s $d $F/🔨️modules/$P/$n; done
for d in $R/$P/🍎️image/*(N); do n=${d:t}; [ $n = 🪜️growth ] || [ -e $F/🔨️modules/$P/🍎️image/$n ] || ln -s $d $F/🔨️modules/$P/🍎️image/$n; done
for f in $R/$P/🍎️image/*(.N); do [ -e $F/🔨️modules/$P/🍎️image/${f:t} ] || ln -s $f $F/🔨️modules/$P/🍎️image/${f:t}; done
rm -rf $F/🔨️modules/$P/🍎️image/🪜️growth; cp -R $HERE/stage/🍎️image/🪜️growth $F/🔨️modules/$P/🍎️image/🪜️growth
echo $F/🔨️modules/$P/🍎️image/🪜️growth
