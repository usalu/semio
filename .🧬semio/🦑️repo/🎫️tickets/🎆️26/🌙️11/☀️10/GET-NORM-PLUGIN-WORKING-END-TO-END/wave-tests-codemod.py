import sys,re
def match_paren(s,i):
    d=0;j=i;instr=False
    while j<len(s):
        c=s[j]
        if instr:
            if c=='\\': j+=1
            elif c=='"': instr=False
        elif c=='"': instr=True
        elif c=='\'' and j+2<len(s) and s[j+2]=="'" : j+=2
        elif c in '([{': d+=1
        elif c in ')]}':
            d-=1
            if d==0: return j
        j+=1
    raise Exception('unbalanced')
def split_args(a):
    out=[];d=0;cur='';instr=False;j=0
    while j<len(a):
        c=a[j]
        if instr:
            cur+=c
            if c=='\\': j+=1;cur+=a[j]
            elif c=='"': instr=False
        elif c=='"': instr=True;cur+=c
        elif c=='|' and d==0 and cur.strip() in ('','&mut','move'):
            k=a.index('|',j+1);cur+=a[j:k+1];j=k
        else:
            if c in '([{': d+=1
            if c in ')]}': d-=1
            if c==',' and d==0: out.append(cur);cur=''
            else: cur+=c
        j+=1
    out.append(cur);return [x.strip() for x in out]
def conv(s):
    n=0;rest=[]
    for kind in('with_decoding','with_encoding'):
        key='original_owners::%s(|native_owner|'%kind
        while True:
            i=s.find(key)
            if i<0: break
            open_i=i+len('original_owners::%s'%kind)
            close=match_paren(s,open_i)
            body=s[i+len(key):close]
            new=None
            m=re.match(r'^(.*?)::decode_sqlite_snapshot_native\(',body)
            if kind=='with_decoding' and m and body.endswith(')'):
                ty=m.group(1);a=split_args(body[m.end():-1])
                if a[-1]=='native_owner': new='original_owners::decode_native::<%s>(%s)'%(ty,','.join(a[:-1]))
            elif kind=='with_decoding' and re.match(r'^\(\w+\.export\)\(',body):
                pv=re.match(r'^\((\w+)\.export\)\(',body);a=split_args(body[pv.end():-1])
                if a[-1]=='native_owner': new='original_owners::export_sqlite(%s,%s)'%(pv.group(1),','.join(a[:-1]))
            elif kind=='with_encoding' and '.encode_sqlite_snapshot_native(' in body:
                e,r=body.split('.encode_sqlite_snapshot_native(',1);a=split_args(r[:-1])
                if a[-1]=='native_owner' and re.match(r'^[\w.]+$',e): new='original_owners::encode_native(&%s,%s)'%(e,','.join(a[:-1]))
            elif kind=='with_encoding' and re.match(r'^\(\w+\.import\)\(',body):
                pv=re.match(r'^\((\w+)\.import\)\(',body);a=split_args(body[pv.end():-1])
                if a[-1]=='native_owner':
                    mm=re.match(r'^&mut Some\((.*)\)$',a[2])
                    if mm: new='original_owners::import_sqlite(%s,%s,%s,%s,%s,%s)'%(pv.group(1),a[0],a[1],mm.group(1),a[3],a[4])
            if new is None:
                rest.append(body[:160]);s=s[:i]+'@@OO@@'+s[i+len('original_owners::'):]; continue
            s=s[:i]+new+s[close+1:];n+=1
    return s,n,rest
for p in sys.argv[1:]:
    s=open(p).read();s2,n,rest=conv(s);s2=s2.replace('@@OO@@','original_owners::')
    s2=s2.replace('with_decoding(|native_owner|','original_owners::with_decoding(|native_owner|') if False else s2
    open(p,'w').write(s2);print(p[-60:],n,'converted',len(rest),'left');[print('   LEFT:',r) for r in rest]
