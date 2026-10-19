"""cstep: port InteractiveJob / FixedOperationOwner / ArtifactCommandWork impls to the borrowed-cx close API.
usage: port_traits.py [--write] [--diff] listfile|path...
 close_step(&mut self, grant) -> ICS  ==> close_step(&mut self, cx) -> Result<ICS, OriginalTurnFailure> over close_original_granted_step
 InteractiveJob::step ... ValueError  ==> inherent `step_value` + lifting trait method
 ArtifactCommandWork::step ... Fault  ==> closure-lifted Result<_, OriginalTurnFailure>"""
import re,sys,os,difflib
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
from body import transform_body

IMPL=re.compile(r'\bimpl\b(?P<head>[^{;]*?)\b(?P<trait>InteractiveJob|FixedOperationOwner|ArtifactCommandWork)\b(?P<targs><[^{]*?>)?\s+for\b(?P<tail>[^{;]*?)\{',re.S)
JOB='semio_framework_job'

def depth_array(m):
    d=0;out=[0]*(len(m)+1)
    for i,c in enumerate(m):
        out[i]=d
        if c=='{': d+=1
        elif c=='}': d-=1
    out[len(m)]=d
    return out

def children(m,dep,b,e):
    for f in re.finditer(r'\bfn\s+(\w+)',m[b+1:e]):
        pos=b+1+f.start()
        if dep[pos]!=dep[b]+1: continue
        i=b+1+f.end();q=0;bo=-1;n=len(m)
        while i<e:
            c=m[i]
            if c in '([': q+=1
            elif c in ')]': q-=1
            elif c=='{' and q==0: bo=i;break
            elif c==';' and q==0: break
            i+=1
        if bo<0: continue
        be=match_brace(m,bo)
        yield f.group(1),pos,bo,be

def ics_names(s,ret):
    names={'InteractiveJobCloseStep'}
    for a in re.finditer(r'InteractiveJobCloseStep\s+as\s+(\w+)',s): names.add(a.group(1))
    last=ret.split('::')[-1].strip()
    if last: names.add(last)
    return names

def owning_crate(path):
    d=os.path.dirname(os.path.abspath(path))
    while len(d)>1:
        t=os.path.join(d,'📦️packages','🦀️rust','Cargo.toml')
        if os.path.isfile(t): return t
        d=os.path.dirname(d)
    return None

def has_dep(path,dep='semio-framework-job'):
    t=owning_crate(path)
    if not t: return None
    return re.search(r'^'+re.escape(dep)+r'(?:\s*=|\.workspace)',open(t,encoding='utf-8').read(),re.M) is not None

def indent(text,pad='    '):
    return '\n'.join((pad+l if l.strip() else l) for l in text.split('\n'))

def port_close(s,m,fn,trait,flags):
    name,pos,bo,be=fn
    sig=s[pos:bo]
    if 'StepContext' in sig: return None
    mm=re.match(r'fn\s+close_step\s*\(\s*&mut\s+self\s*,\s*(_?\w+)\s*:\s*([^)]*?)\)\s*->\s*([\w:]+)\s*$',re.sub(r'\s+',' ',sig).strip())
    if not mm:
        flags.append('close-sig:'+re.sub(r'\s+',' ',sig)[:100]);return None
    gname,gty,ret=mm.groups()
    if 'Grant' not in gty: flags.append('close-grant-type:'+gty);return None
    ics=ics_names(s,ret)
    body=s[bo+1:be]
    nb,bf=transform_body(body,ics)
    flags.extend(bf)
    multiline='\n' in body
    head=f"fn close_step(&mut self,cx:&mut {JOB}::StepContext<'_>)->Result<{JOB}::InteractiveJobCloseStep,{JOB}::OriginalTurnFailure>"
    if multiline:
        ls=s.rfind('\n',0,pos)+1;pad=s[ls:pos]
        if pad.strip(): pad=''
        inner=indent(nb.lstrip('\n').rstrip())
        new=f"{head}{{\n{pad}    {JOB}::close_original_granted_step(cx,|{gname}|{{\n{inner}\n{pad}    }})\n{pad}}}"
    else:
        new=f"{head}{{{JOB}::close_original_granted_step(cx,|{gname}|{{{nb}}})}}"
    return (pos,be+1,new)

def port_step_value(s,m,fn,impl_start,impl_head_end,flags):
    name,pos,bo,be=fn
    sig=re.sub(r'\s+',' ',s[pos:bo]).strip()
    mm=re.match(r"fn step<'a>\s*\(&'a mut self,\s*(_?\w+)\s*:\s*&mut (?:[\w:]*::)?StepContext<'_>\s*\)\s*->\s*(.+)$",sig)
    if not mm:
        flags.append('step-sig:'+sig[:140]);return None
    cxname,ret=mm.groups()
    ok='ValueError' in ret
    alias=re.fullmatch(r"(\w+)<'a>",ret.strip())
    if alias and re.search(r"type\s+"+alias.group(1)+r"<'a>\s*=\s*[^;]*ValueError",s): ok=True
    if not ok:
        flags.append('step-ret:'+ret[:100]);return None
    return cxname,ret,s[pos:be+1]

def port_step_fault(s,m,fn,flags):
    name,pos,bo,be=fn
    sigtxt=s[pos:bo]
    sg=re.sub(r'\s+',' ',sigtxt)
    if not re.search(r',\s*Fault\s*>\s*$',sg.strip()):
        if 'OriginalTurnFailure' in sg: return None
        flags.append('acw-step-sig:'+sg[:150]);return None
    new_sig=re.sub(r',(\s*)Fault(\s*>\s*)$',r','+r'\1'+JOB+r'::OriginalTurnFailure\2',sigtxt.rstrip())+' '
    body=s[bo+1:be]
    multiline='\n' in body
    if multiline:
        ls=s.rfind('\n',0,pos)+1;pad=s[ls:pos]
        if pad.strip(): pad=''
        new=f"{new_sig.rstrip()} {{\n{pad}    let original: Result<_, Fault> = (|| {{{body}\n{pad}    }})();\n{pad}    original.map_err({JOB}::OriginalTurnFailure::from)\n{pad}}}"
    else:
        new=f"{new_sig.rstrip()} {{let original: Result<_, Fault> = (|| {{{body}}})();original.map_err({JOB}::OriginalTurnFailure::from)}}"
    return (pos,be+1,new)

def port_text(s,path=''):
    m=mask(s);dep=depth_array(m);edits=[];flags=[]
    for im in IMPL.finditer(m):
        b=im.end()-1;e=match_brace(m,b);trait=im.group('trait')
        fns={n:(n,p,bo,be) for n,p,bo,be in children(m,dep,b,e)}
        if 'close_step' in fns:
            r=port_close(s,m,fns['close_step'],trait,flags)
            if r: edits.append(r)
        if trait=='ArtifactCommandWork' and 'step' in fns:
            r=port_step_fault(s,m,fns['step'],flags)
            if r: edits.append(r)
        if trait=='InteractiveJob' and 'step' in fns:
            f=fns['step'];sg=re.sub(r'\s+',' ',s[f[1]:f[2]])
            if 'OriginalTurnFailure' not in sg:
                r=port_step_value(s,m,f,im.start(),b,flags)
                if r:
                    cxname,ret,fntext=r
                    head=s[im.start():b]
                    hh=re.sub(r'\b(?:[\w]+::)*InteractiveJob\b(?:<[^{]*?>)?\s+for\s+','',head,count=1)
                    ls=s.rfind('\n',0,f[1])+1;pad=s[ls:f[1]]
                    if pad.strip(): pad=''
                    prev=s[:ls].rstrip().rsplit('\n',1)[-1].strip()
                    if prev.startswith('///') or prev.startswith('#['): flags.append('step-attached-docs')
                    lift=f"fn step<'a>(&'a mut self,{cxname}:&mut {JOB}::StepContext<'_>)->Result<Option<{JOB}::JobOutcomeBorrow<'a>>,{JOB}::OriginalTurnFailure>{{self.step_value({cxname}).map_err({JOB}::OriginalTurnFailure::from)}}"
                    inh_fn=re.sub(r"^fn step<'a>","/// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.\n"+pad+"fn step_value<'a>",fntext,count=1)
                    if 'step_value' in s: flags.append('step_value-exists')
                    edits.append((f[1],f[3]+1,lift))
                    edits.append((im.start(),im.start(),f"{hh}{{\n{pad}{inh_fn}\n}}\n\n"))
    return edits,flags

def apply(s,edits):
    edits=sorted(edits,key=lambda t:(t[0],t[1]),reverse=True)
    for a,b,t in edits: s=s[:a]+t+s[b:]
    return s

if __name__=='__main__':
    write='--write' in sys.argv;show='--diff' in sys.argv
    paths=[a for a in sys.argv[1:] if not a.startswith('--')]
    files=[]
    for a in paths:
        if os.path.isfile(a) and a.endswith('.rs'): files.append(a)
        else: files+= [l for l in open(a).read().split('\n') if l]
    changed=0;flagged=0
    for p in files:
        s=open(p,encoding='utf-8').read()
        edits,flags=port_text(s,p)
        if edits:
            dep=has_dep(p)
            if dep is not True and 'semio_framework_job::' not in s: flags.append('crate-dep-job:%s'%dep)
        if not edits and not flags: continue
        ns=apply(s,edits) if edits else s
        tag='CHG' if edits else '   '
        if edits: changed+=1
        if flags: flagged+=1
        print(f"{tag}\t{p}\t{sorted(set(flags))}")
        if show and edits:
            sys.stdout.writelines(difflib.unified_diff(s.splitlines(True),ns.splitlines(True),'a','b',n=1))
        if write and edits: open(p,'w',encoding='utf-8').write(ns)
    print('changed',changed,'flagged',flagged,file=sys.stderr)
