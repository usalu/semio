import json,glob,subprocess,os
REPO='/Users/ueli/Documents/semio'
S=REPO+'/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog'
rows=[]
for p in sorted(glob.glob(S+'/*/*/🔣️.json')):
    a=json.load(open(p))['asset']
    rel=os.path.relpath(p,REPO)
    try: h={r['id']:r for r in json.loads(subprocess.check_output(['git','-C',REPO,'show','HEAD:'+rel],stderr=subprocess.DEVNULL))['asset']['thermal']['regions']}
    except Exception: h={}
    for r in a['thermal']['regions']:
        L=r['layers']
        if len(L)<2: continue
        cur=[l['id'] for l in L]; old=[l['id'] for l in h[r['id']]['layers']] if r['id'] in h else None
        changed = old is not None and sorted(old)==sorted(cur) and old!=cur
        other = old is not None and sorted(old)!=sorted(cur)
        flag = L[-1]['material']!=r['material'] and any(l['material']==r['material'] for l in L)
        rows.append(dict(asset=a['id'],region=r['id'],model=r['model'],kind=r['kind'],material=r['material'],cur=[(l['id'],l['material'],l['kind'],l.get('thicknessM'),l['resistanceM2KW']) for l in L],old=old,changed=changed,other=other,flag=flag))
json.dump(rows,open('🗑️generated/areareview/layers.json','w'),ensure_ascii=False,indent=1)
print(len(rows),sum(r['changed'] for r in rows),sum(r['flag'] for r in rows),sum(r['other'] for r in rows))
for r in rows:
    if r['changed'] or r['flag']:
        print(('CHG ' if r['changed'] else '    ')+('FLAG ' if r['flag'] else '     '),r['asset'],r['region'],r['model'],r['material'],'|',' > '.join(f"{c[0]}:{c[1]}" for c in r['cur']),'| old',r['old'])
