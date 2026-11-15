import json,sys
# usage: author-31-stretch.py <slug> <scale> <metres> [prefixes]  moves first/last section of lobe lofts outward along the bone axis (world y)
slug,S,g=sys.argv[1],sys.argv[2],float(sys.argv[3])
pre=tuple(sys.argv[4].split(',')) if len(sys.argv)>4 else ('lobe','fill')
f=f'volumes/vegetation/{slug}.json'
d=json.load(open(f))
n=0
for s in d['volumes'][S]['body']:
    if s['id'].startswith(pre) and s['kind']=='loft':
        s['sections'][0]['at'][0]-=g; s['sections'][-1]['at'][0]+=g; n+=1
json.dump(d,open(f,'w'),indent=1); open(f,'a').write('\n')
print('stretched',n)
