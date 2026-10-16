"""🕵️ Definition scan: every non-pub item, field, method or associated item defined in the OWNER files that any OTHER file mentions as an identifier (analysis only)."""
import sys,re,os,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from common import *
from items import parse
from rlex2 import lex2
def idents(path):
    t=open(path,encoding='utf-8',errors='ignore').read()
    L=[l+'\n' for l in t.split('\n')]
    I=lex2(L);s=set()
    for info in I:
        for n,_ in info['toks']:s.add(n)
        for st in info['strs']:
            for m in re.finditer(r'\{([A-Za-z_]\w*)(?::[^}]*)?\}',st):s.add(m.group(1))
    return s
def defs(path):
    """yield (kind,name,line,visibility) for items, fields and impl members that are not plain pub"""
    L,items=parse(path)
    t=''.join(L);I=lex2(L)
    out=[]
    for it in items:
        head=L[it['head']].strip()
        vis=re.match(r'^(pub(?:\([^)]*\))?)\s',head)
        vis=vis.group(1) if vis else ''
        if it['kind'] in('struct','enum','fn','const','static','type','trait','union','mod','macro') and vis!='pub':
            out.append((it['kind'],it['name'],it['head']+1,vis or 'private'))
        if it['kind'] in('struct','union'):
            for x in range(it['head'],it['end']+1):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^\s*(pub(?:\([^)]*\))?\s+)?(\w+)\s*:',L[x])
                    if m and (m.group(1) or '').strip()!='pub':out.append(('field',m.group(2),x+1,(m.group(1) or 'private').strip()))
        if it['kind']=='impl' and not re.search(r'\bfor\b',L[it['head']].split('{')[0]) :
            for x in range(it['head']+1,it['end']):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^\s*(pub(?:\([^)]*\))?\s+)?((?:(?:unsafe|async|const|default)\s+)*)(fn|const|static|type)\s+(\w+)',L[x])
                    if m and (m.group(1) or '').strip()!='pub':out.append(('member',m.group(4),x+1,(m.group(1) or 'private').strip()))
        if it['kind']=='enum':
            pass
    return out
if __name__=='__main__':
    owner=json.loads(sys.argv[1]);others=json.loads(sys.argv[2])
    otoks={}
    for f in others:otoks[f]=idents(f if os.path.isabs(f) else REPO+f)
    hits=0;count=0
    for f in owner:
        fp=f if os.path.isabs(f) else REPO+f
        for kind,name,line,vis in defs(fp):
            count+=1
            users=[o for o,s in otoks.items() if name in s and o!=f]
            if users:
                hits+=1;print(f"{kind}\t{name}\t{vis}\t{os.path.basename(os.path.dirname(f))}:{line}\tused-in {len(users)} file(s): {os.path.basename(os.path.dirname(users[0]))}")
    print("definitions checked",count,"mentioned elsewhere",hits)
