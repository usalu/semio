import { L, load, save } from "./lib.ts";
import { simplifyShape } from "./simplify.ts";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${L}/🔨️modules/🖨️representation/🟦️.ts`);
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const clone = (x: any) => JSON.parse(JSON.stringify(x));
const IGNORED = ["view-twin-missing", "solid-on-elevation", "lateral-ambiguous"];

export type Spec = { slug: string; den: 200 | 500; drop: RegExp; en: Record<string, string>; de: Record<string, string>; minFeatureM: number; epsM: number; pens: any; range: [number, number] };

export function bandOf(a: any, spec: Spec): any {
  const base = a.representations[process.env.BASE ?? "100"];
  const rep: any = { status: "authored", denominator: spec.den, validRange: { minDenominator: spec.range[0], maxDenominator: spec.range[1] }, pens: spec.pens, hatches: [], geometry: { elevation: [], front: [], plan: [] }, decisions: {} };
  for (const v of ["elevation", "front", "plan"] as const) {
    for (const s0 of base.geometry[v]) {
      const s = clone(s0);
      if (s.role !== "body" && s.role !== "ground") continue;
      if (spec.drop.test(s.id)) continue;
      delete s.hatch; delete s.penMm;
      if (s.kind === "capsule") { s.r0 = Math.max(s.r0, spec.minFeatureM / 2); s.r1 = Math.max(s.r1, spec.minFeatureM / 2); }
      if (s.kind === "ellipse") { s.rx = Math.max(s.rx, spec.minFeatureM / 2); s.ry = Math.max(s.ry, spec.minFeatureM / 2); }
      if (!s.solid || s.solid.kind === "extruded") simplifyShape(s, spec.epsM);
      if (s.solid?.kind === "revolved" && spec.den === 500 && s.kind === "poly") for (const p of s.points) p[1] = +(p[1] * 1.12).toFixed(4);
      rep.geometry[v].push(s);
    }
  }
  const drawn = new Set<string>();
  for (const v of ["elevation", "front", "plan"] as const) for (const s of rep.geometry[v]) { if (s.region) drawn.add(s.region); for (const r of s.regions ?? []) drawn.add(r); }
  for (const id of a.thermal.regions.filter((r: any) => !r.hidden && !drawn.has(r.id)).map((r: any) => r.id)) {
    for (const v of ["elevation", "front", "plan"] as const) {
      const src = base.geometry[v].find((s: any) => s.region === id || (s.regions ?? []).includes(id));
      if (!src) continue;
      const host = rep.geometry[v].find((s: any) => s.bone === src.bone && s.role === "body") ?? rep.geometry[v].find((s: any) => s.role === "body");
      host.regions = [...new Set([...(host.regions ?? (host.region ? [host.region] : [])), id])]; delete host.region;
    }
  }
  rep.decisions = Object.fromEntries(Object.keys(spec.en).map((k) => [k, { en: spec.en[k], de: spec.de[k] }]));
  return rep;
}

export function build(spec: Spec, write: boolean): void {
  const doc = load(spec.slug); const a = doc.asset;
  const rep = bandOf(a, spec);
  a.representations[String(spec.den)] = rep;
  if (write) save(spec.slug, doc);
  const issues = validateAsset(a).filter((i: any) => !IGNORED.includes(i.code));
  console.log(spec.slug, spec.den, write ? "WRITTEN" : "dry-run", "shapes", ["elevation", "front", "plan"].map((v) => rep.geometry[v].length).join("/"), "issues", issues.length);
  for (const i of issues.slice(0, 12)) console.log("  ", i.code, i.path, (i.message?.en ?? i.message ?? "").slice(0, 220));
  const s = measureSuitability(a, rep, spec.den, DEFAULT_PRINT_PROFILE);
  console.log("  suitable", s.suitable, "minFeature", s.minFeatureMm?.toFixed?.(3), "minGap", s.minGapMm, "violations", s.violations.length);
  for (const v of s.violations.slice(0, 10)) console.log("  ", JSON.stringify(v).slice(0, 220));
}
