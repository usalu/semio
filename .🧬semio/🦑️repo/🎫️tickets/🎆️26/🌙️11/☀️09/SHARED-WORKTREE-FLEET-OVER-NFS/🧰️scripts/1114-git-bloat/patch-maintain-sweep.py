import sys
root=sys.argv[1]
p=root+"/🔁️replica/🟦️.ts"
s=open(p).read()
a=" await pruneLostV1(paths.lost,{now,retainMs:settings.retentionDays*86_400_000,maxEntries:lostEntries}).catch(()=>0);\n"
assert s.count(a)==1
s=s.replace(a,a+" await sweepGitGarbageV1(settings.worktree,{now}).catch(()=>undefined);\n")
b='import {applyIndexV1,'
assert s.count(b)==1
s=s.replace(b,'import {sweepGitGarbageV1,applyIndexV1,')
open(p,'w').write(s)
