import json,sys,glob,re
for p in glob.glob('**/*.json',recursive=True):
    try: d=json.load(open(p,encoding='utf8'))
    except Exception as e: print('PARSE',p,e); continue
    def w(o,path):
        if isinstance(o,dict):
            if set(o)=={'en','de'} and all(isinstance(v,str) for v in o.values()):
                for l,v in o.items():
                    if re.match(r'^\s',v) or v!=v.strip() or re.search(r'\b(of|into|with|the|and|in|dess)\.$',v) or re.search(r'(sitzenplatte|trägtwert|\s;|;\s*[a-z]+ (is|ist) )',v) or (l=='en' and re.match(r'^[a-z]',v) and path.split('.')[-1] in('thinElements','omitted','merged','retained','overlays')) or (l=='de' and re.match(r'^[a-zäöü]',v) and 'decisions' in path and path.split('.')[-1] in('thinElements','omitted','merged','retained','overlays')): print(p,path,l,repr(v[:140]))
            else:
                for k,v in o.items(): w(v,path+'.'+k)
        elif isinstance(o,list):
            for i,v in enumerate(o): w(v,path+'.'+str(i))
    w(d,'')
