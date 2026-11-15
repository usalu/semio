import json,sys
# usage: author-31-grow.py <slug> <scale> <metres> [prefixes]  grows lobe lofts' sections outward by an offset
slug,S,g=sys.argv[1],sys.argv[2],float(sys.argv[3])
pre=tuple(sys.argv[4].split(',')) if len(sys.argv)>4 else ('lobe','fill')
f=f'volumes/vegetation/{slug}.json'
d=json.load(open(f))
n=0
for s in d['volumes'][S]['body']:
    if s['id'].startswith(pre) and s['kind']=='loft':
        for x in s['sections']:
            x['rx']=max(0.02,round(x['rx']+g,5)); x['ry']=max(0.02,round(x['ry']+g,5))
        n+=1
json.dump(d,open(f,'w'),indent=1); open(f,'a').write('\n')
print('grown',n)
