"""🧷️ Pruned import header from an old parent file for a set of consumer files. usage: hdr.py <oldheader.rs> <file>..."""
import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from usage import toks,leaves
from wsan import parse_use,gather_text,top_items,classify
from rlex2 import lex2
REW=[
 (r'^crate::infinite::','semio_framework_os_infinite::'),
 (r'^crate::os_store::','semio_framework_os_kernel::os_store::'),
 (r'^crate::os_spr::','semio_framework_os_kernel::os_spr::'),
 (r'^evaluation_source::work::','semio_framework_os_flow_host_source::work::'),
]
GLOBREW={
 'crate::artifact':'semio_framework_artifact_flow_flow::artifact',
 'crate::bridge':'semio_framework_os_flow_operators::bridge',
 'crate::catalogue':'semio_framework_os_flow_operators::catalogue',
 'crate::registry':'semio_framework_os_flow_operators::registry',
 'crate::drawing':'semio_framework_os_flow_drawing',
 'crate::host':'semio_framework_os_flow_host',
}
def rewrite(full):
    for a,b in REW:full=re.sub(a,b,full)
    return full
def items(path):
    t=open(path,encoding='utf-8').read()
    m=re.search(r'\n\s*//\s*#region',t)
    cut=m.start() if m else len(t)
    head=t[:cut]
    out=[]
    for m in re.finditer(r'((?:#\[[^\]]*\]\s*)*)(pub(?:\([^)]*\))?\s+)?use\s+(.*?);',head,re.S):
        attrs=[a.strip() for a in re.findall(r'#\[[^\]]*\]',m.group(1))]
        txt=("pub " if m.group(2) else "")+"use "+m.group(3)+";"
        nm,anon,globs=parse_use(txt)
        out.append((attrs,txt,nm,anon,globs))
    return out
def build(old,files,skip_names=()):
    used=set()
    for f in files:used|=toks(f)
    lines=[]
    for attrs,txt,nm,anon,globs in items(old):
        if txt.startswith('pub'):continue
        keep=[(n,rewrite(f)) for n,f in nm if n in used and n not in skip_names]
        gl=[GLOBREW[g] for g in globs if g in GLOBREW]
        pre=''.join(a+' ' for a in attrs)
        if keep:
            groups={}
            for n,f in keep:
                p='::'.join(f.split('::')[:-1]);groups.setdefault(p,[]).append((n,f))
            for p,ls in groups.items():
                parts=[(f.split('::')[-1] if f.split('::')[-1]==n else f"{f.split('::')[-1]} as {n}") for n,f in ls]
                if len(parts)==1:
                    n,f=ls[0];lines.append(pre+("use "+f+";" if f.split('::')[-1]==n else f"use {f} as {n};"))
                else:lines.append(pre+f"use {p}::{{{', '.join(parts)}}};")
        for g in gl:lines.append(pre+f"use {g}::*;")
    for alias,full in (('neural','neural_engine'),('dag','semio_framework_os_infinite::board::ports::directed_dag'),('canvas','semio_framework_canvas')):
        if any(re.match(r'^(?:#\[[^\]]*\]\s*)*use '+alias+'::',l) for l in lines) and not any((l.endswith('use '+full+' as '+alias+';')) for l in lines):
            lines.insert(0,f"use {full} as {alias};")
    return lines
if __name__=="__main__":
    print("\n".join(build(sys.argv[1],sys.argv[2:])))
