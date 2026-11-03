import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${R}/🔨️modules/🧮️evaluation/🟦️.ts`);
const a = JSON.parse(readFileSync(path(process.argv[2]),"utf8")).asset;
const sel:any = process.argv[3].startsWith("sym:") ? {kind:"symbol",symbol:process.argv[3].slice(4)} : {kind:"scale",scale:{denominator:Number(process.argv[3])}};
const pose = process.argv[4] ?? a.presentation.defaultPose;
const st = evaluate(a,{kind:"pose",pose},sel);
for (const v of ["elevation","front","plan"]) { console.log("--",v); for (const e of st.shapes[v]) { const p=e.rings.flat(); if(!p.length) continue; const xs=p.map((q:any)=>q.x),ys=p.map((q:any)=>q.y); console.log(" ",e.shape.id.padEnd(22), `x[${Math.min(...xs).toFixed(3)},${Math.max(...xs).toFixed(3)}] y[${Math.min(...ys).toFixed(3)},${Math.max(...ys).toFixed(3)}]`, e.shape.role); } }
