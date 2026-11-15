import { drawn, flag, openCtx, VIEWS } from "./kit-lib.ts";
const args = process.argv.slice(2);
const c = openCtx(args[0]!, Number(flag(args, "--scale", "50")));
const fr = (c.state as any).frames;
const r = (x: number[]) => x.map((q) => q.toFixed(3)).join(",");
for (const [k, f] of Object.entries<any>(fr)) console.log(k, "o", r(f.o), "u", r(f.u), "v", r(f.v), "w", r(f.w));
for (const view of VIEWS) {
  const g: Record<string, string[]> = {};
  for (const s of drawn(c, view)) (g[s.bone] ??= []).push(`${s.id}/${s.region ?? "-"}`);
  console.log(view, JSON.stringify(g));
}
