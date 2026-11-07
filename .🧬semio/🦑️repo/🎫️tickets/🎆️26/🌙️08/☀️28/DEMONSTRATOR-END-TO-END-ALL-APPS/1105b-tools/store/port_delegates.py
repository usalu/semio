import re,sys
sys.path.insert(0,__file__.rsplit("/",1)[0])
from port_native import match_close,OWNER
SIG=re.compile(r"fn (decode|encode)_sqlite_snapshot_native\b")
CALLPAT=r"[\w:.]*\b(?:encode|decode)\w*\([^;]*?\)"
CALL=re.compile(r"^"+CALLPAT+r"$",re.S)
def split_top(body):
    parts=[];d=0;k=0;j=0
    while j<len(body):
        ch=body[j]
        if ch=='"':
            j+=1
            while body[j]!='"':
                if body[j]=='\\': j+=1
                j+=1
        elif ch in "({[": d+=1
        elif ch in ")}]": d-=1
        elif ch==";" and d==0: parts.append(body[k:j+1]);k=j+1
        j+=1
    parts.append(body[k:]);return parts
def into_returns(t):
    out=[];i=0
    for m in re.finditer(r"return Err\(",t):
        pass
    while True:
        m=re.compile(r"return Err\(").search(t,i)
        if not m: out.append(t[i:]);break
        j=match_close(t,m.end()-1,"(",")")
        out.append(t[i:j+1]);
        if not t.startswith(".into()",j+1): out.append(".into()")
        i=j+1
    return "".join(out)
def port(path,write):
    s=open(path,encoding="utf8").read();out=[];i=0;n=0;skipped=0
    while True:
        m=SIG.search(s,i)
        if not m: out.append(s[i:]);break
        fn=m.start();par=s.index("(",m.end());pe=match_close(s,par,"(",")");brace=s.index("{",pe)
        rtxt=s[pe:brace].replace("\n"," ")
        ret=re.search(r"->\s*(?:[\w:]*::)?Result<\s*(.+?)\s*,\s*(?:[\w:]*::)?ValueError\s*>\s*$",rtxt) or re.search(r"->\s*Result<\s*([^,]+?)\s*>\s*$",rtxt)
        be=match_close(s,brace,"{","}");body=s[brace+1:be];om=OWNER.search(s[par:pe])
        if "RetainedWorkOutcome" in rtxt or not ret or not om:
            out.append(s[i:be+1]);i=be+1;continue
        prefix=om.group(1) or "semio_os_kernel"
        parts=split_top(body);tail=parts[-1].strip();new=None
        tm=re.search(r"\}\s*("+CALLPAT+r")\s*$",tail,re.S)
        if tm and not CALL.match(tail):
            parts=parts[:-1]+[tail[:tm.start(1)]+";",tm.group(1)];tail=tm.group(1)
        if tail and CALL.match(tail):
            pre="".join(parts[:-1])
            new="{"+body+"}" if not pre.strip() else f"{{{prefix}::native_snapshot_turn(|work|{{{pre}work.take({tail})}})}}"
        else:
            hit=None
            for idx,p in enumerate(parts[:-1]):
                mm=re.match(r"^(\s*let\s+[\w ]+=\s*)("+CALLPAT+r")\?;\s*$",p,re.S)
                if mm: hit=(idx,mm);break
            if hit:
                idx,mm=hit;parts[idx]=mm.group(1)+"work.take("+mm.group(2)+")?;"
                new=f"{{{prefix}::native_snapshot_turn(|work|{{{''.join(parts)}}})}}"
        if new is None: out.append(s[i:be+1]);i=be+1;skipped+=1;continue
        new=into_returns(new)
        newret=f"->semio_framework_value::RetainedWorkOutcome<{ret.group(1)},{prefix}::NativeSnapshotFailure> "
        out.append(s[i:pe+1]);out.append(newret);out.append(new);i=be+1;n+=1
    t="".join(out)
    if write and n: open(path,"w",encoding="utf8").write(t)
    return n,skipped
if __name__=="__main__":
    w=sys.argv[1]=="write"
    for p in sys.argv[2:]:
        n,k=port(p,w)
        if n or k: print(n,k,p[-80:])
