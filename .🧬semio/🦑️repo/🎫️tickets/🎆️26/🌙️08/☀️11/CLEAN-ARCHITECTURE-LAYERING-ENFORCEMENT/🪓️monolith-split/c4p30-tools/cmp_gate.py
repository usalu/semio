import json,collections,re,sys
def load(p,R):
    c=collections.Counter()
    for l in open(p,encoding='utf-8'):
        if not l.startswith('{'): continue
        try: m=json.loads(l)
        except: continue
        if m.get('level')!='error': continue
        code=(m.get('code') or {}).get('code') or ''
        msg=m.get('message','').replace(R,'<sandbox>')
        if 'aborting due to' in msg: continue
        c[code+'\t'+msg]+=1
    return c
base='/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/c4p30-work/'
a=load(base+'gdo-test.json',base+'ab-orig/')
b=load(base+'gdc-test.json',base+'gd-drawing-combined/')
print('orig total',sum(a.values()),'unique',len(a),'| split total',sum(b.values()),'unique',len(b))
oa=a-b; ob=b-a
print('only-orig',sum(oa.values()),'only-split',sum(ob.values()))
for k,v in list(ob.items())[:15]: print(' +split',v,k[:200])
for k,v in list(oa.items())[:15]: print(' -orig',v,k[:200])
