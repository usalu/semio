import json,glob,subprocess,sys
R='/Users/Shared/semio/';G=R+'.cache/b/g/m/generations/'
ms=sorted(glob.glob(G+'m/*.json'),key=lambda f:json.load(open(f))['generation'])
m=json.load(open(ms[-1]))
out=open('nodes.jsonl','w')
n=0
for k,v in m['owners'].items():
    h=v['fragment']
    r=subprocess.run(['brotli','-dc',G+f'o/{h}.json.br'],capture_output=True)
    if r.returncode:continue
    fr=json.loads(r.stdout)
    node=fr.get('node',{})
    ts={t:{'cache':x.get('cache'),'inputs':x.get('inputs'),'dependsOn':x.get('dependsOn')} for t,x in (node.get('targets') or {}).items()}
    deps=sorted({e['target'] for e in fr.get('edges',[])})
    out.write(json.dumps({'name':k,'root':fr.get('root'),'tags':node.get('tags'),'groups':node.get('namedInputs'),'targets':ts,'deps':deps},ensure_ascii=False)+'\n')
    n+=1
print(n)
