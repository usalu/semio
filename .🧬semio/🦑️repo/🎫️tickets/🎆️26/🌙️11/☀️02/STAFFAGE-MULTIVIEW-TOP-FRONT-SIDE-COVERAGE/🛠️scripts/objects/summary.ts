import { readFileSync } from "node:fs";
const d = JSON.parse(readFileSync(process.argv[2], "utf8"));
const show = process.argv[3];
for (const [slug, r] of Object.entries<any>(d)) {
  if (show && slug !== show) continue;
  console.log(`== ${slug} dims h${r.dims.heightM} w${r.dims.widthM} d${r.dims.depthM} poses[${r.poses}] clips[${r.clips}] abd${r.abd} tw${r.twist} emit[${r.emitters}] sym${r.symbols.length}`);
  for (const [k, s] of Object.entries<any>(r.scales)) {
    if (s.status) { console.log(`  1:${k} UNAVAILABLE alt=${JSON.stringify(s.alt)}`); continue; }
    const c = s.counts;
    const dm = s.dim.e ? `E${s.dim.e} F${s.dim.f} P${s.dim.p}` : "nodim";
    const flags: string[] = [];
    if (!c.elevation || !c.front || !c.plan) flags.push("MISSING-VIEW");
    if (s.unbound.length) flags.push("unbound" + s.unbound.length);
    if (s.idmiss.length) flags.push("idmiss" + s.idmiss.length);
    if (s.mism.bad !== "0/" + s.mism.bad.split("/")[1]) flags.push("mism " + s.mism.bad + " " + s.mism.first);
    if (s.mism.evalFail || s.mism.empty) flags.push(`fail${s.mism.evalFail ?? 0}/empty${s.mism.empty ?? 0}`);
    const am = Object.entries<any>(s.anchorMiss).filter(([, v]) => v.e > 0.05 || v.f > 0.05 || v.p > 0.05).map(([n, v]) => `${n}${JSON.stringify(v)}`);
    if (am.length) flags.push("anchors " + am.join(" "));
    const mo = Object.entries<any>(s.motion).map(([n, m]) => `${n}[${m.worstPlan ? "P:" + m.worstPlan : ""}${m.worstFront ? " F:" + m.worstFront : ""}]`);
    if (mo.length) flags.push("motion " + mo.join(" "));
    console.log(`  1:${k} v${c.elevation}/${c.front}/${c.plan} ${dm} tol${s.tol} ${flags.join(" | ")}`);
  }
}
