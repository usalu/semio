import {Database} from "bun:sqlite";
import {appendFileSync} from "node:fs";
const T=Date.now();
const out=process.env.CAD_TRACE_OUT!,counts=new Map<string,{n:number,first:number,last:number}>(),t0=Date.now();
const short=(s:string)=>s.replace(/^.*\/semio\//,"").replace(/\/Users\/ueli\/Documents\/semio\//g,"");
function stack(){const e=new Error();const lines=String(e.stack).split("\n").slice(2,16).map(l=>short(l.trim()));return lines.join(" <- ");}
function note(kind:string,detail:string){const key=kind+"|"+detail+"|"+stack();const now=Date.now()-t0;const row=counts.get(key);if(row){row.n++;row.last=now;}else counts.set(key,{n:1,first:now,last:now});}
const exec=Database.prototype.exec,prepare=Database.prototype.prepare;
Database.prototype.exec=function(this:any,sql:string,...r:any[]){note("sqlite.exec",String(sql).slice(0,60));try{return (exec as any).call(this,sql,...r);}catch(e:any){note("sqlite.ERROR",String(e?.code??e?.message).slice(0,40)+" "+String(sql).slice(0,40)+" file="+String(this.filename).replace(/^.*\/semio\//,"")+"");throw e;}};
Database.prototype.prepare=function(this:any,sql:string,...r:any[]){note("sqlite.prepare",String(sql).slice(0,80));return (prepare as any).call(this,sql,...r);};
const st=globalThis.setTimeout;(globalThis as any).setTimeout=function(fn:any,ms?:number,...a:any[]){note("setTimeout",String(ms));return (st as any)(fn,ms,...a);};
const flush=()=>{const rows=[...counts.entries()].sort((a,b)=>b[1].n-a[1].n).slice(0,25).map(([k,v])=>`${v.n}x [${v.first}..${v.last}ms] ${k}`);appendFileSync(out,`==== ${new Date().toISOString()} uptime ${Date.now()-t0}ms\n${rows.join("\n")}\n`);};setInterval(flush,15000).unref?.();process.on("exit",flush);
