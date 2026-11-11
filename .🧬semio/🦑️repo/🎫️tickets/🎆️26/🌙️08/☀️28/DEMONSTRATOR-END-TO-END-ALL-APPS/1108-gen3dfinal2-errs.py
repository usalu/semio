import re,sys,os
txt=open(sys.argv[1]).read().split('\n')
pkg=sys.argv[2] if len(sys.argv)>2 else ''
root='/Users/ueli/Documents/semio/'
i=0
while i<len(txt):
    l=txt[i]
    if l.startswith('error') and not l.startswith('error: could'):
        msg=l[:300]
        j=i+1
        while j<len(txt) and not txt[j].strip().startswith('-->'): j+=1
        if j>=len(txt): break
        m=re.search(r'--> (.*?):(\d+):(\d+)',txt[j])
        f,ln,col=m.group(1),int(m.group(2)),int(m.group(3))
        f=os.path.normpath(root+f)
        if pkg in f or not pkg:
            try:
                s=open(f).read().split('\n')[ln-1]
                print(msg); print('  ',f.split('/')[-2]+'/'+f.split('/')[-1],ln,col,'::',s[max(0,col-1-60):col-1+260]); 
                k=j+1
                while k<len(txt) and not txt[k].startswith('error') and not txt[k].startswith('warning'):
                    if re.match(r'\s*= (note|help)|\s*\|\s+[\^-]+ ',txt[k]): print('     ',txt[k].strip()[:250])
                    k+=1
                print()
            except Exception as e: print(msg,f,e)
        i=j
    i+=1
