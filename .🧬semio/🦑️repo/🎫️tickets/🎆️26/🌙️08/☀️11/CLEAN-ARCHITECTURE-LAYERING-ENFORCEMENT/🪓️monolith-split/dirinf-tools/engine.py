import os,re,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex,items,name_kind

def read_lines(p):
    return open(p,encoding='utf-8').read().split('\n')

def assemble(src,children,parent_wire):
    """src: list of lines (no trailing newline element handling: last element is '' if file ends with newline).
    children: list of dict(name, folder, doc, ranges, pre, post, dedent, uses, cfg, glob, modvis, mod, reexport)
    returns (parent_text, {folder: text})"""
    n=len(src)
    owner=[None]*(n+1)
    for c in children:
        for a,b in c['ranges']:
            for i in range(a,b+1):
                assert owner[i] is None,(c['name'],i)
                owner[i]=c['name']
        for i in c.get('drop',[]):
            assert owner[i]==c['name']
    first={}
    for c in children:
        first[c['ranges'][0][0]]=c
    out=[]
    for i in range(1,n+1):
        if i in first:
            out.extend(parent_wire(first[i]))
        if owner[i] is None:
            out.append(src[i-1])
    ptxt=collapse(out)
    files={}
    for c in children:
        body=[]
        drop=set(c.get('drop',[]))
        dd=c.get('dedent')
        for k,(a,b) in enumerate(c['ranges']):
            if k:body.append('')
            for i in range(a,b+1):
                if i in drop:continue
                l=src[i-1]
                if dd and dd[0]<=i<=dd[1] and l.strip()!='':
                    assert l.startswith(' '*dd[2]),(c['name'],i,l)
                    l=l[dd[2]:]
                elif l.strip()=='':l=''
                for rx,rp in c.get('sub',[]):l=re.sub(rx,rp,l)
                body.append(l)
        while body and body[-1].strip()=='':body.pop()
        head=[c['doc'],'']
        head+=c.get('uses',[])
        if head[-1]!='':head.append('')
        lines=head+c.get('pre',[])+body+c.get('post',[])
        files[c['folder']]='\n'.join(lines)+'\n'
    return ptxt,files

def collapse(lines):
    out=[];blank=0
    for l in lines:
        if l.strip()=='':
            blank+=1
            if blank>1:continue
            out.append('')
        else:
            blank=0;out.append(l)
    return '\n'.join(out)

def ident_set(text):
    s=set()
    for k,x,sl,el,si in lex(text):
        if k=='id':s.add(x)
        elif k=='str':
            for m in re.finditer(r'\{([A-Za-z_][A-Za-z0-9_]*)',x):s.add(m.group(1))
    return s
