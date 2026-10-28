import json,sys,hashlib,os,glob
R='/Users/ueli/Documents/semio'
rel=sys.argv[1]
p=os.path.join(R,rel); h=hashlib.sha256(open(p,'rb').read()).hexdigest()
print('live',h[:12])
for mf in glob.glob('*-tools/staged/*/manifest.json'):
    m=json.load(open(mf))
    for r,e in m.items():
        if r==rel or r.endswith(sys.argv[2] if len(sys.argv)>2 else '\0'):
            if r!=rel: continue
            tag=('NEW' if e.get('new')==h else '')+('BASE' if e['base']==h else '')
            if tag or True: print(mf[:-14],tag,(e['base'] or '-')[:8],'->',(e.get('new') or '-')[:8])
