import { readdirSync } from "node:fs";
import { CAT, load, bands } from "./lib.ts";
for (const d of readdirSync(CAT)) {
  const { asset: a } = load(d); const emReg = new Set(a.thermal.emitters.map((e: any) => e.region).filter(Boolean));
  const out: string[] = [];
  for (const den of bands(a)) {
    const g = a.representations[String(den)].geometry; const regs = (v: string) => new Set(g[v].flatMap((s: any) => s.region ? [s.region] : (s.regions ?? [])));
    const e = regs("elevation");
    for (const v of ["front", "plan"]) { const r = regs(v); const miss = [...e].filter((x) => !r.has(x)); const em = [...emReg].filter((x: any) => !r.has(x)); if (miss.length || em.length) out.push(`${den} ${v}: lacks [${miss}] emitterRegionMissing [${em}]`); }
  }
  console.log(d.padEnd(24), out.length ? out.join(" | ") : "ok");
}
