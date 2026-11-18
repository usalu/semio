#!/bin/zsh
R=/Users/ueli/Documents/semio
L=$R/.cache/f/graph-server/read.log
now=$(date -u +%H:%M:%S)
rec=$(cat $R/.cache/f/graph-server/read-record.json)
proj=$(grep -a 'Inferring projects:' $L | tail -1 | sed -E 's/.*Inferring projects: ([0-9]+ of [0-9]+) projects · ([0-9]+) running.*/\1 run\2/')
aside=$(grep -a -c ' is set aside' $L)
own=$(grep -a '^\[owners\] owners [0-9]* of' $L | tail -1 | sed -E 's/\[owners\] owners //')
wait=$(grep -a '^\[owners\] no owner arrived' $L | tail -1 | sed -E 's/\[owners\] no owner arrived for [0-9]+ minutes //' | cut -c1-90)
rd=$(echo "$rec" | python3 -c "import sys,json;print(json.load(sys.stdin).get('read'))")
kid=$(ps -o pid=,ppid= -ax | awk -v p=$rd '$2==p{print $1}' | head -1)
ch=$(ps -o pid=,ppid= -ax | awk -v p=$kid '$2==p' | wc -l | tr -d ' ')
cpu=$(ps -o %cpu= -p $kid | tr -d ' ')
echo "$now | projects $proj | aside $aside | ledger $own | $wait | children(of read) $ch | parent cpu $cpu | load $(uptime | sed -E 's/.*averages: //')"
L=$R/.cache/f/graph-server/read.log
echo "   farm: $(grep -a -c 'were not used\|no more worker\|refused:\|workerRefused\|was not used because' $L) off/refused lines; by-reason: $(grep -a 'by reason' $L | tail -1 | cut -c1-200)"
echo "   ledger: $(grep -a '^\[owners\] owners [0-9]* of' $L | tail -1 | cut -c1-200)"
echo "   pools: $(python3 $(dirname $0)/pools.py | cut -c1-260)"
