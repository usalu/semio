import re,sys,subprocess
t=open(sys.argv[1]).read().split('\n');flt=sys.argv[2] if len(sys.argv)>2 else ''
seen=set();root='/Users/ueli/Documents/semio/'
cache={}
for i,l in enumerate(t):
    if l.startswith('error') and 'could not compile' not in l:
        for j in range(i+1,i+3):
            m=re.search(r'--> /Users/ueli/Documents/semio/(.*?):(\d+):(\d+)',t[j])
            if m:
                path=m.group(1);
                if flt not in path: break
                import os
                full=os.path.normpath(root+path)
                L=int(m.group(2));c=int(m.group(3))
                key=(full,L,c,l[:60])
                if key in seen:break
                seen.add(key)
                try:
                    src=cache.setdefault(full,open(full).read().split('\n'))[L-1]
                except Exception as e: src='?'
                print(f"{os.path.relpath(full,root)[-60:]}:{L}:{c} {l[:100]}\n    {src[max(0,c-1-50):c-1+80]}")
                break
