import re,sys,os,glob
roots=sys.argv[2:]
dry=sys.argv[1]=='dry'
def split_top_commas(body):
    d=0
    for i,c in enumerate(body):
        if c in '<([': d+=1
        elif c in '>)]' and not (c=='>' and body[i-1:i]=='-'): d-=1
        elif c==',' and d==0: return True
    return False
def key(t): return re.sub(r'<.*$','',re.sub(r'^.*::','',t.strip()))
def split_top(body):
    out=[];d=0;cur=''
    i=0
    while i<len(body):
        c=body[i]
        if c in '<([': d+=1
        if c in '>)]' and not (c=='>' and body[i-1:i]=='-'): d-=1
        if c=='+' and d==0: out.append(cur.strip()); cur=''
        else: cur+=c
        i+=1
    out.append(cur.strip()); return [t for t in out if t]
LINE=re.compile(r'^(\s*)((?:where )?)([A-Za-z_][A-Za-z_0-9]*)(\s*):(?!:)\s*(.+?)(,?)\s*$')
def prefix_of(src):
    m=re.search(r'use\s+((?:::)?[A-Za-z_0-9:]+)::\{?[^;]*\bColdMutation\b',src)
    if m: return m.group(1)+'::'
    m=re.search(r'((?:::)?(?:[A-Za-z_0-9]+::)+)ColdMutation',src)
    return m.group(1) if m else 'semio_framework_os_spr::'
def rewrite_clause(name,body,pref):
    toks=split_top(body); ks={key(t) for t in toks}
    if 'ColdMutation' in ks and 'OpBinary' in ks and 'OpText' in ks and 'Mutation' in ks and 'Clone' in ks and ({'ToValue','ValueToValue'}&ks) and ({'FromValue','ValueFromValue'}&ks) and 'Send' in ks and "'static" in ks:
        cold=[t for t in toks if key(t)=='ColdMutation'][0]
        pm=re.search(r'<(.*)>$',cold)
        if not pm: return None
        drop={'Clone','ToValue','ValueToValue','FromValue','ValueFromValue','Mutation','ColdMutation','OpBinary','OpText','Send',"'static"}
        return ' + '.join([t for t in toks if key(t) not in drop]+[f'{pref}StoreMutationFor<{pm.group(1)}>'])
    if {'Clone','Send','Sync',"'static"}<=ks and ({'ToValue','ValueToValue'}&ks) and ({'FromValue','ValueFromValue'}&ks) and 'Mutation' not in ks:
        drop={'Clone','ToValue','ValueToValue','FromValue','ValueFromValue','Send','Sync',"'static"}
        return ' + '.join([t for t in toks if key(t) not in drop]+[f'{pref}StoreState'])
    return None
def fix_inline(l,pref):
    out=l; changed=False
    for m in list(re.finditer(r'\bwhere\b',l))[::-1]:
        i=m.end(); d=0; cur=i; parts=[]; j=i
        while j<len(l):
            c=l[j]
            if c in '<([': d+=1
            elif c in '>)]' and not (c=='>' and l[j-1:j]=='-'): d-=1
            if d==0 and c==',' : parts.append((cur,j)); cur=j+1
            if d==0 and c in '{;': break
            j+=1
        parts.append((cur,j))
        new=l[:i]; pos=i; res=[]
        for a,b in parts:
            seg=l[a:b]; mm=re.match(r'^(\s*)([A-Za-z_][A-Za-z_0-9]*)(\s*):(?!:)\s*(.+?)(\s*)$',seg)
            if mm and l[a:b].strip():
                r=rewrite_clause(mm.group(2),mm.group(4),pref)
                if r: seg=f"{mm.group(1)}{mm.group(2)}{mm.group(3)}: {r}{mm.group(5)}"; changed=True
            res.append(seg)
        if changed: out=l[:i]+','.join(res)+l[j:]; l=out
    return out,changed
def fix(src):
    lines=src.split('\n'); changed=False
    pref=prefix_of(src)
    for i,l in enumerate(lines):
        if 'where' in l:
            nl,ch=fix_inline(l,pref)
            if ch: lines[i]=nl; changed=True; continue
        m=LINE.match(l)
        if not m: continue
        ind,w,name,sp,body,comma=m.groups()
        if re.search(r'[{};]',body) or split_top_commas(body): continue
        toks=split_top(body)
        keys=[key(t) for t in toks]
        ks=set(keys)
        if 'ColdMutation' in ks and 'OpBinary' in ks and 'OpText' in ks and 'Mutation' in ks and 'Clone' in ks and 'ToValue' in ks|{k.replace('ValueToValue','ToValue') for k in ks} and 'Send' in ks and "'static" in ks:
            cold=[t for t in toks if key(t)=='ColdMutation'][0]
            pm=re.search(r'<(.*)>$',cold)
            if not pm: continue
            P=pm.group(1)
            drop={'Clone','ToValue','ValueToValue','FromValue','ValueFromValue','Mutation','ColdMutation','OpBinary','OpText','Send',"'static"}
            if not ({'FromValue','ValueFromValue'}&ks): continue
            rest=[t for t in toks if key(t) not in drop]
            new=rest+[f'{pref}StoreMutationFor<{P}>']
            lines[i]=f"{ind}{w}{name}{sp}: "+' + '.join(new)+comma; changed=True
        elif {'Clone','Send','Sync',"'static"}<=ks and ({'ToValue','ValueToValue'}&ks) and ({'FromValue','ValueFromValue'}&ks) and 'Mutation' not in ks:
            drop={'Clone','ToValue','ValueToValue','FromValue','ValueFromValue','Send','Sync',"'static"}
            rest=[t for t in toks if key(t) not in drop]
            new=rest+[f'{pref}StoreState']
            lines[i]=f"{ind}{w}{name}{sp}: "+' + '.join(new)+comma; changed=True
    return '\n'.join(lines),changed
n=0
import subprocess
files=subprocess.run(['rg','--files','-g','*.rs']+roots,capture_output=True,text=True).stdout.split('\n')
for r in [0]:
    for p in [f for f in files if f]:
        if '/node_modules/' in p or '/🧪️' in p or 'tests' in p.split('/')[-1]: continue
        s=open(p).read()
        if 'ColdMutation' not in s and 'Clone + ' not in s and 'Clone+' not in s: continue
        t,c=fix(s)
        if c:
            n+=1
            if not dry: open(p,'w').write(t)
            if dry and n<=3: print(p[-90:])
print('files',n)
