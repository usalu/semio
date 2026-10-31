#!/usr/bin/env python3
"""usage: wrapsig.py <dir> -- wraps `fn close_step(..)->Result<RetainedCloneStep,ValueError>{..}` in ErasedSnapshotRetirement impls and `fn retire_owned(..)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,T)>{..}` in factory impls into RetainedWorkOutcome-returning closures."""
import re,sys,os
def mb(s,i):
    d=0;j=i
    while True:
        c=s[j]
        if c=='{':d+=1
        elif c=='}':
            d-=1
            if d==0:return j
        j+=1
P1=re.compile(r'fn close_step\(&mut self,\s*(\w+):\s*(?:semio_framework_value::(?:retained_clone::)?)?RetainedCloneGrant\)\s*->\s*Result<(?:semio_framework_value::(?:retained_clone::)?)?RetainedCloneStep,\s*(?:semio_framework_value::)?ValueError>\s*\{')
P2=re.compile(r'fn retire_owned\(([^)]*)\)\s*->\s*Result<(\(Box<dyn ErasedSnapshotRetirement>,\s*RetainedCloneProgress\)),\s*(\(ValueError,\s*[^>{]*?\))>\s*\{')
tot=0
for root,_,fs in os.walk(sys.argv[1]):
    for fn in fs:
        if fn!='🦀️.rs':continue
        p=os.path.join(root,fn);s=open(p).read();out='';pos=0;n=0
        for pat,kind in ((P1,'close'),(P2,'retire')):
            out='';pos=0
            for m in pat.finditer(s):
                pre=s[:m.start()];k=max(pre.rfind('\nimpl'),pre.rfind('\n    impl'))
                head=pre[k:k+260]
                if kind=='close' and 'ErasedSnapshotRetirement' not in head:continue
                if kind=='retire' and 'ArtifactOwnedValueRetirementFactory' not in head:continue
                b=m.end()-1;e=mb(s,b);body=s[b+1:e]
                if kind=='close':
                    g=m.group(1)
                    new=f'fn close_step(&mut self, {g}: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome {{ let result = (|| -> Result<RetainedCloneStep, ValueError> {{{body}}})(); let work = u64::from(result.as_ref().is_ok_and(|step| step.progress() != Default::default())); semio_framework_value::RetainedWorkOutcome {{ result, work }} }}'
                else:
                    sig=m.group(0);head_sig=sig[:sig.index('->')]
                    ok,err=m.group(2),m.group(3)
                    new=f'{head_sig}-> semio_framework_value::RetainedWorkOutcome<{ok},{err}> {{ let result = (|| -> Result<{ok},{err}> {{{body}}})(); let work = u64::from(result.is_ok()); semio_framework_value::RetainedWorkOutcome {{ result, work }} }}'
                out+=s[pos:m.start()]+new;pos=e+1;n+=1
            out+=s[pos:];s=out
        if n:open(p,'w').write(s);tot+=n;print(n,p[-70:])
print(tot)
