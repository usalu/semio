/** 📐️ World bounds of body solids of one scale: bun figure-solids.ts <asset> --scale N --ids regex [--src source.json] */
import { openCtx, flag } from "./kit-lib.ts";
import { scaleMeshes } from "./volumes-lib.ts";
const args = process.argv.slice(2);
const c = openCtx(args[0]!, Number(flag(args, "--scale", "50")), flag(args, "--src"));
const re = new RegExp(flag(args, "--ids", ".")!);
const sm = scaleMeshes(c.m, c.scale, c.m.volumes[String(c.scale)]);
for (const m of sm.body as any[]) { if (!re.test(m.id)) continue; const p: Float64Array = m.world; const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9]; for (let k = 0; k < p.length; k += 3) for (let a = 0; a < 3; a++) { lo[a] = Math.min(lo[a]!, p[k + a]!); hi[a] = Math.max(hi[a]!, p[k + a]!); } console.log(m.id.padEnd(28), lo.map((v) => v.toFixed(3)).join(" "), "|", hi.map((v) => v.toFixed(3)).join(" ")); }
