import re,subprocess,sys,os,tempfile
apply=len(sys.argv)>1 and sys.argv[1]=="apply"
names="Block2dMutation Block3dMutation Block5dMutation EquationMutation FormMutation LowpolyMutation ModelMutation ProcedureMutation ProgramMutation RasterMutation SequenceMutation VcsDemoMutation WriterMutation Wfc2dMutation".split()
for n in names:
    fs=[f for f in subprocess.run(["grep","-rIlE",rf"OpBinary for {n}\b","--include=🦀️.rs","--exclude-dir=.cache","--exclude-dir=.🧬semio","--exclude-dir=🎫️tickets","--exclude-dir=node_modules","✏️s/🔌️plugins"],capture_output=True,text=True).stdout.split("\n") if f]
    fs=[f for f in fs if "🧪️tests" not in f]
    if len(fs)!=1: print("SKIP",n,fs); continue
    f=fs[0]; t=open(f,encoding="utf8").read()
    if "artifact_persisted_binary_operation" in t or "OpBinaryReceiving" in t or "operation_receiving!" in t: print("ALREADY",n); continue
    pre="dsl::" if "dsl::" in t else "semio_os_kernel::" if "semio_os_kernel::" in t else "protocol::"
    lines=t.split("\n"); out=[]; done=False
    for l in lines:
        m=re.match(rf"^(\s*)impl (?:protocol::)?OpBinary for {n}\b",l)
        if m and not done:
            ind=m.group(1)
            out+= [f"{ind}{pre}original_decoded_operation_receiving!({n});",f"{ind}{pre}artifact_persisted_binary_operation!({n});",""]; done=True
        out.append(l)
    print("ADD",n,pre,f[-60:])
    if apply and done:
        fd,p=tempfile.mkstemp(dir=os.path.dirname(f)); os.write(fd,"\n".join(out).encode()); os.close(fd); os.chmod(p,os.stat(f).st_mode); os.replace(p,f)
