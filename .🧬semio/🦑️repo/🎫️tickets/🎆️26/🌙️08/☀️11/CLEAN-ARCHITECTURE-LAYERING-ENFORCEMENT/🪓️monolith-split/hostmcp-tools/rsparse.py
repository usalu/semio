import re
from rlex import lex
IDENT=re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
def code_tokens(lines,infos):
    """per line: list of (ident, prev_char_ctx) tokens in code (not comments/strings)"""
    out=[]
    for l,info in zip(lines,infos):
        out.append(info['idents'])
    return out
def top_items(lines,infos):
    """yield (start,end,attrs,kind,name,vis,text_first) for depth-0 items. start/end line idx inclusive of attrs/docs."""
    items=[]
    i=0;n=len(lines)
    while i<n:
        info=infos[i]
        if info['depth']!=0 or info['state']!='code':
            i+=1;continue
        s=lines[i].strip()
        if not s or s.startswith('//') :
            i+=1;continue
        j=i
        # attributes and docs
        start=i
        attrs=[]
        while j<n and (lines[j].strip().startswith('#[') or lines[j].strip().startswith('///') or lines[j].strip().startswith('//') or not lines[j].strip()):
            if lines[j].strip().startswith('#['):
                # attribute may span lines
                k=j;txt=lines[k]
                bal=txt.count('[')-txt.count(']')
                while bal>0 and k+1<n:
                    k+=1;txt+=lines[k];bal+=lines[k].count('[')-lines[k].count(']')
                attrs.append(' '.join(txt.split()))
                j=k+1
                continue
            if not lines[j].strip():
                break
            j+=1
        if j>=n:break
        head=lines[j].strip()
        # find end: the first line k>=j where end_depth==0 and (line ends with ';' or '}' )
        k=j
        while k<n:
            if infos[k]['end_depth']==0 and infos[k]['state']=='code':
                t=lines[k].rstrip()
                if t.endswith(';') or t.endswith('}') or t.endswith('},'):
                    break
                # `fn x() -> T where` etc continue
            k+=1
        items.append((start,j,k,attrs,head))
        i=k+1
    return items
