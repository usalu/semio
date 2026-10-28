import {test,expect} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {fileURLToPath} from "node:url";
import {acquireResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {retireArtifactDirectoriesV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";
import {retireOldDashboardExecutablesV1} from "../🟦️.ts";

const unissued:Parameters<typeof retireOldDashboardExecutablesV1>[4]=(directories,options)=>retireArtifactDirectoriesV1(directories,{...options,control:undefined});
const model=fileURLToPath(new URL("./🐍️model.py",import.meta.url));
const digest=(index:number)=>createHash("sha256").update("dashboard-"+index).digest("hex");

/** 🧹️ Retention keeps the current digest and the newest few, and reclaims an older digest only when no process holds it: the Python table names the keep set, the lease table names the holders. */
test("old dashboard executables are reclaimed only when no process holds them",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"dr-"))),parent=join(root,"dashboard-cli","prototype"),leases=join(root,".cache/b/l/m/data"),count=6,current=3,age=(index:number)=>1_700_000_000+index*100;
 mkdirSync(leases,{recursive:true});mkdirSync(parent,{recursive:true});
 try{
  const images=Array.from({length:count},(_,index)=>({identity:digest(index),createdAt:age(index)}));
  for(const image of images){const directory=join(parent,image.identity);mkdirSync(directory);writeFileSync(join(directory,"semio"),image.identity);utimesSync(directory,image.createdAt,image.createdAt);}
  const oracle=Bun.spawnSync(["python3","-I",model],{stdin:Buffer.from(JSON.stringify({images,current:digest(current),keepNewest:2})),stdout:"pipe",stderr:"pipe"});expect(oracle.exitCode,oracle.stderr.toString()).toBe(0);
  const plan=JSON.parse(oracle.stdout.toString()) as {retire:string[]};expect(plan.retire.length).toBeGreaterThan(1);
  const holder=await acquireResourceLease({directory:leases,resource:"artifact:"+join(parent,plan.retire[0]!),mode:"shared",signal:new AbortController().signal}),stop=new AbortController();
  try{
   await retireOldDashboardExecutablesV1(parent,digest(current),root,{signal:stop.signal} as never,unissued);
   for(const image of images){const expected=!plan.retire.includes(image.identity)||image.identity===plan.retire[0];expect(existsSync(join(parent,image.identity)),image.identity).toBe(expected);}
  }finally{holder.release();}
  await retireOldDashboardExecutablesV1(parent,digest(current),root,{signal:stop.signal} as never,unissued);
  for(const image of images)expect(existsSync(join(parent,image.identity)),image.identity).toBe(!plan.retire.includes(image.identity));
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);
