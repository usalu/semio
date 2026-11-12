import json,re,subprocess,sys,os
T=os.path.dirname(os.path.abspath(__file__))
m=[]
for f in ('rootsplit-c-mapping.json',):
    m+=json.load(open(T+'/🗑️generated/'+f))
mp={t:p for t,p,d in m}
alt='|'.join(re.escape(k) for k in sorted(mp,key=lambda k:-len(k)))
rx=re.compile(r'workspace:('+alt+r')(?![A-Za-z0-9_-])')
files=subprocess.run(['git','-c','core.quotepath=off','grep','-I','-l','-E','workspace:('+alt+')','--','.',':!*tickets*',':!bun.lock'],capture_output=True,text=True).stdout.split('\n')
n=0
for f in files:
    if not f or 'collect-errors' in f: continue
    s=open(f,encoding='utf8').read()
    s2=rx.sub(lambda x: mp[x.group(1)]+':'+x.group(1),s)
    if s2!=s:
        n+=1; print('edit',f[-90:],len(rx.findall(s)))
        if 'apply' in sys.argv: open(f,'w',encoding='utf8').write(s2)
print(n)
