import re,sys
t=open(sys.argv[1],encoding='utf-8',errors='replace').read()
secs=re.split(r'(?m)^==================== Test output for //:(t_\w+):',t)
seen={}
for i in range(1,len(secs),2):
    tid,body=secs[i],secs[i+1]
    if tid in seen: continue
    proj=re.findall(r'OwnedCommandFailure: (\S+):test:\w+ failed',body)
    body=re.sub(r'(?m)^\[(script|repository:command|bazel:\w+|process:owner-command)\].*\n','',body)
    lines=[l for l in body.split('\n') if re.search(r'(×|FAIL |Error|error:|expected|Expected|Received|✗|\(fail\)| fail$|exceeded|ENOENT|Cannot find|not found)',l)]
    seen[tid]=1
    print('###',tid,proj[:1])
    for l in lines[:int(sys.argv[2]) if len(sys.argv)>2 else 8]: print('   ',l[:260])
