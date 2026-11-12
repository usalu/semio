import json,sys
S='/tmp/claude-1000/-home-deck-semio/0ec9c8a1-06e8-45c6-b0fa-6d0c7cb86d6b/scratchpad'
files=[l.rstrip('\n') for l in open(S+'/pj.txt',encoding='utf8')]
bad=0
for f in files:
    s=open(f,encoding='utf8').read()
    try:d=json.loads(s)
    except: continue
    o=json.dumps(d,indent=2,ensure_ascii=False)+'\n'
    if o!=s: bad+=1; 
    if o!=s and bad<4: print('diff',f[-70:])
print('bad',bad,'of',len(files))
