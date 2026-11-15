import subprocess,pickle,json
R='/Users/Shared/semio/'
def git(*a):return subprocess.run(['git','--no-optional-locks','-c','core.quotepath=false','-C',R[:-1]]+list(a),capture_output=True).stdout
commits=[l.split() for l in git('log','--since=14 days ago','--format=%H %P').decode().splitlines()][::-1]
meta,keys,tf=pickle.load(open('keys.pkl','rb'))
fa=set()
for t,fs in tf.items():fa.update(fs)
jobs=[]
for p_c in commits:
    if len(p_c)<2:continue
    c,par=p_c
    names=[x for x in git('diff-tree','-r','--no-commit-id','--name-only','-z','--no-renames',par,c).decode().split('\0') if x]
    for f in names:
        if f.endswith('/Cargo.toml') and f in fa:jobs.append({'c':c[:10],'path':f,'a':par+':'+f,'b':c+':'+f})
json.dump(jobs,open('mjobs.json','w'));print(len(jobs))
