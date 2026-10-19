#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
bun $C/rgplug-tools/gate.ts $(python3 -c "
import json,sys
d=json.load(open('$C/rg-rest-tools/targets.json'))
print('\n'.join(d.values()))
") 
