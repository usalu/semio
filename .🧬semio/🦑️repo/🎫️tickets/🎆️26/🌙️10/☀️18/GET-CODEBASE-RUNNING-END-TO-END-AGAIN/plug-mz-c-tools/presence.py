import re,subprocess,sys
dry="--dry" in sys.argv
files=[f for f in subprocess.run(["rg","-l",r"ArtifactPresenceSnapshot for \w+ *\{\s*\}","-g","*.rs","✏️s"],capture_output=True,text=True).stdout.split("\n") if f]
IMPL='''impl store::ArtifactPresenceSnapshot for {n} {{
    fn decode_presence_native(bytes: &[u8], owner: &mut store::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, semio_framework_value::ValueError> {{
        store::decode_presence_json(bytes, owner)
    }}
}}'''
ADD=["semio_framework_value_derive::NativeReceive","semio_framework_value::RetireOwned"]
for f in files:
    s=open(f,encoding="utf-8").read(); o=s
    for m in list(re.finditer(r"impl (?:store::)?ArtifactPresenceSnapshot for (\w+) *\{\s*\}",s)):
        n=m.group(1)
        sm=re.search(r"((?:#\[[^\n]*\]\n)*)pub struct "+n+r"\b",s)
        if not sm: print("NOSTRUCT",f[-70:],n); continue
        block=sm.group(1)
        dm=re.search(r"#\[derive\(([^\n]*)\)\]",block)
        if not dm: print("NODERIVE",f[-70:],n); continue
        d=dm.group(1)
        for a in ADD:
            short=a.split("::")[-1]
            if short not in d: d+=", "+a
        newblock=block.replace(dm.group(0),"#[derive("+d+")]")
        s=s.replace(block+"pub struct "+n,newblock+"pub struct "+n,1) if block else s
        s=re.sub(r"impl (?:store::)?ArtifactPresenceSnapshot for "+n+r" *\{\s*\}",lambda _:IMPL.format(n=n),s,count=1)
    if s!=o:
        print("fix",f[-80:])
        if not dry: open(f,"w",encoding="utf-8").write(s)
