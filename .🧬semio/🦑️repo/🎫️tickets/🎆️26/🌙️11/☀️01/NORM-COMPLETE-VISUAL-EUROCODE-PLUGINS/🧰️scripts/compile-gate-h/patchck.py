import re,os,sys,collections
sys.path.insert(0,os.environ.get("RSCK_DIR","/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🗑️generated/compile-review-a"))
from rsck import clean
def mc(s,i,o,c):
    d=0
    for j in range(i,len(s)):
        if s[j]==o:d+=1
        elif s[j]==c:
            d-=1
            if d==0:return j
    return -1
def split_top(body):
    parts=[];d=0;cur=''
    for ch in body:
        if ch in '([{<':d+=1
        if ch in ')]}>':d-=1
        if ch==',' and d==0: parts.append(cur);cur=''
        else: cur+=ch
    if cur.strip(): parts.append(cur)
    return parts
def norm(t):
    t=re.sub(r'\s+','',t)
    t=re.sub(r'(?:semio_s_artifact_norm_en\d+_[a-z_]+::|crate::|super::)','',t)
    return t
def structs(root):
    out=collections.defaultdict(list)
    for dp,dn,fn in os.walk(root):
        dn[:]=[x for x in dn if x not in('node_modules','target','.git')]
        for f in fn:
            if not f.endswith('.rs'): continue
            p=os.path.join(dp,f)
            try:s=clean(open(p,encoding='utf8').read())
            except:continue
            s=re.sub(r'#!?\[(?:[^\[\]]|\[[^\]]*\])*\]','',s)
            for m in re.finditer(r'\bstruct\s+(\w+)\s*(?:<[^>{]*>)?\s*\{',s):
                e=mc(s,m.end()-1,'{','}')
                body=s[m.end():e]
                fl={}
                for p_ in split_top(body):
                    mm=re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:\s*(.*)$',p_,re.S)
                    if mm: fl[mm.group(1)]=norm(mm.group(2))
                out[m.group(1)].append((p,fl))
    return out
if __name__=='__main__':
    art=sys.argv[1]
    S=structs(art)
    for dp,dn,fn in os.walk(art):
        dn[:]=[x for x in dn if x not in('node_modules','target','.git')]
        for f in fn:
            if not f.endswith('.rs'): continue
            p=os.path.join(dp,f)
            try:s=clean(open(p,encoding='utf8').read())
            except:continue
            s=re.sub(r'#!?\[(?:[^\[\]]|\[[^\]]*\])*\]','',s)
            for m in re.finditer(r'row_patch!\s*\{',s):
                e=mc(s,m.end()-1,'{','}'); body=s[m.end():e]
                mm=re.search(r'pub\s+(\w+)\s+of\s+([\w:]+)\s*\{\s*set\s*\{(.*)\}\s*\}\s*$',body,re.S)
                if not mm: print(p[-60:],'unparsed row_patch',body[:80].replace('\n',' ')); continue
                name,row,sets=mm.groups(); row=row.split('::')[-1]
                cands=S.get(row,[])
                if not cands: print(p[-70:],name,'row struct not found',row); continue
                fields={}
                for _,fl in cands: fields.update(fl)
                pass
                for it in split_top(sets):
                    m2=re.match(r'\s*(\w+)\s*:\s*(.*)$',it,re.S)
                    if not m2: continue
                    fn_,ty=m2.group(1),norm(m2.group(2))
                    if fn_ not in fields: print(p[-70:],name,'field',fn_,'not in',row)
                    elif fields[fn_]!=ty and not (fields[fn_].replace('Option<','')[:-1]==ty if fields[fn_].startswith('Option<') else False): print(p[-70:],name,'field',fn_,'type',ty,'vs model',fields[fn_])
