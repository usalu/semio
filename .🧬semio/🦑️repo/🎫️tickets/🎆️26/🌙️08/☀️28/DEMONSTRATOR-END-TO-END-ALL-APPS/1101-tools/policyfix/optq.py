import re,sys,collections,json
sys.path.insert(0,__file__.rsplit('/',1)[0])
from classify import ret_of
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rows=[l.strip().rsplit(":",1) for l in open(C+"/🗑️generated/1101/aud-policy-unwrap-rows.txt")]
only=[a for a in sys.argv[1:] if not a.startswith('--')]
apply='--apply' in sys.argv
byf=collections.OrderedDict()
for f,n in rows:byf.setdefault(f,set()).add(int(n))
pat=re.compile(r'(?P<recv>[A-Za-z_]\w*)(?P<op>\.as_mut\(\)|\.as_ref\(\)|\.take\(\))\.unwrap\(\)')
stat=collections.Counter();skipped=[]
for f,ns in byf.items():
    if only and not any(o in f for o in only):continue
    t=open(f).read()
    L=t.split("\n");off=[0]
    for l in L:off.append(off[-1]+len(l)+1)
    hasv=re.search(r'\bValueError\b',t) is not None
    edits=[]
    for n in sorted(ns):
        for m in pat.finditer(t,off[n-1],off[n]-1):
            nm,r=ret_of(t,m.start());r=r or ''
            em=re.search(r',\s*(BitmapTraceError|BooleanError|StrokeOutlineError|PixelEditError|AffineImageError)>',r)
            if em and re.search(em.group(1)+r'::Invalid\(',t):mode='inv'
            elif ('OriginalTurnFailure' in r or 'ValueError' in r or 'RetainedWorkOutcome' in r or 'RetainedCloneStep' in r) and hasv:mode='val'
            elif "&'static str" in r or r.startswith('Result<') and r.endswith('&str>'):mode='str'
            else:
                skipped.append((f,n,r[:50]));stat['skip']+=1;continue
            msg=f'{m.group("recv")} absent'
            rep=f'{m.group("recv")}{m.group("op")}'+(f'.ok_or_else(||ValueError::invariant("{msg}"))?' if mode=='val' else f'.ok_or({em.group(1)}::Invalid("{msg}"))?' if mode=='inv' else f'.ok_or("{msg}")?')
            edits.append((m.start(),m.end(),rep));stat[mode]+=1
    if apply and edits:
        for s,e,rp in sorted(edits,reverse=True):t=t[:s]+rp+t[e:]
        open(f,'w').write(t)
print(stat)
json.dump(skipped,open(C+"/🗑️generated/1101/policyfix-skipped.json","w"),ensure_ascii=False)
c=collections.Counter(s[2] for s in skipped)
for k,v in c.most_common(25):print(v,k)
