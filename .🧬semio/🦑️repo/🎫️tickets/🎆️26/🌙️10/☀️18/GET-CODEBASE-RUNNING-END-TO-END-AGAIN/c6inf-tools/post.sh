#!/bin/zsh
# usage: post.sh <tag>  (module split tags): install + all live oracles
cd "$(dirname "$0")" || exit 1
t=$1
export C5B_NORM=1
R=/Users/ueli/Documents/semio
d=$(python3 -c "
import importlib,os,sys
sys.path.insert(0,'.')
sp=importlib.import_module('specs.$t').SPEC
print(os.path.join('$R',os.path.dirname(sp['file'])))")
python3 modsplit.py install $t || exit 1
python3 vmod.py live $t | tail -4
python3 defscan.py $t $d | tail -2
python3 ambig.py live $t | tail -3
C5B_LIVE=1 python3 modclash.py $t
python3 modsplit.py lossless $t $d | grep -E "^(missing|LOSSLESS)"
python3 fmt.py $t | head -4
python3 oracle5.py $t | grep -E "^==|^ORACLE|^  [-+]"
