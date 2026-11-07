import re,sys
SIG=re.compile(r"fn \w*(?:decode|encode)\w*\b")
CALL=re.compile(r"[\w:]*(?:de|en)code_sqlite_snapshot_record_native\s*[(<]")
OWNER=re.compile(r"(?:([\w:]+)::)?NativeSnapshot(?:Decode|Encode)Owner")
def match_close(s,i,o,c):
    d=0;j=i
    while True:
        ch=s[j]
        if ch=='"':
            j+=1
            while s[j]!='"':
                if s[j]=='\\': j+=1
                j+=1
        elif ch=="'" and s[j+2:j+3]=="'" : j+=2
        elif ch==o: d+=1
        elif ch==c:
            d-=1
            if d==0: return j
        j+=1
def port(path,write):
    s=open(path,encoding="utf8").read();out=[];i=0;n=0;skipped=[]
    while True:
        m=SIG.search(s,i)
        if not m: out.append(s[i:]);break
        fn=m.start();par=s.index("(",m.end());pe=match_close(s,par,"(",")")
        brace=s.index("{",pe);head=s[fn:brace]
        rtxt=s[pe:brace].replace("\n"," ");ret=re.search(r"->\s*(?:[\w:]*::)?Result<\s*(.+?)\s*,\s*(?:[\w:]*::)?ValueError\s*>\s*$",rtxt) or re.search(r"->\s*Result<\s*([^,]+?)\s*>\s*$",rtxt)
        be=match_close(s,brace,"{","}");body=s[brace+1:be]
        om=OWNER.search(s[par:pe])
        c=None
        for c in CALL.finditer(body): pass
        if ret and om and not c and re.fullmatch(r"\s*[\w:.()]*(?:native::(?:de|en)code|typed\(\)\.(?:de|en)code_sqlite_snapshot_native|(?:de|en)code\w*native\w*)\([^;]*\)\s*",body):
            prefix=om.group(1) or 'semio_os_kernel';newret=f"->semio_framework_value::RetainedWorkOutcome<{ret.group(1)},{prefix}::NativeSnapshotFailure>"
            out.append(s[i:fn]);out.append(s[fn:pe+1]+newret+" ");out.append("{"+body+"}");i=be+1;n+=1;continue
        if not ret or not om or not c:
            out.append(s[i:be+1]);i=be+1;skipped.append((fn,"shape"));continue
        cp=body.index("(",c.end()-1) if body[c.end()-1]!="(" else c.end()-1
        ce=match_close(body,cp,"(",")")
        prefix=om.group(1) or 'semio_os_kernel'
        if body[ce+1:].strip():
            newret=f"->semio_framework_value::RetainedWorkOutcome<{ret.group(1)},{prefix}::NativeSnapshotFailure>"
            nb=[];k=0
            for cm in CALL.finditer(body):
                if cm.start()<k: continue
                cpp=body.index("(",cm.end()-1) if body[cm.end()-1]!="(" else cm.end()-1
                cee=match_close(body,cpp,"(",")")
                nb.append(body[k:cm.start()]);nb.append("work.take("+body[cm.start():cee+1]+")");k=cee+1
            nb.append(body[k:])
            out.append(s[i:fn]);out.append(s[fn:pe+1]+newret+" ");out.append(f"{{{prefix}::native_snapshot_turn(|work|{{{''.join(nb)}}})}}");i=be+1;n+=1;continue
        pre=body[:c.start()];call=body[c.start():ce+1]
        newret=f"->semio_framework_value::RetainedWorkOutcome<{ret.group(1)},{prefix}::NativeSnapshotFailure>"
        sigtxt=s[fn:pe+1]
        # replace the return type text between pe and brace
        newhead=sigtxt+newret+" "
        if pre.strip():
            newbody=f"{{{prefix}::native_snapshot_turn(|work|{{{pre}work.take({call})}})}}"
        else:
            newbody=f"{{{body[:c.start()]}{call}}}"
        out.append(s[i:fn]);out.append(newhead);out.append(newbody);i=be+1;n+=1
    t="".join(out)
    if write and n: open(path,"w",encoding="utf8").write(t)
    return n,skipped
if __name__=="__main__":
    write=sys.argv[1]=="write"
    for p in sys.argv[2:]:
        n,sk=port(p,write);print(("OK " if n and not sk else "PARTIAL " if n else "NONE "),n,sk,p[-90:])
