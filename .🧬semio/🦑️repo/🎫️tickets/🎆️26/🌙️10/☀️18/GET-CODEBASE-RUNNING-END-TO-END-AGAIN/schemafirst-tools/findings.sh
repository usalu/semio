#!/bin/sh
# usage: findings.sh <row slug>...: the law's findings of the rows on the mirror (Python oracle), with 90 chars of context
. "$(dirname "$0")/env.sh"
cd "$MR"
D="$L/🏗️build/🧪️tests/🧬️policy-literals"
for s in "$@"; do
row=$(python3 -I -c "
import os,re,sys
for n in os.listdir(sys.argv[1]):
    if re.sub(r'^[^A-Za-z0-9]+','',n)==sys.argv[2]: print(os.path.join(sys.argv[1],n,'🔣️.json'))" "$D" "$s")
python3 -I "$D/🐍️.py" . "$row" | python3 -c "
import json,sys,os
mr=os.environ['MR'];L=os.environ['L']
for k,v in json.load(sys.stdin).items():
    row=json.load(open(k))
    base=mr+'/'+{'library':L,'process':os.environ['P'],'repository':''}[row['root']]
    for r in v:
        line=open(base+'/'+r['source'],encoding='utf8').read().split('\n')[r['line']-1]
        needle=str(r['value'])
        i=line.find(needle)
        print(r['source'][-34:],r['line'],r['kind'],r['value'],'|',line[max(0,i-60):i+40].strip())"
done
