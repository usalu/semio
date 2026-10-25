import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
const worker=resolve("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/📥️worker/🟦️.ts");
function start(spec:Record<string,unknown>){
 const child=Bun.spawn([process.execPath,worker,JSON.stringify(spec)],{stdin:"pipe",stdout:"pipe",stderr:"pipe",env:{...process.env}});
 const lines:string[]=[];(async()=>{let buffer="";for await(const chunk of child.stdout){buffer+=new TextDecoder().decode(chunk);for(let at=buffer.indexOf("\n");at>=0;at=buffer.indexOf("\n")){lines.push(buffer.slice(0,at));buffer=buffer.slice(at+1);}}})();
 const wait=async(prefix:string,ms:number)=>{const end=Date.now()+ms;while(Date.now()<end){const hit=lines.find(l=>l.startsWith(prefix));if(hit)return hit;await Bun.sleep(50);}return undefined;};
 return {wait,send:(t:string)=>{child.stdin.write(t+"\n");child.stdin.flush();},stop:()=>child.kill()};
}
test("a shared newcomer waits behind a queued writer while a queued shared holder is active (default policy)",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"fair-"));mkdirSync(directory,{recursive:true});const resource="repo:compiler-writer:/fixture/m";
 const holder=start({mode:"hold",queued:true,directory,resource,access:"shared"});expect(await holder.wait("HELD",8000)).toBeDefined();
 const writer=start({mode:"wait",queued:true,directory,resource,access:"exclusive",timeoutMs:60000});await Bun.sleep(1500);
 const newcomers=[0,1,2].map(()=>start({mode:"wait",queued:true,directory,resource,access:"shared",timeoutMs:6000}));
 const results=await Promise.all(newcomers.map(n=>n.wait("ACQUIRED",7000).then(r=>r??"none")));
 console.log("newcomer results",results);
 expect(results.every(r=>r==="none")).toBe(true);
 holder.send("go");expect(await writer.wait("ACQUIRED",8000)).toBeDefined();
 for(const n of newcomers)n.stop();writer.stop();holder.stop();rmSync(directory,{recursive:true,force:true});
},60000);
