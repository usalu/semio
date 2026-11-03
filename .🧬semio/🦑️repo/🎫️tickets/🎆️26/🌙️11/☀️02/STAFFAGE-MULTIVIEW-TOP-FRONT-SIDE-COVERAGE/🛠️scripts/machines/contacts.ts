import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "./lib.ts";
import { extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const ids = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const write = process.argv.includes("--write");
for (const id of ids) {
  const file = fileOf(id);
  const doc = JSON.parse(readFileSync(file, "utf8"));
  const a = doc.asset;
  const state: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } } as any);
  const kids = (b: string): string[] => [b, ...a.rig.bones.filter((x: any) => x.parent === b).flatMap((x: any) => kids(x.id))];
  const out: any[] = [];
  let lastHalf: number | undefined;
  const split = new Map<string, string[]>();
  for (const an of a.anchors) {
    if (an.kind !== "contact" || !/^contact-/.test(an.id)) { out.push(an); continue; }
    const set = new Set(kids(an.bone));
    const zs = state.shapes.front.filter((s: any) => set.has(s.shape.bone) && /tyre|tire|wheel|track-belt/.test(s.shape.id) && s.shape.role !== "ground").map((s: any) => { const e = extentOf([s], true)!; return +((e.minX + e.maxX) / 2).toFixed(3); });
    const pos = [...new Set(zs.filter((z: number) => z > 0.05))].sort((p: number, q: number) => p - q);
    const widest = pos[pos.length - 1];
    const wheel = state.shapes.front.filter((s: any) => set.has(s.shape.bone) && /tyre|tire|wheel/.test(s.shape.id));
    const half0 = widest ?? lastHalf;
    if (widest === undefined && half0 === undefined) { console.log(id, an.id, "no lateral wheel shapes: unchanged"); out.push(an); continue; }
    const half = half0!; lastHalf = half;
    const names = [`${an.id}-l`, `${an.id}-r`];
    split.set(an.id, names);
    for (const [n, z, side] of [[names[0], half, ["left", "links"]], [names[1], -half, ["right", "rechts"]]] as const) out.push({ ...an, id: n, z, title: { en: `${an.title.en} (${side[0]})`, de: `${an.title.de} (${side[1]})` }, ...(an.footprintWidthM !== undefined ? { footprintWidthM: +(an.footprintWidthM / 2).toFixed(4) } : {}) });
    console.log(id, an.id, "->", names.join(","), "z", half, "candidates", pos.join(","));
  }
  a.anchors = out;
  for (const p of a.poses) if (p.contacts) p.contacts = p.contacts.flatMap((c: string) => split.get(c) ?? [c]);
  for (const c of a.clips) if (c.contacts) c.contacts = c.contacts.flatMap((x: any) => (split.get(x.anchor) ?? [x.anchor]).map((n) => ({ ...x, anchor: n })));
  if (write) { writeFileSync(file, JSON.stringify(doc, null, 1) + "\n"); console.log("written", id); }
}
