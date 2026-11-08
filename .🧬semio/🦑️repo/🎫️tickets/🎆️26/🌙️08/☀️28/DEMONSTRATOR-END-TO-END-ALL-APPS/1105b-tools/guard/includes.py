import os,re,sys,unicodedata
roots=sys.argv[1:]
pat=re.compile(r'(include_str|include_bytes|include)!\(\s*"([^"]+)"')
pp=re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
miss=0
for r in roots:
    for d,ds,fs in os.walk(r):
        ds[:]=[x for x in ds if x not in('target','node_modules')]
        for f in fs:
            if not f.endswith('.rs'):continue
            p=os.path.join(d,f)
            try:s=open(p,encoding='utf8').read()
            except Exception:continue
            for m in list(pat.finditer(s))+list(pp.finditer(s)):
                rel=m.group(m.lastindex)
                t=os.path.normpath(os.path.join(d,rel))
                if not os.path.exists(t):
                    miss+=1;print(p,'->',rel)
print('missing',miss)
