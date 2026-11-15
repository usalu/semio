import {interfaceDigestV1} from "./iface/🟦️.ts";
const jobs=JSON.parse(await Bun.file(process.argv[2]).text()) as {id:string;files:{name:string;before:string|null;after:string|null}[]}[];
const proc=Bun.spawn(["git","--no-optional-locks","-C","/Users/Shared/semio","cat-file","--batch"],{stdin:"pipe",stdout:"pipe",stderr:"ignore"});
let buf=new Uint8Array(0);const reader=proc.stdout.getReader();
async function fill(n:number){while(buf.length<n){const {value,done}=await reader.read();if(done)throw Error("eof");const m=new Uint8Array(buf.length+value.length);m.set(buf);m.set(value,buf.length);buf=m;}}
async function line():Promise<string>{for(;;){const i=buf.indexOf(10);if(i>=0){const s=new TextDecoder().decode(buf.slice(0,i));buf=buf.slice(i+1);return s;}await fill(buf.length+1);}}
const cache=new Map<string,Uint8Array|null>();
async function get(spec:string):Promise<Uint8Array|null>{
 if(cache.has(spec))return cache.get(spec)!;
 proc.stdin.write(spec+"\n");proc.stdin.flush();
 const head=await line();let out:Uint8Array|null=null;
 if(!head.endsWith(" missing")){const size=Number(head.split(" ")[2]);await fill(size+1);out=buf.slice(0,size);buf=buf.slice(size+1);}
 if(cache.size>20000)cache.clear();cache.set(spec,out);return out;
}
const result:Record<string,{before:string;after:string;same:boolean;bodies:number}>={};
for(const job of jobs){
 const b=[],a=[];
 for(const f of job.files){
  b.push({name:f.name,bytes:f.before?await get(f.before):null});
  a.push({name:f.name,bytes:f.after?await get(f.after):null});
 }
 const db=interfaceDigestV1(b),da=interfaceDigestV1(a);
 result[job.id]={before:db.digest,after:da.digest,same:db.digest===da.digest,bodies:da.bodies};
}
await Bun.write(process.argv[3],JSON.stringify(result));
proc.stdin.end();
