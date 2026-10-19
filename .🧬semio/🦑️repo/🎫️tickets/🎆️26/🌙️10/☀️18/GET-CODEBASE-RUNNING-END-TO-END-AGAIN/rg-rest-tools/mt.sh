#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
M=$(python3 -c "
import json;d=json.load(open('$C/rg-rest-tools/targets.json'));print([v for k,v in d.items() if k.endswith('$1')][0])")
python3 $C/rg-rest-tools/modtree.py "$M" $2
