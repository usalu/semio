import re,sys,collections,json
sys.path.insert(0,__file__.rsplit('/',1)[0])
from classify import ret_of
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rows=[l.strip().rsplit(":",1) for l in open(C+"/🗑️generated/1101/aud-policy-unwrap-rows.txt")]
only=[a for a in sys.argv[1:] if not a.startswith('--')]
apply='--apply' in sys.argv
byf=collections.OrderedDict()
for f,n in rows:byf.setdefault(f,set()).add(int(n))
patB=re.compile(r'(?<![\w.\]])(?P<recv>self\.[A-Za-z_]\w*(?:\.[A-Za-z_]\w*|\[[^\]\[]*\])*)\.unwrap\(\)')
patC=re.compile(r'\.(?P<callee>[A-Za-z_]\w*)\((?P<args>[^()]*)\)\.expect\((?P<lit>"[^"\\]*")\)')
patD=re.compile(r'\.(?P<callee>get|get_mut|first|last|last_mut|first_mut|pop|next|peek|front|back|find_map)\((?P<args>[^()]*)\)\.unwrap\(\)')
patE=re.compile(r'(?<![\w.\]\)])(?P<recv>[a-z_]\w*)\.unwrap\(\)')
patF=re.compile(r'(?:unreachable|panic)!\((?P<lit>"[^"\\]*")?\)')
OPT={'get','get_mut','next','first','last','checked_add','checked_sub','checked_mul','find','position','pop','as_ref','as_mut','take','first_mut','last_mut','strip_prefix','split_first','iter_mut','peek','max','min','copied','cloned','nth','find_map'}
stat=collections.Counter();skipped=[]
TESTISH=re.compile(r'(testkit|🧪|cold-laws|/tests?/)')
for f,ns in byf.items():
    if only and not any(o in f for o in only):continue
    if TESTISH.search(f):continue
    t=open(f).read()
    L=t.split("\n");off=[0]
    for l in L:off.append(off[-1]+len(l)+1)
    hasv=re.search(r'\bValueError\b',t) is not None
    edits=[]
    def mode_of(pos):
        nm,r=ret_of(t,pos);r=r or ''
        em=re.search(r',\s*(BitmapTraceError|BooleanError|StrokeOutlineError|PixelEditError|AffineImageError)>',r)
        if em and re.search(em.group(1)+r'::Invalid\(',t):return 'inv',em.group(1),r
        if ('OriginalTurnFailure' in r or 'ValueError' in r or 'RetainedWorkOutcome' in r or 'RetainedCloneStep' in r) and hasv:return 'val',None,r
        if "&'static str" in r:return 'str',None,r
        return None,None,r
    def mk(mode,tn,msg,kind):
        if kind=='opt':
            return {'val':f'.ok_or_else(||ValueError::invariant({msg}))?','inv':f'.ok_or({tn}::Invalid({msg}))?','str':f'.ok_or({msg})?'}[mode]
        return {'val':f'.map_err(|_|ValueError::invariant({msg}))?','inv':f'.map_err(|_|{tn}::Invalid({msg}))?','str':f'.map_err(|_|{msg})?'}[mode]
    for n in sorted(ns):
        s0,e0=off[n-1],off[n]-1
        for m in patB.finditer(t,s0,e0):
            mode,tn,r=mode_of(m.start())
            if not mode:skipped.append((f,n,'B '+r[:40]));stat['skipB']+=1;continue
            last=m.group('recv').split('.')[-1]
            last=re.sub(r'\[.*','',last)
            edits.append((m.start(),m.end(),m.group('recv')+mk(mode,tn,f'"{last} absent"','opt')));stat['B'+mode]+=1
        for m in patF.finditer(t,s0,e0):
            if 'proc' in f or 'derive' in f or '✨' in f:continue
            mode,tn,r=mode_of(m.start())
            if not mode or r.startswith('Poll') :skipped.append((f,n,'F '+r[:40]));stat['skipF']+=1;continue
            lit=m.group('lit') or '"state invariant violated"'
            e={'val':f'ValueError::invariant({lit}).into()','inv':f'{tn}::Invalid({lit})','str':lit}[mode]
            edits.append((m.start(),m.end(),f'return Err({e})'));stat['F'+mode]+=1
        for m in patD.finditer(t,s0,e0):
            mode,tn,r=mode_of(m.start())
            if not mode:skipped.append((f,n,'D '+r[:40]));stat['skipD']+=1;continue
            edits.append((m.start(),m.end(),f'.{m.group("callee")}({m.group("args")})'+mk(mode,tn,f'"{m.group("callee")} target absent"','opt')));stat['D'+mode]+=1
        for m in patE.finditer(t,s0,e0):
            if m.group('recv') in ('self','Self'):continue
            mode,tn,r=mode_of(m.start())
            if not mode:skipped.append((f,n,'E '+r[:40]));stat['skipE']+=1;continue
            edits.append((m.start(),m.end(),m.group('recv')+mk(mode,tn,f'"{m.group("recv")} absent"','opt')));stat['E'+mode]+=1
        for m in patC.finditer(t,s0,e0):
            mode,tn,r=mode_of(m.start())
            if not mode:skipped.append((f,n,'C '+r[:40]));stat['skipC']+=1;continue
            kind='opt' if m.group('callee') in OPT else 'res'
            s=m.start();e=m.end()
            edits.append((s,e,f'.{m.group("callee")}({m.group("args")})'+mk(mode,tn,m.group('lit'),kind)));stat['C'+mode+kind]+=1
    if apply and edits:
        for s,e,rp in sorted(edits,reverse=True):t=t[:s]+rp+t[e:]
        open(f,'w').write(t)
print(stat)
json.dump(skipped,open(C+"/🗑️generated/1101/policyfix-skipped2.json","w"),ensure_ascii=False)
