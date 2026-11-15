/** Structure/thermal solids outside the body union per view (cells): bun author-27-outside.ts <asset> --scale N --source file [--list structure|thermal] */
import { readFileSync } from "node:fs";
import { VIEWS } from "./volumes-lib.ts";
import { Fitter } from "./kit-fitter.ts";
import { openCtx, flag } from "./kit-lib.ts";

const a = process.argv.slice(2);
const scale = Number(flag(a, "--scale"));
const src = flag(a, "--source")!;
const list = flag(a, "--list") ?? "structure";
const c = openCtx(a[0]!, scale, src);
const fit = new Fitter(c, 1100);
const body = c.vol.body as any[];
const bc = body.filter((s) => s.slender !== true && s.texture !== true).map((s) => fit.cellsOf(fit.mesh(s)));
const cover = new Map<string, Set<number>>();
for (const v of VIEWS) cover.set(v, new Set());
for (const cells of bc) for (const v of VIEWS) for (const i of cells.get(v)!) cover.get(v)!.add(i);
const rows: string[] = [];
for (const s of c.vol[list] as any[]) {
  if (s.of !== undefined) continue;
  const cells = fit.cellsOf(fit.mesh(s));
  const out = VIEWS.map((v) => {
    let n = 0;
    for (const i of cells.get(v)!) if (!cover.get(v)!.has(i)) n++;
    return n;
  });
  if (out.some((n) => n > 0)) rows.push(`${s.id.padEnd(26)} ${out.map((n, i) => `${VIEWS[i]} ${n}`).join("  ")}`);
}
console.log(rows.join("\n") || "none outside");
