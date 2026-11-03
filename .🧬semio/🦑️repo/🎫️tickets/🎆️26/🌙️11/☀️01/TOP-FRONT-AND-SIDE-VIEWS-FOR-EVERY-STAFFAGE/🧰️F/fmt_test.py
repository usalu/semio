import json,glob,sys
sys.stdout.reconfigure(encoding='utf8')
for f in sorted(glob.glob('orig/*.json')):
    raw=open(f,encoding='utf8',newline='').read()
    d=json.loads(raw)
    for ind in (1,):
        out=json.dumps(d,ensure_ascii=False,indent=ind)
        for tail in ('','\n'):
            if out+tail==raw: print(f,'IDENTICAL indent',ind,repr(tail)); break
        else:
            a=(out+'\n').splitlines(); b=raw.splitlines()
            n=sum(1 for x,y in zip(a,b) if x!=y); print(f,'diff lines',n,len(a),len(b)); 
            for x,y in zip(a,b):
                if x!=y: print(repr(x[:100]),repr(y[:100])); break
