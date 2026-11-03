import sys,re
log,crate=sys.argv[1],sys.argv[2]
L=open(log,errors='replace').read().split('\n')
end=next(i for i,l in enumerate(L) if l.startswith('error: could not compile `%s`'%crate))
start=max(i for i,l in enumerate(L[:end]) if re.match(r'\s*(Checking|Compiling) '+re.escape(crate)+' ',l))
blk=L[start:end]
out=[];i=0
while i<len(blk):
    if re.match(r'error(\[|:)',blk[i]):
        out.append(blk[i][:200]);j=i+1
        while j<len(blk) and not blk[j].startswith(('error','warning')) :
            t=blk[j]
            if re.search(r'-->|^\d+ *\|',t): out.append(re.sub(r'/Users/ueli/Documents/semio/','',t)[:330])
            j+=1
        i=j
    else: i+=1
print('\n'.join(out))
