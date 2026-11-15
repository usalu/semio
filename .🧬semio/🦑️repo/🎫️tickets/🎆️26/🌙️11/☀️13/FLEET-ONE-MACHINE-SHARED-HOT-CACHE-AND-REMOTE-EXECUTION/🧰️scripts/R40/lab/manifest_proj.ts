const jobs=JSON.parse(await Bun.file("mjobs.json").text());
function proj(t:any){const pk=t.package??{};const keep:any={};for(const k of ["name","version","edition","rust-version","links","build","resolver","autolib","autobins","lints","publish"])if(k in pk)keep[k]=pk[k];
 const tg:any={};for(const [k,v] of Object.entries<any>(t.target??{}))tg[k]={d:v.dependencies,b:v["build-dependencies"]};
 return JSON.stringify({package:keep,lib:t.lib,features:t.features,dependencies:t.dependencies,"build-dependencies":t["build-dependencies"],target:tg,lints:t.lints,bin:t.bin,script:pk.metadata?.semio?.["build-script"]});}
const proc=Bun.spawn(["git","--no-optional-locks","-C","/Users/Shared/semio","cat-file","--batch"],{stdin:"pipe",stdout:"pipe",stderr:"ignore"});
let buf=new Uint8Array(0);const rd=proc.stdout.getReader();
async function fill(n:number){while(buf.length<n){const {value,done}=await rd.read();if(done)throw Error("eof");const m=new Uint8Array(buf.length+value.length);m.set(buf);m.set(value,buf.length);buf=m;}}
async function line(){for(;;){const i=buf.indexOf(10);if(i>=0){const s=new TextDecoder().decode(buf.slice(0,i));buf=buf.slice(i+1);return s;}await fill(buf.length+1);}}
async function get(spec:string){proc.stdin.write(spec+"\n");proc.stdin.flush();const h=await line();if(h.endsWith(" missing"))return null;const n=Number(h.split(" ")[2]);await fill(n+1);const o=buf.slice(0,n);buf=buf.slice(n+1);return new TextDecoder().decode(o);}
let equal=0,diff=0,fail=0,added=0;
for(const j of jobs){const a=await get(j.a),b=await get(j.b);if(a===null||b===null){added++;continue;}
 try{if(proj(Bun.TOML.parse(a))===proj(Bun.TOML.parse(b)))equal++;else diff++;}catch{fail++;}}
console.log(JSON.stringify({edits:jobs.length,projectionEqual:equal,projectionChanged:diff,parseFail:fail,addedOrRemoved:added}));
proc.stdin.end();
