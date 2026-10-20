#!/bin/sh
# Stock Nx 23.2.0 (unpatched, installed under /private/tmp/conc-nx/stock) concurrency evidence; at most 2 concurrent nx processes.
# 1 same-hash concurrent puts, 2 concurrent put + restore of other hashes, 3 integrity of every restored output, 4 graph reuse vs construction.
set -e
T=$(cd "$(dirname "$0")" && pwd); W=/private/tmp/conc-nx/work; mkdir -p $W
[ -d /private/tmp/conc-nx/stock/node_modules/nx ] || { mkdir -p /private/tmp/conc-nx/stock && cd /private/tmp/conc-nx/stock && echo '{"name":"stock","private":true,"devDependencies":{"nx":"23.2.0"}}' > package.json && bun install --no-progress --backend=copyfile >/dev/null; }
rm -rf /private/tmp/conc-nx/fx && python3 $T/fixture.py /private/tmp/conc-nx/fx 300 20 >/dev/null && ln -sfn /private/tmp/conc-nx/stock/node_modules /private/tmp/conc-nx/fx/node_modules
cd /private/tmp/conc-nx/fx
( $W/nx.sh e1 run-many -t build --projects='p00[0-4]*' --parallel=2 & $W/nx.sh e2 run-many -t build --projects='p00[0-4]*' --parallel=2 & wait )
python3 $T/verify-fixture-outputs.py .
rm -f p00[0-4]*/out.txt
( $W/nx.sh e3 run-many -t build --projects='p00[5-9]*,p01*' --parallel=2 & $W/nx.sh e4 run-many -t build --projects='p00[0-4]*' --parallel=2 & wait )
grep "Cache:" $W/e3.log $W/e4.log
python3 $T/verify-fixture-outputs.py .
