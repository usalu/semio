import sys,re,os
S=sys.argv[1]; names=sys.argv[2].split(',')
for root,_,fs in os.walk(S):
    if '/📦️packages' in root or 'node_modules' in root: continue
    for f in fs:
        if f!='🦀️.rs': continue
        p=os.path.join(root,f)
        t=open(p,encoding='utf8').read()
        code=re.sub(r'//[^\n]*','',t)
        for n in names:
            uses=[m for m in re.finditer(r'(?<![\w:.])'+re.escape(n)+r'\b',code)]
            if not uses: continue
            if re.search(r'\b(?:fn|const|static|struct|enum|trait|type)\s+'+re.escape(n)+r'\b',code): continue
            if re.search(r'\buse\b[^;]*\b'+re.escape(n)+r'\b',code): continue
            if re.search(r'\buse\b[^;]*::\*\s*;',code) and False: pass
            print(n,'<-',p.split('any/')[-1])
