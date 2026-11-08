import re,sys
SIG=re.compile(r"\)\s*->\s*semio_framework_value::RetainedWorkOutcome<([\w:]+),semio_os_kernel::NativeSnapshotFailure>\s*\{")
HEAD=re.compile(r"semio_os_kernel::native_snapshot_turn\(\|work\|\{")
CALL=re.compile(r"work\.take\(store::(decode|encode)_sqlite_snapshot_record_native\(")
def match(s,i,o,c):
    d=0
    while i<len(s):
        ch=s[i]
        if ch=='"':
            i+=1
            while s[i]!='"':
                i+=2 if s[i]=='\\' else 1
        elif ch==o:d+=1
        elif ch==c:
            d-=1
            if d==0:return i
        i+=1
    raise Exception('unbalanced')
def split_top(s,sep):
    out=[];d=0;cur=0;i=0;pipe=False
    while i<len(s):
        ch=s[i]
        if ch=='|' and d==0 and sep==',':
            if pipe:pipe=False
            elif s[cur:i].strip() in ('','move'):pipe=True
        if ch=='"':
            i+=1
            while s[i]!='"':i+=2 if s[i]=='\\' else 1
        elif ch in '([{':d+=1
        elif ch in ')]}':d-=1
        elif ch==sep and d==0 and not pipe:
            out.append(s[cur:i]);cur=i+1
        i+=1
    out.append(s[cur:]);return out
def port(text):
    n=0;pos=0
    while True:
        m=SIG.search(text,pos)
        if not m:break
        kind=m.group(1)
        out_ty=kind if kind!='Self' else 'Self'
        sig_new=",output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<%s>>)->store::os_store::serialization::SnapshotReceivingOutcome {"%out_ty
        body_open=m.end()-1
        body_close=match(text,body_open,'{','}')
        body=text[body_open+1:body_close]
        h=HEAD.match(body)
        if not h:
            pos=m.end();print('SKIP shape',text[:0],file=sys.stderr);continue
        inner_open=body_open+1+h.end()-1
        inner_close=match(text,inner_open,'{','}')
        inner=text[inner_open+1:inner_close]
        after=text[inner_close+1:body_close]
        c=CALL.search(inner)
        if not c or after.strip()!=')' :
            pos=m.end();print('SKIP complex',file=sys.stderr);continue
        pre=inner[:c.start()]
        call_open=c.end()-1
        call_close=match(inner,call_open,'(',')')
        args=inner[call_open+1:call_close]
        tail=inner[call_close+1:]
        if tail.strip()!=')' and tail.strip()!=')':
            pos=m.end();print('SKIP tail',repr(tail[:40]),file=sys.stderr);continue
        parts=split_top(args,',')
        if len(parts)!=6:
            pos=m.end();print('SKIP args',len(parts),file=sys.stderr);continue
        cl=parts[3].strip()
        parts[3]=' &mut '+cl
        parts.append('output')
        stm=[s for s in split_top(pre,';') if s.strip()]
        keep=[];chk=[]
        for s in stm:
            t=s.strip()
            if t.startswith('let ') and '?' not in t:keep.append(s)
            else:chk.append(s)
        pre_code=';'.join(keep)+(';' if keep else '')
        if chk:
            pre_code+="if let Err(error)=(||->Result<(),semio_framework_value::ValueError>{"+';'.join(chk)+";Ok(())})(){return semio_framework_value::RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(error)),work:0}}"
        newbody=pre_code+"store::%s_sqlite_snapshot_record_native(%s)"%(c.group(1),','.join(parts))
        new=text[:m.start()]+sig_new+newbody+"}"
        pos=len(new)
        text=new+text[body_close+1:]
        n+=1
    return text,n
if __name__=='__main__':
    for f in sys.argv[1:]:
        t=open(f).read();r,n=port(t)
        if n:open(f,'w').write(r)
        print(f[-60:],n)
