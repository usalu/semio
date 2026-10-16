import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex,items,name_kind
def members(text,startline_idx=None):
    src=text
    out=[]
    for it in items(src):
        v,k,nm=name_kind(it['toks'])
        if k=='impl':
            toks=it['toks']
            br=[q for q,t in enumerate(toks) if t[0]=='p' and t[1]=='{'][0]
            hdr=''.join(t[1]+' ' for t in toks[:br] if t[0]!='comment')
            base=toks[br][4]+1
            endq=len(toks)-1
            sub=src[base:toks[endq][4]]
            for mit in items(sub):
                mv,mk,mn=name_kind(mit['toks'])
                out.append((hdr.strip(),mit['first_tok_line']+src[:base].count('\n')+1,mit['end_line']+src[:base].count('\n')+1,mv,mk,mn,mit['attrs']))
    return out
if __name__=='__main__':
    for r in members(open(sys.argv[1],encoding='utf-8').read()):
        if len(sys.argv)>2 and sys.argv[2] not in r[0]:continue
        print(r)
