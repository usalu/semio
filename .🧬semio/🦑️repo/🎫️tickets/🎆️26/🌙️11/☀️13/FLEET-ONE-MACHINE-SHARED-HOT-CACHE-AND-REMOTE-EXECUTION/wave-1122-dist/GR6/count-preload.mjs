import fs from "node:fs";
const c={intervals8000:0,cleared:0,factsReads:0,fetch:0,fetchPaths:{}};
const si=globalThis.setInterval;globalThis.setInterval=(f,ms,...a)=>{if(ms===8000)c.intervals8000++;return si(f,ms,...a);};
const ci=globalThis.clearInterval;globalThis.clearInterval=t=>{c.cleared++;return ci(t);};
const rf=fs.readFileSync;fs.readFileSync=function(p,...a){if(typeof p==="string"&&p.endsWith("cache-facts.json"))c.factsReads++;return rf.call(this,p,...a);};
const f=globalThis.fetch;globalThis.fetch=(u,...a)=>{c.fetch++;const k=String(u).replace(/[0-9a-f]{20,}/g,"H").replace(/^https?:\/\/[^/]+/,"").slice(0,60);c.fetchPaths[k]=(c.fetchPaths[k]||0)+1;return f(u,...a);};
process.on("exit",()=>{console.error("[COUNT]",JSON.stringify(c));});
