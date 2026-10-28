// stands in for node:fs inside the repository modules: reads of overlaid paths return the overlaid bytes
import * as real from "node:fs";
export * from "node:fs";
const state=globalThis.__overlay;
const key=p=>typeof p==="string"?p:p instanceof URL?p.pathname:undefined;
const encoding=a=>typeof a[0]==="string"?a[0]:a[0]?.encoding;
const fake=(stat,bytes)=>{const s=Object.create(stat);Object.defineProperties(s,{size:{value:bytes.length},mtimeMs:{value:stat.mtimeMs-3.6e6-state.epoch()},ctimeMs:{value:stat.ctimeMs-3.6e6-state.epoch()},ino:{value:stat.ino+1e9+state.epoch()}});return s;};
export function readFileSync(p,...a){const b=state.get(key(p));if(b){state.calls.read++;const e=encoding(a);return e?b.toString(e):Buffer.from(b);}return real.readFileSync(p,...a);}
export function statSync(p,...a){const r=real.statSync(p,...a),b=state.get(key(p));return b&&r?fake(r,b):r;}
export function lstatSync(p,...a){const r=real.lstatSync(p,...a),b=state.get(key(p));return b&&r?fake(r,b):r;}
export default {...real,readFileSync,statSync,lstatSync};
