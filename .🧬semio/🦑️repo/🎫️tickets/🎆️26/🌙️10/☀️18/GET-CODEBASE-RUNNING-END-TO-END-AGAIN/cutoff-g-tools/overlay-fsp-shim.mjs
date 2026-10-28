import * as real from "node:fs/promises";
export * from "node:fs/promises";
const state=globalThis.__overlay;
const key=p=>typeof p==="string"?p:p instanceof URL?p.pathname:undefined;
const encoding=a=>typeof a[0]==="string"?a[0]:a[0]?.encoding;
export async function readFile(p,...a){const b=state.get(key(p));if(b){state.calls.read++;const e=encoding(a);return e?b.toString(e):Buffer.from(b);}return real.readFile(p,...a);}
export async function stat(p,...a){const r=await real.stat(p,...a),b=state.get(key(p));if(!b||!r)return r;const s=Object.create(r);Object.defineProperties(s,{size:{value:b.length},mtimeMs:{value:r.mtimeMs-3.6e6-state.epoch()},ctimeMs:{value:r.ctimeMs-3.6e6-state.epoch()},ino:{value:r.ino+1e9+state.epoch()}});return s;}
export default {...real,readFile,stat};
