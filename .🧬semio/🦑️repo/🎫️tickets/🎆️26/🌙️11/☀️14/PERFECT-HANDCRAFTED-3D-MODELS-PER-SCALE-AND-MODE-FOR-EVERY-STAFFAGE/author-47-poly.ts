import { openCtx, drawn, flag } from "./kit-lib.ts";
const a = process.argv.slice(2);
const c = openCtx(a[0]!, Number(flag(a, "--scale")));
for (const v of ["elevation","front","plan"] as const) for (const s of drawn(c, v)) if (a.slice(2).some((id)=>s.id===id)) console.log(v, s.id, s.rings[0]!.map((p)=>`${p.x.toFixed(3)},${p.y.toFixed(3)}`).join(" ").slice(0,900));
