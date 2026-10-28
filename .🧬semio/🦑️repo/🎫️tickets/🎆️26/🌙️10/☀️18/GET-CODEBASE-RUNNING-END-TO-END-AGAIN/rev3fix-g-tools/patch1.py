import sys
T=sys.argv[1]
p=T+"/files/dead-holder.ts"
s=open(p,encoding="utf-8").read()
s=s.replace('import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";','import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";')
old='''async function killedHolder(root:string,shared:string,seed?:string):Promise<string>{
 const child=Bun.spawn([process.execPath,worker,root,shared,...seed===undefined?[]:[seed]],{stdout:"pipe",stderr:"inherit"}),id=(await new Response(child.stdout).text()).trim();
 expect(await child.exited,"the holder dies of SIGKILL").not.toBe(0);expect(id).toMatch(/^[a-z0-9]{1,12}$/u);return id;
}'''
new='''async function killedHolder(root:string,shared:string):Promise<string>{
 const child=Bun.spawn([process.execPath,worker,root,shared],{stdout:"pipe",stderr:"inherit"}),id=(await new Response(child.stdout).text()).trim();
 expect(await child.exited,"the holder dies of SIGKILL").not.toBe(0);expect(id).toMatch(/^[a-z0-9]{1,12}$/u);
 const claim=join(root,"u",layout.locks,"lane-"+id+".lock"),past=new Date(Date.now()-UNIT_CACHE_POLICY_V1.lock.staleMilliseconds*3);utimesSync(claim,past,past);
 return id;
}'''
assert old in s
s=s.replace(old,new)
open(p,"w",encoding="utf-8").write(s)
p=T+"/files/dead-holder-worker.ts"
s=open(p,encoding="utf-8").read()
s=s.replace("`<root> <shared> [seed]`","`<root> <shared>`")
open(p,"w",encoding="utf-8").write(s)
