import re,os,sys
files=sys.argv[1:]
bad=0
for f in files:
    s=open(f,encoding='utf8').read()
    for m in re.finditer(r'\]\(([^)#\s]+)(#[^)]*)?\)',s):
        t=m.group(1)
        if re.match(r'[a-z]+:',t): continue
        p=os.path.normpath(os.path.join(os.path.dirname(f),t))
        if not os.path.exists(p):
            print('DEAD',f,t); bad+=1
    for m in re.finditer(r'\]\(([^)#\s]+)?#([^)]*)\)',s):
        pass
    if len(f.encode())>255: print('LONG',f)
print('dead links:',bad)
