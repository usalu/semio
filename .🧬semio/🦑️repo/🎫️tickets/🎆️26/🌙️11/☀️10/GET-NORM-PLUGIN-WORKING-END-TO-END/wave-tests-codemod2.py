import sys,re,importlib.util
spec=importlib.util.spec_from_file_location('c1',sys.argv[0].replace('codemod2','codemod'));
src=open(sys.argv[0].replace('codemod2','codemod')).read().split('for p in sys.argv[1:]')[0]
exec(src)
key='store::decode_sqlite_snapshot_record_native('
for p in sys.argv[1:]:
    s=open(p).read();n=0;pos=0
    while True:
        i=s.find(key,pos)
        if i<0: break
        o=i+len(key)-1;c=match_paren(s,o);a=split_args(s[o+1:c])
        if len(a)!=5: pos=c;print('skip',len(a));continue
        cl=re.sub(r',\s*_body\s*\|','|',a[3],count=1)
        new='original_owners::with_decoding(|native_owner|{let mut output=None;store::decode_sqlite_snapshot_record_native(%s,%s,%s,&mut %s,%s,native_owner,&mut output)}).result'%(a[0],a[1],a[2],cl,a[4])
        s=s[:i]+new+s[c+1:];pos=i+len(new);n+=1
    open(p,'w').write(s);print(p[-40:],n)
