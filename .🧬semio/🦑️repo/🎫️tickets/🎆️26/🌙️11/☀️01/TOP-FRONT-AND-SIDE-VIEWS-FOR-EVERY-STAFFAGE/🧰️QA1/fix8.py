import json,glob,re
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/herring-gull/🔣️.json')[0]
d=json.load(open(f,encoding='utf8'))
for sc in ('50','100','200'):
    for s in d['asset']['representations'][sc]['geometry']['front']:
        m=re.fullmatch(r'(near|far)-(arm|hand)',s['id'])
        if not m: continue
        sg=-1 if m.group(1)=='near' else 1
        # centred on the pivot (+-0.068): arm band 0.06 wide, hand band 0.056 wide
        if m.group(2)=='arm':
            lo,hi=0.098,0.038; y0,y1=0.094,-0.014
            pts=[[0.098,0.094],[0.098,-0.014],[0.038,-0.014],[0.038,0.094]]
        else:
            pts=[[0.096,0.082],[0.093,-0.002],[0.043,-0.002],[0.04,0.082]]
        s['points']=[[sg*x,y] for x,y in pts]
        if sg>0: s['points']=list(reversed(s['points']))
open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
