import { readFileSync, writeFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const file = (slug: string) => `${L}/🗂️catalog/👤️humans/${slug}/🔣️.json`;
const clone = <T>(v: T): T => JSON.parse(JSON.stringify(v));
const r5 = (n: number) => +n.toFixed(5);

type Range = [number, number];

function lateralOf(rep: any, bone: string, ids: string[]): Range | null {
  const pick = (id?: string): Range | null => {
    const match = (s: any) => s.bone === bone && (id === undefined || s.id === id);
    for (const v of ["front", "plan"]) for (const s of rep.geometry[v]) if (match(s) && s.solid?.kind === "extruded") return [s.solid.z0, s.solid.z1];
    for (const s of rep.geometry.front) if (match(s) && s.kind === "ellipse" && !s.solid) return [s.cx - s.rx, s.cx + s.rx];
    for (const s of rep.geometry.plan) if (match(s) && s.kind === "capsule") return [s.y0 - Math.max(s.r0, s.r1), s.y0 + Math.max(s.r0, s.r1)];
    for (const s of rep.geometry.plan) if (match(s) && s.kind === "ellipse" && !s.solid) return [s.cy - s.ry, s.cy + s.ry];
    return null;
  };
  for (const id of ids) {
    const r = pick(id);
    if (r) return r;
  }
  return pick();
}

export function solidify(slug: string, only?: string[]): void {
  const doc = JSON.parse(readFileSync(file(slug), "utf8"));
  const a = doc.asset;
  const targets = new Map<string, Set<string>>();
  for (const issue of validateAsset(a)) {
    if (issue.code !== "lateral-ambiguous") continue;
    const m = /\(([^)]*)\) on bone "([^"]+)"/.exec(issue.message.en);
    if (!m) continue;
    const set = targets.get(m[2]!) ?? new Set<string>();
    for (const id of m[1]!.split(",").map((s) => s.trim())) set.add(id);
    targets.set(m[2]!, set);
  }
  const log: string[] = [];
  for (const rep of Object.values<any>(a.representations)) {
    if (rep.status !== "authored") continue;
    for (const [bone, ids] of targets) {
      if (only && only.length > 0 && !only.includes(bone)) continue;
      const idList = [...ids];
      const range = lateralOf(rep, bone, idList);
      const sibling = lateralOf(rep, bone, []);
      const fallback = range ?? sibling;
      for (const view of ["elevation", "front", "plan"] as const) {
        const list: any[] = rep.geometry[view];
        for (let i = 0; i < list.length; i++) {
          const s = list[i];
          if (s.bone !== bone || !ids.has(s.id) || s.solid || s.role === "ground") continue;
          const own = lateralOf(rep, bone, [s.id]) ?? fallback;
          if (!own) {
            log.push(`${rep.denominator} ${view} ${s.id}: no lateral range`);
            continue;
          }
          const solid = { kind: "extruded", z0: r5(own[0]), z1: r5(own[1]) };
          if (view === "elevation") {
            s.solid = solid;
            continue;
          }
          const twin = rep.geometry.elevation.find((e: any) => e.id === s.id && e.bone === bone) ?? rep.geometry.front.find((e: any) => e.id === s.id && e.bone === bone && e.solid && e.kind !== "ellipse") ?? rep.geometry.elevation.find((e: any) => e.bone === bone && e.role === "body");
          if (!twin) {
            if (s.kind === "ellipse" && view === "front") {
              list[i] = { id: s.id, bone: s.bone, role: s.role, z: s.z, filled: s.filled, tone: s.tone, ...(s.region ? { region: s.region } : {}), ...(s.material ? { material: s.material } : {}), ...(s.hatch ? { hatch: s.hatch } : {}), kind: "ellipse", cx: 0, cy: 0, rx: s.rx, ry: s.ry, solid };
              continue;
            }
            log.push(`${rep.denominator} ${view} ${s.id}: no elevation twin`);
            continue;
          }
          const keep: any = {};
          for (const k of ["region", "regions", "material", "hatch", "penMm", "filled", "tone", "role", "z"]) if (s[k] !== undefined) keep[k] = s[k];
          const profile = clone(twin);
          delete profile.solid;
          list[i] = { ...profile, ...keep, id: s.id, bone: s.bone, solid };
        }
      }
    }
  }
  writeFileSync(file(slug), JSON.stringify(doc, null, 2) + "\n");
  console.log(slug, "targets", [...targets].map(([b, i]) => `${b}:${[...i].length}`).join(" "), log.length ? "\n  " + [...new Set(log)].join("\n  ") : "");
}

if (import.meta.main) solidify(process.argv[2]!, process.argv.slice(3));
