import re,subprocess,sys
R="/Users/ueli/Documents/semio"
V="semio_framework_value"
TRIO={
"initial_state_demands":f"fn initial_state_demands()->Result<{V}::RetirementDemand,{V}::ValueError>{{Ok({V}::RetirementDemand{{copy_bytes:2*std::mem::size_of::<Self::State>(),depth:1,..Default::default()}})}}",
"initial_state":f"fn initial_state(grant:{V}::RetainedCloneGrant)->Result<(Self::State,{V}::RetainedCloneProgress),{V}::ValueError>{{let d=Self::initial_state_demands()?;if !{V}::retained_clone::retained_clone_grant_admits(grant,d){{return Err({V}::ValueError::literal({V}::ValueRefusalKind::WorkLimit,\"original window config birth is not funded\"))}}Ok((Self::State::default(),{V}::RetainedCloneProgress{{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}))}}",
"initial_state_pack_source":f"fn initial_state_pack_source(state:&Self::State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),{V}::ValueError>{{Ok((state,<Self::State as semio_framework_dsl_record::BorrowedDslRecord>::RECORD))}}",
}
def block_end(s,i):
    d=0;n=len(s)
    while i<n:
        c=s[i]
        if c=='"':
            i+=1
            while s[i]!='"':i+=2 if s[i]=='\\' else 1
        elif s.startswith("//",i):i=s.index("\n",i)
        elif c=="'" and (s[i+2:i+3]=="'" or (s[i+1]=="\\" and s[i+3:i+4]=="'")):i+=3 if s[i+1]!="\\" else 4;continue
        elif c=="{":d+=1
        elif c=="}":
            d-=1
            if d==0:return i
        i+=1
    raise Exception("unbalanced")
files=subprocess.run(["git","grep","-l","WindowConfigOwner for","--","*.rs"],cwd=R,capture_output=True,text=True).stdout.split("\n")
tot=0
for f in filter(None,files):
    p=f"{R}/{f}";s=open(p,encoding="utf8").read();out=[];pos=0
    for m in re.finditer(r'impl\b[^{;]*?\bWindowConfigOwner\s+for\b[^{]*\{',s):
        if m.start()<pos:continue
        end=block_end(s,m.end()-1);body=s[m.end():end]
        # only top-level fns of this impl: crude check
        missing=[k for k in TRIO if not re.search(r'\bfn\s+'+k+r'\b',body)]
        if not missing:continue
        out.append((m.end(),"\n    "+"\n    ".join(TRIO[k] for k in missing)));tot+=len(missing);pos=end
    for at,txt in reversed(out):s=s[:at]+txt+s[at:]
    if out:open(p,"w",encoding="utf8").write(s);print(len(out),"impls",f[-70:])
print("fns inserted",tot)
