import json,glob,collections
S='/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog'
rows=[]
for p in sorted(glob.glob(S+'/*/*/🔣️.json')):
    a=json.load(open(p))['asset']
    for sc,v in a.get('volumes',{}).items():
        for lst in ('body','thermal','structure'):
            for s in v.get(lst,[]) or []:
                if s.get('sectionFaces'): rows.append((a['id'],sc,lst,s['id'],tuple(s['sectionFaces']),s.get('region'),s.get('of')))
print(len(rows))
c=collections.Counter((r[0],r[1],r[2]) for r in rows)
for k,n in sorted(c.items()): print(k,n)
json.dump(rows,open('🗑️generated/areareview/sectionfaces.json','w'),ensure_ascii=False)
