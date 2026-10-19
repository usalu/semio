"""cstep stale scanner for the borrowed-cx close API (extends lfix-tools/scan/stale*.py). usage: stale.py roots...
Rules (each prints `RULE path:line text`; exit code 1 when any fires):
  R1 grant-shaped close_step in an impl of InteractiveJob / FixedOperationOwner / ArtifactCommandWork
  R2 `<InteractiveJobCloseStep or alias>::Refused` anywhere
  R3 InteractiveJob impl whose step still answers ValueError (alias aliases resolved)
  R4 ArtifactCommandWork impl whose step still answers Fault
  R5 bare close_step(<non-cx>) on a receiver typed as an InteractiveJob / ArtifactCommandWork / FixedOperationOwner implementor (typed_calls heuristic)"""
import re,sys,os,subprocess
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
from rs import mask,match_brace,enclosing_impl
from port_traits import IMPL,children,depth_array
from census import walk
roots=sys.argv[1:]
files=[p for p in walk(roots)]
bad=0
def emit(rule,p,s,pos,txt=None):
    global bad;bad+=1;ln=s.count('\n',0,pos)+1
    print(f"{rule} {p}:{ln}: {(txt or s.splitlines()[ln-1]).strip()[:150]}")
for p in files:
    try:s=open(p,encoding='utf-8').read()
    except Exception: continue
    if not re.search(r'InteractiveJob|FixedOperationOwner|ArtifactCommandWork|InteractiveJobCloseStep',s): continue
    m=mask(s);dep=depth_array(m)
    names={'InteractiveJobCloseStep'}
    for a in re.finditer(r'InteractiveJobCloseStep\s+as\s+(\w+)',s): names.add(a.group(1))
    for r in re.finditer(r'\b(?:'+'|'.join(names)+r')::Refused\b',m): emit('R2',p,s,r.start())
    for im in IMPL.finditer(m):
        b=im.end()-1;e=match_brace(m,b);trait=im.group('trait')
        for n,pos,bo,be in children(m,dep,b,e):
            sig=re.sub(r'\s+',' ',s[pos:bo])
            if n=='close_step' and re.search(r'Grant|grant',sig) and 'StepContext' not in sig: emit('R1',p,s,pos,sig)
            if n=='step' and trait=='InteractiveJob' and 'OriginalTurnFailure' not in sig:
                ret=sig.split('->',1)[-1]
                al=re.fullmatch(r"\s*(\w+)<'a>\s*",ret)
                if 'ValueError' in ret or (al and re.search(r"type\s+"+al.group(1)+r"<'a>\s*=\s*[^;]*ValueError",s)): emit('R3',p,s,pos,sig)
            if n=='step' and trait=='ArtifactCommandWork' and re.search(r',\s*(?:[\w:]*::)?Fault\s*>\s*$',sig.strip()): emit('R4',p,s,pos,sig)
listing=os.path.join(os.path.dirname(HERE),'🗑️generated','cstep-stale-files.txt');os.makedirs(os.path.dirname(listing),exist_ok=True);open(listing,'w').write('\n'.join(files))
r=subprocess.run([sys.executable,os.path.join(HERE,'typed_calls.py'),listing,listing],capture_output=True,text=True)
for l in r.stdout.split('\n'):
    if not l or re.search(r'close_with_self_funded_context|self_funded|with_retained_loan|caller\.turn|EnergyJobAuthority::|rejected\.close_step|engine\.close_step\(\)|SceneDecodeCursor|SceneInputCursor|TypedWindowConfigStoreOwner|\[ij\] request : |\[ij\] rejected : ArtifactEnvelopeDecodeAuthority',l): continue
    bad+=1;print('R5',l[:220])
print('stale findings',bad,file=sys.stderr)
sys.exit(1 if bad else 0)
