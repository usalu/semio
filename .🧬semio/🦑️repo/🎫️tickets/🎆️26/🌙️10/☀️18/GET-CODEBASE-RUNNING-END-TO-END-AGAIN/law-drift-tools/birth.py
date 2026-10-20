import sys,re
f=sys.argv[1]; owner=sys.argv[2]
s=open(f,encoding='utf-8').read()
pat=re.compile(r'(\b\w+)\.receiveScriptProcessInvocation\(process\.env,')
print(len(pat.findall(s)), s.count('receiveScriptProcessInvocation(process.env'))
s=pat.sub(lambda m:m.group(1)+'.withScriptProcessEnvelope('+m.group(1)+'.createScriptProcessEnvelope({version:1,owner:"'+owner+'",maximumElapsedMilliseconds:0},{},Date.now()),',s)
open(f,'w',encoding='utf-8').write(s)
