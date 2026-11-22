import { loadDoc, measureAsset, featureOf } from "./lib.ts";
const a = loadDoc(process.argv[2]!).asset;
const m: any = measureAsset(a, Number(process.argv[3] ?? 50), undefined, undefined, featureOf(Number(process.argv[3] ?? 50) as never));
const v = m.views[process.argv[4] ?? "elevation"];
console.log(Object.keys(v));
for (const k of Object.keys(v)) if (typeof v[k] !== "object") console.log(k, v[k]); else console.log(k, JSON.stringify(v[k]).slice(0, 300));
