import hashlib,json,os,shutil
base=os.path.dirname(os.path.abspath(__file__)); real=os.path.join(base,'real','🔬️slice-lab')
LIVE='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab'
shutil.rmtree(real,ignore_errors=True); shutil.copytree(LIVE,real)
files=json.load(open(os.path.join(base,'files.json')))
sha=lambda p:hashlib.sha256(open(p,'rb').read()).hexdigest()
before={}
for r in files['overlay']:
    src=os.path.join(base,'new',r)
    if os.path.exists(os.path.join(LIVE,r)): before[r]=sha(os.path.join(LIVE,r))
    shutil.copyfile(src,os.path.join(real,r))
for r in files.get('edit',[]): before[r]=sha(os.path.join(LIVE,r))
def edit(rel,pairs):
    p=os.path.join(real,rel); s=open(p,encoding='utf8').read()
    for a,b in pairs:
        assert s.count(a)==1,(rel,a[:70],s.count(a)); s=s.replace(a,b)
    open(p,'w',encoding='utf8').write(s)
if os.path.exists(os.path.join(base,'extra.py')): exec(open(os.path.join(base,'extra.py'),encoding='utf8').read())
json.dump(before,open(os.path.join(base,'before.json'),'w'))
print(before)
