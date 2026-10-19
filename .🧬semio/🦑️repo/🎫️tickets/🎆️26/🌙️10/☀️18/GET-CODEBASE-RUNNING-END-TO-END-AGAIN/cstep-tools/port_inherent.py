"""cstep: turn bare InteractiveJobCloseStep-valued inherent fns that construct Refused into Result<_, ValueError> fns.
usage: port_inherent.py [--write] [--diff] listfile|path..."""
import re,sys,os,difflib
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace,fn_items,enclosing_impl
from body import transform_body
from port_traits import ics_names,apply,IMPL

def port_text(s):
    m=mask(s);edits=[];flags=[];done=[]
    for name,st,bo,be,sig in fn_items(s,m):
        sg=re.sub(r'\s+',' ',sig)
        rm=re.search(r'->\s*((?:[\w]+::)*(?:InteractiveJobCloseStep|Step|Close|ICS))\s*$',sg.strip())
        if not rm or re.search(r'->.*Result',sg): continue
        body=s[bo+1:be]
        if 'Refused' not in m[bo+1:be]: continue
        im=enclosing_impl(m,st)
        if im and IMPL.search(m[im[1]:im[2]+1]): continue
        ics=ics_names(s,rm.group(1))
        nb,bf=transform_body(body,ics);flags.extend(bf)
        vt='semio_framework_value::ValueError'
        newsig=s[st:bo].rstrip()
        newsig=re.sub(r'->\s*'+re.escape(rm.group(1))+r'\s*(?:where[^{]*)?$','',newsig.rstrip()) if False else newsig
        pos=newsig.rfind('->')
        head=newsig[:pos]+'-> Result<'+rm.group(1)+', '+vt+'> '
        edits.append((st,be+1,head+'{'+nb+'}'));done.append(name)
    return edits,flags,done

if __name__=='__main__':
    write='--write' in sys.argv;show='--diff' in sys.argv
    paths=[a for a in sys.argv[1:] if not a.startswith('--')]
    files=[]
    for a in paths:
        if os.path.isfile(a) and a.endswith('.rs'): files.append(a)
        else: files+=[l for l in open(a).read().split('\n') if l]
    for p in files:
        s=open(p,encoding='utf-8').read()
        edits,flags,done=port_text(s)
        if not edits: continue
        ns=apply(s,edits)
        print('CHG',p,done,sorted(set(flags)))
        if show: sys.stdout.writelines(difflib.unified_diff(s.splitlines(True),ns.splitlines(True),'a','b',n=1))
        if write: open(p,'w',encoding='utf-8').write(ns)
