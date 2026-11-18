import {readFileSync,readdirSync,statSync,existsSync,lstatSync} from "node:fs";import {brotliDecompressSync} from "node:zlib";import {createHash} from "node:crypto";
const base="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const OLD="/Users/Shared/semio",NEW="/Users/ueli/Documents/semio";const re=p=>p.startsWith(OLD+"/")||p===OLD?NEW+p.slice(OLD.length):p;
const blocks=new Map();const block=d=>{let b=blocks.get(d);if(!b){try{b=JSON.parse(brotliDecompressSync(readFileSync(base+"ob/"+d+".br")).toString())}catch{b=null}blocks.set(d,b)}return b};
const sha=new Map();const shaOf=p=>{let v=sha.get(p);if(v===undefined){try{v=createHash("sha256").update(readFileSync(p)).digest("hex")}catch{v=null}sha.set(p,v)}return v};
const fileHolds=(p,f)=>{let s;try{s=statSync(p)}catch{return false}if(s.size!==f.size)return false;if(s.mtimeMs===f.mtimeMs&&s.ctimeMs===f.ctimeMs&&s.ino===f.ino)return true;return shaOf(p)===f.sha256};
const culprits=new Map();const out=[];const sel=process.argv[2]||"all";
const names=readdirSync(base).filter(n=>n.endsWith(".json")&&n!=="owner-peaks.json");
let ok=0,bad=0,missingBlock=0,old=0;const ages=[];
for(const n of names){const st=statSync(base+n);let h;try{const t=brotliDecompressSync(readFileSync(base+n)).toString();h=JSON.parse(t.slice(0,t.indexOf("\n")))}catch{continue}
 let rows=[...(h.observations||[])];let miss=false;for(const d of h.blocks||[]){const b=block(d);if(!b){miss=true;break}rows.push(...b)}
 if(miss){missingBlock++;continue}
 const fails=[];for(const r of rows){const p=re(r.path);let holds=true;
  if(r.kind==="file"){holds=fileHolds(p,r.fact)}else if(r.kind==="exists"){holds=existsSync(p)===r.fact}else if(r.kind==="analysis"){holds=r.fact.source?fileHolds(p,r.fact.source):true}
  if(!holds)fails.push(r.kind+" "+p)}
 if(fails.length){bad++;for(const f of new Set(fails))culprits.set(f,(culprits.get(f)||0)+1)}else ok++;
 out.push({n,mtime:st.mtimeMs,rows:rows.length,fails:fails.length});}
console.log({records:names.length,holds:ok,stale:bad,missingBlock});
const top=[...culprits].sort((a,b)=>b[1]-a[1]).slice(0,25);for(const [k,v] of top)console.log(v,k.slice(0,230));
const byHour={};for(const o of out){const k=new Date(o.mtime).toISOString().slice(0,13);const e=byHour[k]??={n:0,stale:0};e.n++;if(o.fails)e.stale++}
console.log(JSON.stringify(byHour));
