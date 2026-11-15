import json,subprocess,sys,os
R='/Users/Shared/semio/';G=R+'.cache/b/g/m/generations/'
m=json.load(open(G+'m/9be866174b9d192d9fa2bfe31d8ce19a49e3b3709c22a4d140680fb1e20890a2.json'))
out=open('rust-nodes.jsonl','w');n=0;allo=0
for k,v in m['owners'].items():
    allo+=1
    if v.get('kind')!='project':continue
    root=v['root']
    if not os.path.exists(R+root+'/Cargo.toml'):continue
    r=subprocess.run(['brotli','-dc',G+f"o/{v['fragment']}.json.br"],capture_output=True)
    if r.returncode:continue
    fr=json.loads(r.stdout);node=fr['node']
    ts={t:{'cache':x.get('cache'),'inputs':x.get('inputs'),'dependsOn':x.get('dependsOn')} for t,x in (node.get('targets') or {}).items()}
    out.write(json.dumps({'name':k,'root':root,'config':v.get('config'),'groups':node.get('namedInputs'),'targets':ts,'edges':sorted({e['target'] for e in fr.get('edges',[])})},ensure_ascii=False)+'\n');n+=1
print(allo,n)
