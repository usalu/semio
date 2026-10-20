import sys,json,subprocess,re,os
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
d=sys.argv[1]
src=open(d+"/🟦️.ts",encoding='utf-8').read()
a='expect(tables).toEqual(item.tables);'
assert src.count(a)==1
out=C+"/🗑️generated/law-drift-run/cells-"+str(os.getpid())+".json"
src=src.replace(a,'(globalThis as any).__cells=((globalThis as any).__cells??{});(globalThis as any).__cells[item.name]={tables:tables,bytes:tables.reduce((n:number,t:any)=>n+t.bytes,0)};require("node:fs").writeFileSync('+json.dumps("/Users/ueli/Documents/semio/"+out)+',JSON.stringify((globalThis as any).__cells));continue;'+a)
tmp=d+"/dbg-bless.ts"
open(tmp,'w',encoding='utf-8').write(src)
subprocess.run([C+"/law-drift-tools/t.sh",tmp],capture_output=True)
os.remove(tmp)
cells=json.load(open(out))
fx=d+"/🧫️fixtures/📏️semantic-cells/🔣️.json"
j=json.load(open(fx,encoding='utf-8'))
for c in j['cases']:
    n=cells[c['name']]
    print(c['name'],c['bytes'],'->',n['bytes'])
    c['tables']=n['tables']; c['bytes']=n['bytes']
raw=open(fx,encoding='utf-8').read()
indent=2
open(fx,'w',encoding='utf-8').write(json.dumps(j,ensure_ascii=False,indent=2)+"\n")
