import re,sys
def fix(path):
    s=open(path).read()
    s=s.replace('fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) ->','fn advance(&mut self, _body: usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, _receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<\'_>) ->')
    out=[];i=0
    pat=re.compile(r'impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<([^,>]+(?:<[^>]*>)?), ([^>]+)> for [A-Za-z0-9_<>, ]+ \{')
    pos=0
    result=''
    while True:
        m=pat.search(s,pos)
        if not m:
            result+=s[pos:];break
        snap,mut=m.group(1).strip(),m.group(2).strip()
        # find begin_demand fn
        b=s.index('    fn begin_demand(',m.end())
        # next factory impl boundary guard
        depth=0;j=s.index('{',b)
        k=j
        while True:
            c=s[k]
            if c=='{':depth+=1
            elif c=='}':
                depth-=1
                if depth==0:break
            k+=1
        body=s[b:k+1]
        pm=re.search(r'size_of::<(.+?)>\(\)',body)
        prep=pm.group(1)
        if 'fn begin_copy_byte_demand' in s[m.end():k+400]:
            result+=s[pos:k+2];pos=k+2;continue
        ins='\n\n    fn begin_copy_byte_demand(&self, mutation: &%s, lane: store::os_store::store_schema::HistoryLane) -> Result<usize, semio_framework_value::ValueError> {\n        self.begin_demand(mutation, lane)?;\n        size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<%s, %s, %s>>().checked_add(size_of::<%s>()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original preparation header quote overflow"))\n    }'%(mut,snap,mut,mut,prep)
        result+=s[pos:k+1]+ins
        pos=k+1
    open(path,'w').write(result)
for p in sys.argv[1:]:
    fix(p);print('fixed',p[-40:])
