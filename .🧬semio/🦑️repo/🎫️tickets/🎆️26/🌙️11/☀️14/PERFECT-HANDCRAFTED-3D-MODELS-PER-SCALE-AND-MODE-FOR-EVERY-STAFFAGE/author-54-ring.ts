import { openCtx, drawn } from "./kit-lib.ts";
const [spec, sc, view, id] = [process.argv[2]!, Number(process.argv[3]), process.argv[4] as any, process.argv[5]!];
const c = openCtx(spec, sc);
for (const d of drawn(c, view)) if (d.id === id) for (const r of d.rings) console.log(d.id, r.map((p: any) => `(${p.x.toFixed(3)},${p.y.toFixed(3)})`).join(" "));
const f = c.state.frames[process.argv[6] ?? id]; console.log(JSON.stringify(f));
