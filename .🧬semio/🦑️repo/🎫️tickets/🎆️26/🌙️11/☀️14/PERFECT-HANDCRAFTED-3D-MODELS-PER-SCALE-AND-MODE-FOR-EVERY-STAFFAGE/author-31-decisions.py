import json,sys
slug=sys.argv[1]
D=json.load(open('author-31-decisions.json'))[slug]
f=f'volumes/vegetation/{slug}.json'
d=json.load(open(f))
for S,items in D.items():
    d['volumes'][S]['decisions']=[{'en':e,'de':g} for e,g in items]
json.dump(d,open(f,'w'),indent=1); open(f,'a').write('\n')
print('decisions',slug,{k:len(v) for k,v in D.items()})
