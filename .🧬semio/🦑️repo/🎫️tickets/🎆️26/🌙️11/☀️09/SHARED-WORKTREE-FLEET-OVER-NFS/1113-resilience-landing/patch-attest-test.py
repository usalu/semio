import sys,glob
p=glob.glob(sys.argv[1]+"/*attest/*tests/\U0001F7E6️.ts")[0]
s=open(p,encoding="utf8").read()
old='restartStableMilliseconds:100,'
assert s.count(old)>=1
i=s.index("const fast=")
j=s.index("};",i)
line=s[i:j]
assert "selfCheck" not in line
line=line.replace("restartStableMilliseconds:100,","restartStableMilliseconds:100,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,selfCheck:async()=>({ok:true as const}),logFiles:[] as string[],",1)
s=s[:i]+line+s[j:]
open(p,"w",encoding="utf8").write(s)
print(line[:400])
