/** 🧤️ One body in three views for a part that a silhouette round traced as a thin plate per view (a gripping hand): `bun bodies.ts <id|file.json> --ops <bodies.json> [--write] [--out <file>]` redraws the part from the volume solids of its band. Every entry `{ scale, bone, paint, "views"?: [view…], "solids"?: [id…], "replace"?: [id…], "sweep"?: { <solid id>: "u" | "v" | "w" } }` takes the body solids of `volumes.<scale>` on `bone` (or those named), tessellates each in the bone frame and draws it as the tightest prism around it: the convex outline of the solid seen along one bone axis (`u` along the bone, `v` its normal, `w` lateral; the axis whose prism has the smallest volume, unless `sweep` names it), swept over the extent of the solid along that axis. The shapes carry the solid ids, take the paint fields of the shape `paint` of each view (or, once it is replaced, of the first shape of `replace`) and replace in `views` (default: the three) the shapes of `replace` (default: `paint` and every solid id) on the bone. The three views then draw the same bodies, so they agree in every pose, and a second run changes nothing. `"grow": { <solid id>: metres }` lets the prism of a solid stand that far beyond the solid on every side: two parts that the volume holds a few millimetres apart (a thumb beside a finger) and that no sheet can print as two (the gap rule of the print profile) are drawn as touching. `"slices": { <solid id>: n }` draws a solid whose section changes along the bone (a trunk that leans) as n prisms along `u`, the shapes `<solid id>-1` to `-n`, each the convex outline of the section over its stretch. An entry with `"kind": "revolved"` draws instead ONE body of revolution about the bone axis for the solids named (a sleeve, an upper arm: lofts of nearly round section along the bone) as the shape `as` (default `paint`): its radius at fourteen stations along the bone is the mean of the half extents of the solids along `v` and along `w` there (`"radius": "max"` takes the larger, `"v"` or `"w"` that one). `"paintBone"` names the bone of the shape `paint` where the part had no shape of its own on `bone` (a finger bone whose finger the hand outline drew): the bodies are then drawn on `bone` right after that shape. A solid with skin weights is refused by name. It runs after `fit.ts` (land.sh) and no op of `fit.ts` may name a shape it draws.
 * @see ./fit.ts — the repair by ops that runs before it
 * @see ./where.ts — where the silhouette guard measures the redrawn part */
import { join } from "node:path";
import { S, readDoc, writeDoc } from "../MR/lib.ts";

const { copyMesh } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));

type P2 = [number, number];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const read = readDoc(args[0]!);
const asset = read.doc.asset;
const entries = JSON.parse(await Bun.file(flag("ops")!).text()) as any[];
const rest = restFrames(asset.rig) as Record<string, { frame: { o: number[] } }>;
const QUALITY = { chordM: 0.0005, minSegments: 16, maxSegments: 64 };
const GEOMETRY_KEYS = new Set(["kind", "id", "points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "cx", "cy", "rx", "ry", "d", "followTip", "solid"]);
const AXES = { u: [0, 1, 2], v: [1, 0, 2], w: [2, 0, 1] } as const;
const round = (value: number): number => Math.round(value * 1e5) / 1e5;

/** 🐚️ The convex hull of points, counter-clockwise. */
function hull(points: P2[]): P2[] {
  const sorted = [...points].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cross = (o: P2, a: P2, b: P2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const half = (list: P2[]): P2[] => {
    const out: P2[] = [];
    for (const p of list) {
      while (out.length >= 2 && cross(out[out.length - 2]!, out[out.length - 1]!, p) <= 0) out.pop();
      out.push(p);
    }
    out.pop();
    return out;
  };
  return [...half(sorted), ...half(sorted.reverse())];
}

/** ✂️ A convex ring thinned to corners that stand at least `tolerance` off the chord of their neighbours, at most `most` points. */
function thin(ring: P2[], most: number): P2[] {
  let tolerance = 0.0003;
  let out = ring;
  for (;;) {
    out = [...ring];
    for (let changed = true; changed && out.length > 4; ) {
      changed = false;
      for (let i = 0; i < out.length && out.length > 4; i++) {
        const [a, b, c] = [out[(i + out.length - 1) % out.length]!, out[i]!, out[(i + 1) % out.length]!];
        const length = Math.hypot(c[0] - a[0], c[1] - a[1]) || 1;
        if (Math.abs((c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])) / length < tolerance) {
          out.splice(i, 1);
          changed = true;
        }
      }
    }
    if (out.length <= most) return out;
    tolerance *= 1.5;
  }
}

const area = (ring: P2[]): number => Math.abs(ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0)) / 2;
let changed = 0;
let refused = 0;
for (const entry of entries) {
  const scale = String(entry.scale);
  const volume = asset.volumes?.[scale];
  const views: string[] = entry.views ?? ["elevation", "front", "plan"];
  const solids: any[] = (volume?.body ?? []).filter((solid: any) => solid.bone === entry.bone && (entry.solids === undefined || entry.solids.includes(solid.id)));
  if (solids.length === 0) {
    console.log(`${asset.id} 1:${scale} ${entry.bone}: REFUSED, the band has no body solid on the bone`);
    refused++;
    continue;
  }
  const weighted = solids.find((solid) => solid.weights?.length || solid.repeat !== undefined);
  if (weighted !== undefined) {
    console.log(`${asset.id} 1:${scale} ${entry.bone}: REFUSED, solid "${weighted.id}" has skin weights or a repeat`);
    refused++;
    continue;
  }
  const bodies = solids.flatMap((solid) => {
    const { positions } = copyMesh(solid, QUALITY) as { positions: Float64Array };
    const slices: number = entry.slices?.[solid.id] ?? 1;
    if (slices > 1) {
      const us: number[] = [];
      for (let i = 0; i < positions.length; i += 3) us.push(positions[i]!);
      const [low, high] = [Math.min(...us), Math.max(...us)];
      const step = (high - low) / slices;
      return Array.from({ length: slices }, (_, k) => {
        const [from, to] = [low + k * step, low + (k + 1) * step];
        const flat: P2[] = [];
        for (let i = 0; i < positions.length; i += 3) if (positions[i]! >= from - step / 4 && positions[i]! <= to + step / 4) flat.push([positions[i + 1]!, positions[i + 2]!]);
        return { id: `${solid.id}-${k + 1}`, geometry: { kind: "poly", closed: true, points: thin(hull(flat), 20).map(([x, y]) => [round(x), round(y)]), solid: { kind: "extruded", sweep: "u", z0: round(from), z1: round(to) } } };
      });
    }
    const choices = (["u", "v", "w"] as const).map((sweep) => {
      const [k, a, b] = AXES[sweep];
      const along: number[] = [];
      const flat: P2[] = [];
      for (let i = 0; i < positions.length; i += 3) {
        along.push(positions[i + k]!);
        flat.push([positions[i + a]!, positions[i + b]!]);
      }
      const ring = thin(hull(flat), 20);
      const [low, high] = [Math.min(...along), Math.max(...along)];
      return { sweep, ring, low, high, volume: area(ring) * (high - low) };
    });
    const choice = entry.sweep?.[solid.id] !== undefined ? choices.find((candidate) => candidate.sweep === entry.sweep[solid.id])! : choices.reduce((best, candidate) => (candidate.volume < best.volume ? candidate : best));
    const base = choice.sweep === "w" ? rest[entry.bone]!.frame.o[2]! : 0;
    const grow: number = entry.grow?.[solid.id] ?? 0;
    const centre: P2 = [choice.ring.reduce((sum, p) => sum + p[0], 0) / choice.ring.length, choice.ring.reduce((sum, p) => sum + p[1], 0) / choice.ring.length];
    const grown = choice.ring.map(([x, y]): P2 => {
      const length = Math.hypot(x - centre[0], y - centre[1]) || 1;
      return [x + (grow * (x - centre[0])) / length, y + (grow * (y - centre[1])) / length];
    });
    return [{ id: solid.id as string, geometry: { kind: "poly", closed: true, points: grown.map(([x, y]) => [round(x), round(y)]), solid: { kind: "extruded", sweep: choice.sweep, z0: round(choice.low - grow + base), z1: round(choice.high + grow + base) } } }];
  });
  if (entry.kind === "revolved") {
    const points: [number, number, number][] = [];
    for (const solid of solids) {
      const { positions } = copyMesh(solid, QUALITY) as { positions: Float64Array };
      for (let i = 0; i < positions.length; i += 3) points.push([positions[i]!, positions[i + 1]!, positions[i + 2]!]);
    }
    const [low, high] = [Math.min(...points.map((p) => p[0])), Math.max(...points.map((p) => p[0]))];
    const STATIONS = 14;
    const profile: P2[] = [];
    for (let k = 0; k < STATIONS; k++) {
      const at = low + ((high - low) * k) / (STATIONS - 1);
      const near = points.filter((p) => Math.abs(p[0] - at) <= (high - low) / (STATIONS - 1) / 2);
      if (near.length === 0) continue;
      const [rv, rw] = [Math.max(...near.map((p) => Math.abs(p[1]))), Math.max(...near.map((p) => Math.abs(p[2])))];
      profile.push([round(at), round(entry.radius === "max" ? Math.max(rv, rw) : entry.radius === "v" ? rv : entry.radius === "w" ? rw : (rv + rw) / 2)]);
    }
    bodies.splice(0, bodies.length, { id: (entry.as ?? entry.paint) as string, geometry: { kind: "poly", closed: true, points: [[round(low), 0], ...profile, [round(high), 0]], solid: { kind: "revolved", axisBone: entry.bone, axis: "u", axisX: 0, mirror: true } as any } });
  }
  const replace = new Set<string>(entry.replace ?? (entry.kind === "revolved" ? [entry.as ?? entry.paint] : [entry.paint, ...solids.map((solid) => solid.id)]));
  for (const view of views) {
    const shapes: any[] = asset.representations[scale].geometry[view];
    const paintShape = shapes.find((shape) => shape.id === entry.paint && shape.bone === (entry.paintBone ?? entry.bone)) ?? shapes.find((shape) => shape.bone === entry.bone && replace.has(shape.id));
    if (paintShape === undefined) {
      console.log(`${asset.id} 1:${scale} ${view} ${entry.bone}: REFUSED, no shape "${entry.paint}" to take the paint from`);
      refused++;
      continue;
    }
    const paint = Object.fromEntries(Object.entries(paintShape).filter(([key]) => !GEOMETRY_KEYS.has(key)));
    const before = JSON.stringify(shapes.filter((shape) => shape.bone === entry.bone && replace.has(shape.id)));
    const first = shapes.findIndex((shape) => shape.bone === entry.bone && replace.has(shape.id));
    const at = first >= 0 ? first : shapes.indexOf(paintShape) + 1;
    const drawn = bodies.map((body) => ({ kind: body.geometry.kind, closed: true, id: body.id, ...paint, bone: entry.bone, points: body.geometry.points, solid: body.geometry.solid }));
    const kept = shapes.filter((shape) => !(shape.bone === entry.bone && replace.has(shape.id)));
    kept.splice(Math.min(at, kept.length), 0, ...drawn);
    const same = JSON.stringify(drawn) === before;
    if (!same) {
      shapes.splice(0, shapes.length, ...kept);
      changed++;
    }
    console.log(`${asset.id} 1:${scale} ${view} ${entry.bone}: ${same ? "unchanged" : "drawn"}, ${drawn.length} bodies (${bodies.map((body) => `${body.id}:${body.geometry.solid.sweep}`).join(" ")})`);
  }
}
const out = flag("out");
if (args.includes("--write") || out !== undefined) console.log(writeDoc(read, out) ? `written: ${out ?? read.file}` : "file unchanged");
console.log(`${asset.id}: ${entries.length} entries, ${changed} changed something, ${refused} refused`);
if (refused > 0) process.exit(1);
