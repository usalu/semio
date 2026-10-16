#!/bin/zsh
cd "${0:A:h}"
while pgrep -f 'ab.py norm' >/dev/null; do sleep 10; done
python3 ab.py process process > ../🗑️generated/pfa-ab-process.log 2>&1
python3 ab.py bim bim > ../🗑️generated/pfa-ab-bim.log 2>&1
