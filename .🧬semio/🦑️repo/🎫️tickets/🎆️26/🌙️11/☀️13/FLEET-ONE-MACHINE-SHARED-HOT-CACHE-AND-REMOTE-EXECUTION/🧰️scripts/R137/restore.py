import json,os,hashlib
SC='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R137'
log=json.load(open(SC+'/landed.json'));sha=lambda b:hashlib.sha256(b).hexdigest()[:12]
for p,v in log.items():
    cur=open(p,'rb').read() if os.path.exists(p) else b''
    if sha(cur)!=v['after']: print('CHANGED BY PEER, left',p);continue
    if v['before'] is None: os.remove(p);print('removed',p[-50:])
    else: open(p,'wb').write(open(SC+'/backup/'+sha(p.encode()),'rb').read());print('restored',p[-50:])
