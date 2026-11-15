import { openCtx } from "./kit-lib.ts";
const c = openCtx("vegetation/miscanthus-grass", 200);
for (const s of c.state.elevation as any[]) if (s.shape.id === "culm-1a-axis") console.log(s.closed, s.rings.length, JSON.stringify(s.rings[0].filter((_: any, i: number) => i % 6 === 0).map((p: any) => [+p.x.toFixed(3), +p.y.toFixed(3)])));
