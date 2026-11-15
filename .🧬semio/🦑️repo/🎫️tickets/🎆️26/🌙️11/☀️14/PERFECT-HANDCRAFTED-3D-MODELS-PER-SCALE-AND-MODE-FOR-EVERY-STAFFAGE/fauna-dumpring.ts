import { openCtx, drawn } from "./kit-lib.ts";
const [spec, scaleS, view, ...ids] = process.argv.slice(2);
const c = openCtx(spec!, Number(scaleS));
for (const s of drawn(c, view as any)) if (ids.length === 0 || ids.includes(s.id)) for (const r of s.rings) console.log(s.id, (s as any).filled, r.length, r.map((p) => `${p.x.toFixed(3)},${p.y.toFixed(3)}`).join(" "));
