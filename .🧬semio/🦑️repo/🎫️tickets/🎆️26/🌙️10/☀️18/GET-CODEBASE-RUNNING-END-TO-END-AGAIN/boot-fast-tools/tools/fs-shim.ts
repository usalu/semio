import * as real from "node:fs";
export * from "node:fs";
const now=()=>Bun.nanoseconds()/1e6;
const log=(globalThis as any).__bfCalls as {api:string;path:string;t:number;ms:number;by:string}[];
function wrap<F extends (...a:any[])=>any>(api:string,fn:F):F{
 return function(this:unknown,...args:any[]){const s=now();try{return fn.apply(this,args);}finally{const p=args[0];if(typeof p==="string"||p instanceof URL){const o=Error.stackTraceLimit;Error.stackTraceLimit=3;const st=new Error().stack?.split("\n")[2]??"";Error.stackTraceLimit=o;log.push({api,path:String(p),t:s,ms:now()-s,by:st.trim().replace(/^at /,"")});}}} as F;
}
export const readFileSync=wrap("readFileSync",real.readFileSync);
export const readdirSync=wrap("readdirSync",real.readdirSync);
export const lstatSync=wrap("lstatSync",real.lstatSync);
export const statSync=wrap("statSync",real.statSync);
export const existsSync=wrap("existsSync",real.existsSync);
export const realpathSync=wrap("realpathSync",real.realpathSync);
export const openSync=wrap("openSync",real.openSync);
export const opendirSync=wrap("opendirSync",real.opendirSync);
export const readlinkSync=wrap("readlinkSync",real.readlinkSync);
export default {...real,readFileSync,readdirSync,lstatSync,statSync,existsSync,realpathSync,openSync,opendirSync,readlinkSync};
