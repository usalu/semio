import json,subprocess,sys,os
C=sys.argv[1];G=C+'/🗑️generated/law-sweep2';os.makedirs(G,exist_ok=True)
files=subprocess.run(['git','grep','-l','workspaceCommand','--','*project.json'],capture_output=True,text=True).stdout.split('\n')
out=[]
for f in files:
    if not f: continue
    p=json.load(open(f))
    for t,d in p.get('targets',{}).items():
        wc=d.get('metadata',{}).get('semio',{}).get('workspaceCommand')
        if wc: out.append(dict(command=' '.join(wc),project=p['name'],target=t,file=f,executor=d.get('executor'),options=d.get('options'),dependsOn=d.get('dependsOn')))
json.dump(out,open(G+'/routes.json','w'),ensure_ascii=False,indent=1)
print(len(out))
