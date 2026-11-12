import re,sys,os
log,start,crate=sys.argv[1],sys.argv[2],sys.argv[3]
txt=open(log,errors='replace').read().split('\n')
out=[];i=0
while i<len(txt):
    l=txt[i]
    if l.startswith('error') and 'could not compile' not in l and 'aborting' not in l:
        msg=l;j=i+1
        while j<len(txt) and not txt[j].lstrip().startswith('-->'): j+=1
        if j<len(txt):
            m=re.search(r'--> (.*?):(\d+):(\d+)',txt[j])
            if m and crate in m.group(1):
                p=m.group(1);ln=int(m.group(2));col=int(m.group(3))
                try:
                    src=open(p).read().split('\n')[ln-1]
                    out.append((msg[:110],os.path.basename(os.path.dirname(p)),ln,col,src[max(0,col-1):col+170]))
                except Exception as e: out.append((msg,p,ln,col,str(e)))
                # collect label lines
                k=j+1;lab=[]
                while k<len(txt) and txt[k].strip()!='' and not txt[k].startswith('error') and len(lab)<8:
                    if re.search(r'\^|expected|found|help',txt[k]): lab.append(txt[k].strip()[:200])
                    k+=1
                out[-1]=out[-1]+(lab,)
        i=j
    i+=1
for o in out:
    print(o[0]);print('  ',o[1],o[2],o[3]);print('  SRC:',o[4]);
    for x in o[5] if len(o)>5 else []: print('   |',x)
