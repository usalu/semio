import {appendFileSync} from "node:fs";
const log=(s:string)=>{try{appendFileSync("/tmp/r159-fetch.log",`${new Date().toISOString()} pid ${process.pid} ${process.argv.slice(1,3).join(" ").slice(-80)} ${s}\n`)}catch{}};
const real=globalThis.fetch;
(globalThis as any).fetch=async(input:any,init?:any)=>{const u=typeof input==="string"?input:input?.url??String(input);try{return await real(input,init)}catch(e:any){log(`FETCH-FAIL ${u} unix=${init?.unix??""} proxy=${init?.proxy??""} ${String(e?.message).slice(0,100)}\n${String(e?.stack).split("\n").slice(0,8).join("\n")}`);throw e}};
