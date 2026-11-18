import hashlib,json,os,shutil,sys
LIVE=os.environ.get('LAND_LIVE','/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab')
SCR='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R155/stage'
FILES=json.load(open(SCR+'/files.json')); BEFOREJ=json.load(open(SCR+'/before.json'))
MOD=[r for r in FILES['overlay']+FILES.get('edit',[]) if r in BEFOREJ]; NEW=[r for r in FILES['overlay'] if r not in BEFOREJ]
sha=lambda p:hashlib.sha256(open(p,'rb').read()).hexdigest()
mode,bk=sys.argv[1],sys.argv[2]
if mode=='apply':
    before=json.load(open(SCR+'/before.json'))
    for r in MOD: assert sha(LIVE+'/'+r)==before[r],('live changed since staging',r)
    for r in NEW: assert not os.path.exists(LIVE+'/'+r),('exists',r)
    os.makedirs(bk,exist_ok=True)
    for r in MOD:
        os.makedirs(os.path.dirname(bk+'/'+r),exist_ok=True); shutil.copy2(LIVE+'/'+r,bk+'/'+r)
    for r in MOD+NEW:
        os.makedirs(os.path.dirname(LIVE+'/'+r),exist_ok=True); shutil.copyfile(SCR+'/real/🔬️slice-lab/'+r,LIVE+'/'+r)
    print('applied')
else:
    for r in MOD: shutil.copyfile(bk+'/'+r,LIVE+'/'+r)
    for r in NEW:
        if os.path.exists(LIVE+'/'+r): os.remove(LIVE+'/'+r)
    for d in ['🧊️frozen','🧪️tests/🧊️frozen']:
        try: os.rmdir(LIVE+'/'+d)
        except OSError: pass
    print('restored')
