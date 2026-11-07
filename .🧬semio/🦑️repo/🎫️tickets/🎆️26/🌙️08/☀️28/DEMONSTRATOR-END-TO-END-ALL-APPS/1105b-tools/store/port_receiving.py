import re,sys
sys.path.insert(0,__file__.rsplit("/",1)[0])
from port_native import match_close
FK="semio_os_kernel::NativeSnapshotFailure"
def recv_file(s):
    n=0
    def sub_proj(m):
        nonlocal n;n+=1
        return f"{m.group(1)}->semio_framework_value::RetainedWorkOutcome<{m.group(2)},{FK}>{{owner.receive_sqlite_snapshot"
    s2=re.sub(r"(fn (?:project|reconstruct)_receiving\([^{]*?\))->Result<([^{]*?)>\{owner\.receive_sqlite_snapshot",sub_proj,s)
    def sub_val(m):
        nonlocal n;n+=1
        return f"{m.group(1)}->std::result::Result<(),{FK}>{{owner.receive_sqlite_snapshot"
    s2=re.sub(r"(fn validate_(?:decoding|encoding)\([^{]*?\))->Result<\(\)>\{owner\.receive_sqlite_snapshot",sub_val,s2)
    # append .result before closing brace of validate fns
    out=[];i=0
    for m in re.finditer(r"fn validate_(?:decoding|encoding)\([^{]*?\)->std::result::Result<\(\),"+re.escape(FK)+r">\{",s2):
        pass
    res=[];k=0
    for m in re.finditer(r"fn validate_(?:decoding|encoding)\([^{]*?\)->std::result::Result<\(\),"+re.escape(FK)+r">\{",s2):
        b=m.end()-1;e=match_close(s2,b,"{","}")
        res.append(s2[k:e]);res.append(".result");k=e
    res.append(s2[k:])
    return "".join(res),n
def impl_file(s):
    n=0
    for name,ret_old in (("to_sqlite_database_receiving",r"Result<SqliteDatabase,ValueError>"),("from_sqlite_database_receiving",r"Result<Self,ValueError>")):
        def sub(m):
            nonlocal n;n+=1
            inner=m.group(2).replace("Result<","").rsplit(",ValueError>",1)[0]
            return f"{m.group(1)}->semio_framework_value::RetainedWorkOutcome<{inner},{FK}>{{receiving::"
        s=re.sub(r"(fn "+name+r"\([^{]*?\))->("+ret_old+r")\{receiving::",sub,s)
    out=[];i=0
    pat=re.compile(r"\{let result=\(\|\|\{")
    while True:
        m=pat.search(s,i)
        if not m: out.append(s[i:]);break
        # only inside validate_* impl heads
        head=s[max(0,m.start()-900):m.start()]
        if "receiving::validate_" not in s[m.start():m.start()+2500] or "fn validate_sqlite_snapshot_subset_" not in head.rsplit("fn ",1)[-1]+"fn validate_sqlite_snapshot_subset_" and False:
            out.append(s[i:m.end()]);i=m.end();continue
        ib=m.end()-1;ie=match_close(s,ib,"{","}")
        tail=")();result.map_err(|error|"
        j=ie+1
        mt=re.match(r"\)\(\);result\.map_err\(\|error\|([\w:]*)IoError::from_value_error\(error\.with_retained_progress\(owner\.progress\(\)\)\)\)\}",s[j:])
        if not mt: out.append(s[i:m.end()]);i=m.end();continue
        pre=mt.group(1)
        inner=s[ib+1:ie]
        # return Err(X) -> return Err(IoError::from_value_error(X))
        res=[];k=0
        for r in re.finditer(r"return Err\(",inner):
            if r.start()<k: continue
            e=match_close(inner,r.end()-1,"(",")")
            res.append(inner[k:r.end()]);res.append(f"{pre}IoError::from_value_error({inner[r.end():e]})");k=e
        res.append(inner[k:])
        inner="".join(res)
        inner=re.sub(r"(receiving::validate_(?:decoding|encoding)\([^;]*?\))\.map\(([\w:]+IoOutcome::clean)\)\s*$",lambda mm:f"{mm.group(1)}.map({mm.group(2)}).map_err({pre}IoError::from)",inner.rstrip())
        out.append(s[i:m.start()]);out.append("{"+inner+"}");i=j+mt.end();n+=1
    return "".join(out),n
def run(path,write):
    s=open(path,encoding="utf8").read()
    if "receive_sqlite_snapshot(" in s: t,n=recv_file(s)
    elif "receiving::project_receiving" in s or "receiving::validate_" in s: t,n=impl_file(s)
    else: return 0
    if n and write: open(path,"w",encoding="utf8").write(t)
    return n
if __name__=="__main__":
    w=sys.argv[1]=="write"
    for p in sys.argv[2:]:
        n=run(p,w)
        print(n,p[-85:])
