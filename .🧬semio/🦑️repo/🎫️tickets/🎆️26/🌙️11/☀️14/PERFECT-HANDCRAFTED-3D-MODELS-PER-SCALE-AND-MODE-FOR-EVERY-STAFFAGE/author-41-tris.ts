/** 🔺️ Author 41: triangle counts per solid and list of one scale. Usage: bun author-41-tris.ts <asset> <scale> [list] */
import { openCtx } from "./kit-lib.ts";
import { scaleMeshes } from "./volumes-lib.ts";
const [spec, scaleS, only] = process.argv.slice(2);
const c = openCtx(spec!, Number(scaleS));
const sm: any = scaleMeshes(c.m, Number(scaleS), c.vol);
for (const l of ["body", "thermal", "structure"]) {
  if (only && only !== l) continue;
  const ms = sm[l] as any[];
  const t = ms.map((m) => [m.id, m.indices.length / 3] as const);
  console.log(l, ms.length, "total", t.reduce((a, b) => a + b[1], 0), t.sort((a, b) => b[1] - a[1]).slice(0, 6).map((x) => `${x[0]}:${x[1]}`).join(" "));
}
