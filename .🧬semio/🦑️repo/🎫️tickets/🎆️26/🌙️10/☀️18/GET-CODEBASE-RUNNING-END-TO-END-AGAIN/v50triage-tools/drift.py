import json,sys,hashlib,os
R='/Users/ueli/Documents/semio'
b=sys.argv[1]
m=json.load(open(os.path.join(b,'manifest.json')))
for rel,e in m.items():
    p=os.path.join(R,rel)
    h=hashlib.sha256(open(p,'rb').read()).hexdigest() if os.path.exists(p) else None
    st='new' if h==e['new'] else ('base' if h==e['base'] else 'DRIFT')
    print(st,rel[-70:])
