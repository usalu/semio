#!/bin/bash
f=/Users/Shared/semio-store/v1/origins/g1-m05/ready-log-dependencies.txt
echo "$(date -u +%T) $(grep -o 'store-ownership:[a-zA-Z]*:[0-9]*' $f | tail -2 | tr '\n' ' ') $(grep -o 'still running after [0-9]* s' $f | tail -1) $(grep -o 'providers: [0-9]* edges' $f | tail -1)" >> m05samples.txt
python3 -c "import json;d=json.load(open('/Users/Shared/semio-store/v1/origins/g1-m05/ready.json'));print(d['state'],d['stage'],d['attempts'],d['updatedMs'])" >> m05samples.txt
