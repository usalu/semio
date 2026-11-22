import re,sys
p=sys.argv[1]
s=open(p).read()
def match(s,i):
    d=0
    for j in range(i,len(s)):
        c=s[j]
        if c in '([{':d+=1
        elif c in ')]}':
            d-=1
            if d==0:return j
def split_args(a):
    out=[];d=0;cur=''
    for c in a:
        if c in '([{':d+=1
        elif c in ')]}':d-=1
        if c==',' and d==0:out.append(cur);cur=''
        else:cur+=c
    out.append(cur);return [x.strip() for x in out]
def conv(kind,body):
    body=body.strip()
    m=re.match(r'(.*?)::(decode|encode)_sqlite_snapshot_native\(',body)
    if kind=='decoding' and m and m.group(2)=='decode':
        o=body.index('(',m.end()-1);e=match(body,o);a=split_args(body[o+1:e])
        assert a[-1]=='native_owner' and e==len(body)-1,body
        return 'original_owners::decode_native::<%s>(%s)'%(m.group(1),','.join(a[:-1])),0
    if kind=='decoding' and body.startswith('(owner.export)('):
        o=body.index('(',1);e=match(body,o);a=split_args(body[o+1:e]);assert a[-1]=='native_owner'
        return 'original_owners::export_sqlite(&owner,%s)'%','.join(a[:-1]),1
    if kind=='encoding' and body.startswith('(owner.import)('):
        o=body.index('(',1);e=match(body,o);a=split_args(body[o+1:e]);assert a[-1]=='native_owner'
        m2=re.match(r'&mut Some\((.*)\)$',a[2]);assert m2,a
        return 'original_owners::import_sqlite(&owner,%s,%s,%s)'%(a[0],a[1],','.join([m2.group(1)]+a[3:-1])),1
    m=re.match(r'([A-Za-z_][\w]*)\.encode_sqlite_snapshot_native\(',body)
    if kind=='encoding' and m:
        o=body.index('(',m.end()-1);e=match(body,o);a=split_args(body[o+1:e]);assert a[-1]=='native_owner'
        return 'original_owners::encode_native(&%s,%s)'%(m.group(1),','.join(a[:-1])),0
    raise Exception(kind+' '+body[:120])
for kind in ('decoding','encoding'):
    key='original_owners::with_%s(|native_owner|'%kind
    while True:
        i=s.find(key)
        if i<0:break
        o=i+len('original_owners::with_%s'%kind);e=match(s,o)
        body=s[o+len('(|native_owner|'):e]
        new,dropval=conv(kind,body)
        tail=s[e+1:]
        if dropval and tail.startswith('.unwrap().value'):tail='.unwrap()'+tail[len('.unwrap().value'):]
        s=s[:i]+new+tail
open(p,'w').write(s)
