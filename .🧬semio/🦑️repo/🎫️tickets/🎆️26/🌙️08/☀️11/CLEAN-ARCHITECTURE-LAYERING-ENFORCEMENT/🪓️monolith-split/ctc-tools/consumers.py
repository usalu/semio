import json,sys
h=json.load(open('../🗑️generated/ctc-scan4-hits.json'))
o=sys.argv[1]
print(' '.join(sorted({x['consumerCrate'] for x in h if x['owner']==o and not x['sanctioned'] and x['consumerCrate']!=o})))
