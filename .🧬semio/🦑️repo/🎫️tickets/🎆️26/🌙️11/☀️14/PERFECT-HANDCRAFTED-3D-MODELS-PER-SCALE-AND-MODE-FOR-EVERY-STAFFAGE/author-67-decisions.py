"""usage: python3 author-67-decisions.py <slug>  : sets decisions of the scales named in author-67-decisions.json[slug] (list of [en, de] pairs per scale; a scale not listed keeps its decisions); an entry beginning with '+' appends to the existing list."""
import json,sys
slug=sys.argv[1]
D=json.load(open('author-67-decisions.json'))[slug]
f=f'volumes/vegetation/{slug}.json'
d=json.load(open(f))
for S,items in D.items():
    new=[{'en':e,'de':g} for e,g in items]
    d['volumes'][S]['decisions']=new
json.dump(d,open(f,'w'),indent=1); open(f,'a').write('\n')
print('decisions',slug,{k:len(v) for k,v in D.items()})
