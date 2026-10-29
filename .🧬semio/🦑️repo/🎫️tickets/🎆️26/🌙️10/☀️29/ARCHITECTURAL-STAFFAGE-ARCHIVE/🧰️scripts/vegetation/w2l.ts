// Scratch: converts shapes/anchors flagged "w":1 from world rest coordinates to bone-local coordinates (rigid transform only), then writes one-line-per-element JSON.
import { readFileSync, writeFileSync } from "node:fs";
import { apply, compose, invert, solveRig } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
const [, , output, ...inputs] = process.argv;
const asset: any = Object.assign({}, ...inputs.map((i) => JSON.parse(readFileSync(i, "utf8"))));
const world = solveRig(asset.rig, {}, { x: 0, y: 0, angleDeg: 0 });
const r = (n: number) => Math.round(n * 10000) / 10000;
const inv = (bone: string) => invert(world[bone]!);
const pt = (bone: string, x: number, y: number): [number, number] => { const p = apply(inv(bone), x, y); return [r(p.x), r(p.y)]; };

const rad = (d: number) => (d * Math.PI) / 180;
const f4 = (n: number) => String(Math.round(n * 10000) / 10000);
const P = (x: number, y: number) => `${f4(x)} ${f4(y)}`;
function expand(s: any): void {
  if (s.kind === "cloud") {
    const { cx, cy, rx, ry, lobes } = s; const rot = rad(s.rot ?? 0); const a0 = rad(s.a0 ?? 0); const waist = s.waist ?? 0.82; const bulge = s.bulge ?? 1.28; const under = s.under ?? 0.85; const n = lobes.length;
    const at = (ang: number, k: number) => { const ex = rx * k * Math.cos(ang); const ey = ry * k * Math.sin(ang); return [cx + ex * Math.cos(rot) - ey * Math.sin(rot), cy + ex * Math.sin(rot) + ey * Math.cos(rot)]; };
    const v = (i: number) => at(a0 + (i * 2 * Math.PI) / n, waist * (lobes[(i + n - 1) % n] + lobes[i % n]) / 2);
    const d: string[] = [`M ${P(...(v(0) as [number, number]))}`];
    for (let i = 0; i < n; i++) {
      const mid = a0 + ((i + 0.5) * 2 * Math.PI) / n; const low = Math.sin(mid) < -0.2;
      const c = at(mid, bulge * lobes[i] * (low ? under : 1));
      d.push(`Q ${P(c[0], c[1])} ${P(...(v(i + 1) as [number, number]))}`);
    }
    s.kind = "path"; s.d = d.join(" ") + " Z"; for (const k of ["cx", "cy", "rx", "ry", "lobes", "rot", "a0", "waist", "bulge", "under"]) delete s[k];
  } else if (s.kind === "taper") {
    const [x0, y0] = s.p0; const [x1, y1] = s.p1; const w0 = s.w0; const w1 = s.w1; const bow = s.bow ?? 0; const wob: number[] = s.wob ?? [1, 1];
    const L = Math.hypot(x1 - x0, y1 - y0); const ux = (x1 - x0) / L; const uy = (y1 - y0) / L; const nx = -uy; const ny = ux;
    const wAt = (t: number) => w0 + (w1 - w0) * t;
    const edge = (side: number) => [0, 1 / 3, 2 / 3, 1].map((t, i) => { const m = i === 1 ? wob[0]! : i === 2 ? wob[1]! : 1; const off = side * (wAt(t) / 2) * m + bow * Math.sin(Math.PI * t); return [x0 + ux * L * t + nx * off, y0 + uy * L * t + ny * off]; });
    const l = edge(1); const r = edge(-1);
    const cap = s.round1 ? `Q ${P(x1 + ux * w1 * 0.7, y1 + uy * w1 * 0.7)} ${P(r[3]![0]!, r[3]![1]!)}` : `L ${P(r[3]![0]!, r[3]![1]!)}`;
    s.kind = "path"; s.d = `M ${P(l[0]![0]!, l[0]![1]!)} C ${P(l[1]![0]!, l[1]![1]!)} ${P(l[2]![0]!, l[2]![1]!)} ${P(l[3]![0]!, l[3]![1]!)} ${cap} C ${P(r[2]![0]!, r[2]![1]!)} ${P(r[1]![0]!, r[1]![1]!)} ${P(r[0]![0]!, r[0]![1]!)} Z`;
    for (const k of ["p0", "p1", "w0", "w1", "bow", "wob", "round1"]) delete s[k];
  } else if (s.kind === "marks") {
    const curl = s.curl ?? 0.3; const d: string[] = []; const [ox, oy] = s.c ?? [0, 0]; const [sx, sy] = s.sc ?? [1, 1];
    (s.marks as number[][]).forEach(([x0, y0, a, len, c], i) => { const x = x0! * sx + ox; const y = y0! * sy + oy; const ex = x + len! * Math.cos(rad(a!)); const ey = y + len! * Math.sin(rad(a!)); const k = (c ?? (i % 2 ? curl : -curl)) * len!; d.push(`M ${P(x, y)} Q ${P((x + ex) / 2 - Math.sin(rad(a!)) * k, (y + ey) / 2 + Math.cos(rad(a!)) * k)} ${P(ex, ey)}`); });
    s.kind = "path"; s.d = d.join(" "); delete s.marks; delete s.curl; delete s.c; delete s.sc;
  }
}
function path(bone: string, d: string): string {
  const tokens = d.match(/[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?/g)!;
  const out: string[] = []; let i = 0; let cmd = "";
  while (i < tokens.length) {
    const t = tokens[i]!;
    if (/[MLCQZ]/.test(t)) { cmd = t; out.push(t); i++; if (t === "Z") continue; }
    const n = cmd === "C" ? 3 : cmd === "Q" ? 2 : 1;
    for (let k = 0; k < n; k++) { const [x, y] = pt(bone, Number(tokens[i]), Number(tokens[i + 1])); out.push(`${x} ${y}`); i += 2; }
    if (!/[MLCQZ]/.test(tokens[i] ?? "M") || (tokens[i] === undefined)) { /* implicit repeat of the same command */ }
  }
  return out.join(" ");
}
// Joint plugs: where two tapered woody pieces of one region meet end to end, each piece gets a short plug that reaches into its neighbour (same region, bone and z), so each cap outline lies inside the neighbour's silhouette and is hidden: the chain reads as one continuous form.
{
  const tapers: any[] = asset.geometry.elevation.filter((s: any) => s.w && s.kind === "taper" && !s.noJoin);
  const ends = (s: any) => [{ end: 0, p: s.p0 as number[], q: s.p1 as number[], w: s.w0 as number }, { end: 1, p: s.p1 as number[], q: s.p0 as number[], w: s.w1 as number }];
  const bones: Record<string, any> = Object.fromEntries(asset.rig.bones.map((bn: any) => [bn.id, bn]));
  const adjacent = (x: string, y: string) => x === y || bones[x]?.parent === y || bones[y]?.parent === x;
  const plugs: any[] = []; const used = new Set<string>();
  const plug = (s: any, e: { end: number; p: number[]; q: number[]; w: number }, zLow: number) => {
    const L = Math.hypot(e.p[0]! - e.q[0]!, e.p[1]! - e.q[1]!); const ux = (e.p[0]! - e.q[0]!) / L; const uy = (e.p[1]! - e.q[1]!) / L;
    plugs.push({ id: `${s.id}-plug${e.end}`, bone: s.bone, role: "body", lod: 0, z: zLow - 0.04, filled: true, tone: s.tone, kind: "taper", w: 1, p0: [e.p[0]! - ux * 0.12 * e.w, e.p[1]! - uy * 0.12 * e.w], p1: [e.p[0]! + ux * 0.3 * e.w, e.p[1]! + uy * 0.3 * e.w], w0: e.w * 0.97, w1: e.w * 0.86, noJoin: true, region: s.region, material: s.material });
  };
  for (let i = 0; i < tapers.length; i++) for (let j = i + 1; j < tapers.length; j++) {
    const a = tapers[i]; const b = tapers[j];
    if (!a.region || a.region !== b.region || !adjacent(a.bone, b.bone)) continue;
    for (const ea of ends(a)) for (const eb of ends(b)) {
      const d = Math.hypot(ea.p[0]! - eb.p[0]!, ea.p[1]! - eb.p[1]!); const w = Math.max(ea.w, eb.w);
      if (d > 0.35 * w || used.has(`${a.id}${ea.end}`) || used.has(`${b.id}${eb.end}`)) continue;
      used.add(`${a.id}${ea.end}`); used.add(`${b.id}${eb.end}`);
      const zLow = Math.min(a.z, b.z); plug(a, ea, zLow); plug(b, eb, zLow);
    }
  }
  asset.geometry.elevation.push(...plugs);
  console.log("[DEBUG] joint plugs", asset.id, plugs.length);
}
for (const s of asset.geometry.elevation) {
  if (!s.w) continue; delete s.w;
  expand(s);
  const b = s.bone;
  if (s.kind === "path") s.d = path(b, s.d);
  else if (s.kind === "poly") s.points = s.points.map(([x, y]: number[]) => pt(b, x, y));
  else if (s.kind === "capsule") { [s.x0, s.y0] = pt(b, s.x0, s.y0); [s.x1, s.y1] = pt(b, s.x1, s.y1); }
  else if (s.kind === "ellipse") { [s.cx, s.cy] = pt(b, s.cx, s.cy); const rot = (Math.atan2(world[b]![1], world[b]![0]) * 180) / Math.PI; s.rotationDeg = r((s.rotationDeg ?? 0) - rot); }
  else throw new Error(`unsupported world shape ${s.id}`);
}
for (const a of asset.anchors) if (a.w) { delete a.w; [a.x, a.y] = pt(a.bone, a.x, a.y); }
const line = (v: unknown) => JSON.stringify(v);
const parts: string[] = [];
for (const [k, v] of Object.entries(asset)) {
  if (k === "geometry") parts.push(`  "geometry": {\n    "elevation": [\n${asset.geometry.elevation.map((s: unknown) => "      " + line(s)).join(",\n")}\n    ]${asset.geometry.plan ? `,\n    "plan": ${line(asset.geometry.plan)}` : ""}\n  }`);
  else if (["rig"].includes(k)) parts.push(`  "rig": {\n    "kind": ${line(asset.rig.kind)},\n    "root": ${line(asset.rig.root)},\n    "bones": [\n${asset.rig.bones.map((s: unknown) => "      " + line(s)).join(",\n")}\n    ]\n  }`);
  else if (["materials", "anchors", "poses", "clips"].includes(k)) parts.push(`  ${line(k)}: [\n${(v as unknown[]).map((s) => "    " + line(s)).join(",\n")}\n  ]`);
  else if (k === "thermal") parts.push(`  "thermal": {\n    "environment": ${line(asset.thermal.environment)},\n${asset.thermal.baseMet !== undefined ? `    "baseMet": ${asset.thermal.baseMet},\n` : ""}    "regions": [\n${asset.thermal.regions.map((s: unknown) => "      " + line(s)).join(",\n")}\n    ],\n    "emitters": [\n${asset.thermal.emitters.map((s: unknown) => "      " + line(s)).join(",\n")}\n    ],\n    "assumptions": ${line(asset.thermal.assumptions)}\n  }`);
  else if (k === "provenance") parts.push(`  "provenance": {\n    "author": ${line(asset.provenance.author)}, "method": "handcrafted", "created": ${line(asset.provenance.created)},\n    "references": [\n${asset.provenance.references.map((s: unknown) => "      " + line(s)).join(",\n")}\n    ],\n    "notes": ${line(asset.provenance.notes)}\n  }`);
  else parts.push(`  ${line(k)}: ${line(v)}`);
}
writeFileSync(output!, `{\n${parts.join(",\n")}\n}\n`);
console.log("[DEBUG] converted", asset.id, asset.geometry.elevation.length, "shapes", asset.rig.bones.length, "bones");
