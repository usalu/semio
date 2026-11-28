/** 🍃️ Authors soft botanical masses as faceted lobes: `bun facet.ts <id|file.json> --ops <ops.json> --out <file.json>` writes a trial copy in the indentation of the source, the vertex and triangle lists of the lobes on one line each (a source that already holds polyhedra over many lines keeps that form). An op is `{ "facet": "<regular expression over the solid id>", "list": "body" | "thermal" | "structure", "kinds": ["ellipsoid", …], "scales": [50, …] }` (kinds default `ellipsoid`, also `capsule`, `revolve` and a capped `loft`, scales default all four; `"segments": n` states the facets around every ellipsoid lobe of the op instead, for a tree whose lobes are too many for the budget at the reference facets, and scales each lobe so that its outline straddles the reference outline; `"below": r` keeps such an op to the ellipsoids of at most that largest radius in metres, where the straddling lobe leaves the reference outline by at most 0.54 % of the radius; `"smooth": true` sizes a lobe of the reference facets so that its outline straddles the smooth ellipsoid instead of lying inside it (half the sagitta outwards, for a view whose drawing was fitted to the smooth outline with no room left); `"inscribed": true` sizes a lobe of `segments` facets so that it lies inside its own reference facets instead of straddling them, for a lobe that no view of the guard shows on the outline of the tree (`crown.ts`); `an op `{ "keep": { "<solid id>": [indices…] }, "list": …, "scales": […] }` (written by `crown.ts`) makes every named ellipsoid the convex polyhedron of the chosen corner points of its own reference facets, and every named convex polyhedron that of the chosen ones of its own corner points, which shows in a view exactly the outline of the reference lobe when the corner points of that outline are among them and lies inside the reference lobe everywhere else; an op `{ "thin": { "<solid id>": { "keep": [indices…], "triangles": […] } }, "list": …, "scales": […] }` (written by `thin.ts`) gives a polyhedron the named ones of its own corner points and those facets over them; `"above": r` leaves an ellipsoid, capsule or loft whose smallest radius is below r as it is, because a faceted sphere of exactly the smallest feature of its scale measures 0.9 % thinner across its flats than the feature): every matching real solid of that list becomes a `polyhedron` with the facets the kernel gives the solid at the reference quality of the asset (`qualityOf(asset)` without a scale, the quality the silhouette, area and volume validators measure at: 24 to 64 segments on a circle), welded, in the same bone frame with the same id, bone, material, role, tone, transform and binding, so every validator measures the surface it measured before and the scene of every scale holds these facets instead of the 32 to 64 segments of its own quality. An op `{ "remove": "<regular expression>", "list": …, "scales": […] }` takes the matching solids out of a list (parts that are merged into their neighbours at that scale), `{ "lens": "<regular expression>", "points": 5, … }` turns an ellipsoid of revolution about u into a `revolve` with that many profile points on its meridian (inscribed, so it stays inside the ellipsoid), `{ "decision": { "en": …, "de": … }, "scales": […], "instead": "<start of the English text>" }` appends that decision to the scales (once) or, with `instead`, puts it in the place of the decision that starts so, `{ "set": "<path>", "value": …, "was": … }` is the op of `../CL-H/fix.ts` and `{ "unset": "<path>", "was": … }` takes a property away. The tool refuses a lobe that is not a closed outward shell.
 * @see ../MR/lib.ts
 * @see ../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🪨️polyhedron/🟦️.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { readDoc } from "../MR/lib.ts";
import { hull3 } from "./hull.ts";

type Op = { thin?: Record<string, { keep: number[]; triangles: number[][] }>; keep?: Record<string, number[]>; inscribed?: boolean; facet?: string; remove?: string; lens?: string; points?: number; list?: "body" | "thermal" | "structure"; kinds?: string[]; scales?: number[]; segments?: number; below?: number; above?: number; smooth?: boolean; decision?: { en: string; de: string }; instead?: string; set?: string; unset?: string; value?: unknown; was?: unknown };
const SHAPE: Record<string, string[]> = { ellipsoid: ["radii"], capsule: ["p0", "p1", "r0", "r1", "lateralScale"], revolve: ["profile", "axis", "origin", "sweepRad", "lateralScale"], loft: ["sections", "capStart", "capEnd"] };
const { values, positionals } = parseArgs({ options: { ops: { type: "string" }, out: { type: "string" } }, allowPositionals: true });
const read = readDoc(positionals[0]!);
const asset = read.doc.asset;
const quality = qualityOf(asset);
const round = (n: number): number => Number(n.toFixed(5)) + 0;

/** ⚖️ The factor that lets a lobe of `segments` facets around straddle the outline the reference quality gives its ellipsoid of largest radius `r`: the mean radius of the polygon of `segments` sides equals the mean radius of the reference polygon, so the lobe leaves the measured outline by at most about half the difference of the two sagittas, inwards and outwards alike. */
function straddle(r: number, segments: number): number {
  const ratio = quality.chordM / r;
  const reference = Math.max(quality.minSegments, Math.min(quality.maxSegments, ratio >= 1 ? quality.minSegments : Math.ceil(Math.PI / Math.acos(1 - ratio))));
  return (1 + Math.cos(Math.PI / reference)) / (1 + Math.cos(Math.PI / segments));
}

/** 🫧️ The largest factor of at most 1 about the origin of the solid that keeps every vertex of coarse facets inside the convex reference facets of the same solid (less 0.02 % for the rounding of the coordinates), so that a lobe authored with fewer facets never shows beyond the reference lobe in any view. */
function within(points: ArrayLike<number>, reference: { positions: ArrayLike<number>; indices: ArrayLike<number> }): number {
  let fit = 1;
  const p = reference.positions;
  for (let t = 0; t < reference.indices.length; t += 3) {
    const [a, b, c] = [reference.indices[t]! * 3, reference.indices[t + 1]! * 3, reference.indices[t + 2]! * 3];
    const [ux, uy, uz, vx, vy, vz] = [p[b]! - p[a]!, p[b + 1]! - p[a + 1]!, p[b + 2]! - p[a + 2]!, p[c]! - p[a]!, p[c + 1]! - p[a + 1]!, p[c + 2]! - p[a + 2]!];
    const [nx, ny, nz] = [uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx];
    const d = nx * p[a]! + ny * p[a + 1]! + nz * p[a + 2]!;
    if (!(d > 0)) continue;
    for (let i = 0; i < points.length; i += 3) {
      const h = nx * points[i]! + ny * points[i + 1]! + nz * points[i + 2]!;
      if (h > d * fit) fit = d / h;
    }
  }
  return fit * 0.9998;
}

/** 🪡️ The smallest radius of a round solid in metres (ellipsoid, capsule, loft of elliptic sections; any other solid counts as thick), which `above` holds against the smallest feature: a solid of just that feature stays smooth. */
function slimmest(solid: Record<string, unknown>): number {
  if (solid.kind === "ellipsoid") return Math.min(...(solid.radii as number[]));
  if (solid.kind === "capsule") return Math.min(solid.r0 as number, solid.r1 as number) * Math.min(1, (solid.lateralScale as number | undefined) ?? 1);
  if (solid.kind === "loft") return Math.min(...(solid.sections as { rx?: number; ry?: number }[]).map((section) => Math.min(section.rx ?? Infinity, section.ry ?? Infinity)));
  return Infinity;
}

/** 🔺️ The welded facets of one solid in its own frame, checked to be a closed outward shell. */
function lobe(solid: Record<string, unknown>, segments?: number, smooth?: boolean, inscribed?: boolean): { vertices: number[][]; triangles: number[][] } {
  const shape = Object.fromEntries(SHAPE[solid.kind as string]!.filter((key) => solid[key] !== undefined).map((key) => [key, solid[key]]));
  const mesh = tessellateSolid({ id: "lobe", node: "n", material: "m", kind: solid.kind, ...shape } as never, segments === undefined ? quality : { chordM: quality.chordM, minSegments: segments, maxSegments: segments });
  const fit = inscribed === true ? within(mesh.positions, tessellateSolid({ id: "lobe", node: "n", material: "m", kind: solid.kind, ...shape } as never, quality)) : solid.kind !== "ellipsoid" ? 1 : segments !== undefined ? straddle(Math.max(...(solid.radii as number[])), segments) : smooth === true ? 1 / straddle(Math.max(...(solid.radii as number[])), 1e9) : 1;
  const index = new Map<string, number>();
  const vertices: number[][] = [];
  const remap: number[] = [];
  for (let i = 0; i < mesh.positions.length / 3; i++) {
    const p = [round(mesh.positions[i * 3]! * fit), round(mesh.positions[i * 3 + 1]! * fit), round(mesh.positions[i * 3 + 2]! * fit)];
    const key = p.join(",");
    if (!index.has(key)) { index.set(key, vertices.length); vertices.push(p); }
    remap.push(index.get(key)!);
  }
  const triangles: number[][] = [];
  for (let t = 0; t < mesh.indices.length; t += 3) {
    const [a, b, c] = [remap[mesh.indices[t]!]!, remap[mesh.indices[t + 1]!]!, remap[mesh.indices[t + 2]!]!];
    if (a !== b && b !== c && c !== a) triangles.push([a, b, c]);
  }
  const edges = new Map<number, number>();
  let volume = 0;
  for (const [a, b, c] of triangles) {
    for (const [from, to] of [[a, b], [b, c], [c, a]] as const) edges.set(from * vertices.length + to, (edges.get(from * vertices.length + to) ?? 0) + 1);
    const [p, q, r] = [vertices[a!]!, vertices[b!]!, vertices[c!]!];
    volume += (p[0]! * (q[1]! * r[2]! - q[2]! * r[1]!) - p[1]! * (q[0]! * r[2]! - q[2]! * r[0]!) + p[2]! * (q[0]! * r[1]! - q[1]! * r[0]!)) / 6;
  }
  for (const [edge, count] of edges) if (count !== 1 || !edges.has((edge % vertices.length) * vertices.length + Math.floor(edge / vertices.length))) throw new Error(`${solid.id}: the facets are not a closed shell`);
  if (!(volume > 0)) throw new Error(`${solid.id}: the facets enclose ${volume} m3`);
  return { vertices, triangles };
}

const steps = (path: string): string[] => path.split(".").flatMap((part) => { const m = /^([^\[]+)\[(.+)\]$/.exec(part); return m ? [m[1]!, `#${m[2]}`] : [part]; });
const step = (node: any, key: string, path: string): any => {
  const next = key.startsWith("#") ? (node as { id: string }[]).find((entry) => entry.id === key.slice(1)) : node[key];
  if (next === undefined) throw new Error(`${path}: nothing at "${key}"`);
  return next;
};
const report: string[] = [];
for (const op of JSON.parse(readFileSync(values.ops!, "utf8")) as Op[]) {
  const scales = op.scales ?? [50, 100, 200, 500];
  if (op.facet !== undefined) {
    const pattern = new RegExp(`^(?:${op.facet})$`, "u");
    const kinds = op.kinds ?? ["ellipsoid"];
    for (const scale of scales) {
      const list = asset.volumes[`${scale}`][op.list ?? "body"] as Record<string, unknown>[];
      let made = 0;
      let triangles = 0;
      list.forEach((solid, at) => {
        if (!pattern.test(solid.id as string) || !kinds.includes(solid.kind as string)) return;
        if (op.below !== undefined && !(solid.kind === "ellipsoid" && Math.max(...(solid.radii as number[])) <= op.below)) return;
        if (op.above !== undefined && !(slimmest(solid) >= op.above)) return;
        const { vertices, triangles: faces } = lobe(solid, op.segments, op.smooth, op.inscribed);
        const next: Record<string, unknown> = {};
        for (const [key, value] of Object.entries(solid)) {
          if (key === "kind") Object.assign(next, { kind: "polyhedron", vertices, triangles: faces });
          else if (!SHAPE[solid.kind as string]!.includes(key)) next[key] = value;
        }
        list[at] = next;
        made += 1;
        triangles += faces.length;
      });
      report.push(`1:${scale} ${op.list ?? "body"} /${op.facet}/${op.segments === undefined ? "" : ` at ${op.segments} segments`}: ${made} lobes, ${triangles} triangles`);
    }
  } else if (op.keep !== undefined) {
    for (const scale of scales) {
      const list = asset.volumes[`${scale}`][op.list ?? "body"] as Record<string, unknown>[];
      let made = 0, total = 0;
      list.forEach((solid, at) => {
        const chosen = op.keep![solid.id as string];
        if (chosen === undefined || (solid.kind !== "ellipsoid" && solid.kind !== "polyhedron")) return;
        const reference = solid.kind === "polyhedron" ? { vertices: solid.vertices as number[][] } : lobe(solid);
        const points = [...new Set(chosen)].sort((p, q) => p - q).map((k) => reference.vertices[k]!);
        const raw = hull3(points);
        const order = [...new Set(raw.flat())].sort((p, q) => p - q);
        const index = new Map(order.map((k, i) => [k, i]));
        const vertices = order.map((k) => points[k]!);
        const faces = raw.map((t) => t.map((k) => index.get(k)!));
        const twins = new Set(faces.flatMap(([a, b, c]) => [`${a},${b}`, `${b},${c}`, `${c},${a}`]));
        if (twins.size !== faces.length * 3 || [...twins].some((e) => !twins.has(e.split(",").reverse().join(",")))) throw new Error(`${solid.id}: the kept corner points give no closed shell`);
        const next: Record<string, unknown> = {};
        for (const [key, value] of Object.entries(solid)) {
          if (key === "kind") Object.assign(next, { kind: "polyhedron", vertices, triangles: faces });
          else if (key !== "radii" && key !== "vertices" && key !== "triangles") next[key] = value;
        }
        list[at] = next;
        made += 1;
        total += faces.length;
      });
      report.push(`1:${scale} ${op.list ?? "body"} keep: ${made} lobes of chosen corner points, ${total} triangles`);
    }
  } else if (op.thin !== undefined) {
    for (const scale of scales) {
      const list = asset.volumes[`${scale}`][op.list ?? "body"] as Record<string, unknown>[];
      let made = 0, total = 0, from = 0;
      list.forEach((solid, at) => {
        const chosen = op.thin![solid.id as string];
        if (chosen === undefined || solid.kind !== "polyhedron") return;
        const vertices = chosen.keep.map((k) => (solid.vertices as number[][])[k]!);
        const twins = new Set(chosen.triangles.flatMap(([a, b, c]) => [`${a},${b}`, `${b},${c}`, `${c},${a}`]));
        if (twins.size !== chosen.triangles.length * 3 || [...twins].some((e) => !twins.has(e.split(",").reverse().join(",")))) throw new Error(`${solid.id}: the thinned facets are no closed shell`);
        from += (solid.triangles as number[][]).length;
        list[at] = { ...solid, vertices, triangles: chosen.triangles };
        made += 1;
        total += chosen.triangles.length;
      });
      report.push(`1:${scale} ${op.list ?? "body"} thin: ${made} polyhedra, ${from} -> ${total} triangles`);
    }
  } else if (op.remove !== undefined) {
    const pattern = new RegExp(`^(?:${op.remove})$`, "u");
    for (const scale of scales) {
      const volume = asset.volumes[`${scale}`];
      const list = volume[op.list ?? "body"] as { id: string }[];
      const gone = list.filter((solid) => pattern.test(solid.id)).map((solid) => solid.id);
      volume[op.list ?? "body"] = list.filter((solid) => !pattern.test(solid.id));
      report.push(`1:${scale} ${op.list ?? "body"} /${op.remove}/: ${gone.length} solids removed (${gone.join(", ")})`);
    }
  } else if (op.lens !== undefined) {
    const pattern = new RegExp(`^(?:${op.lens})$`, "u");
    const points = op.points ?? 5;
    for (const scale of scales) {
      const list = asset.volumes[`${scale}`][op.list ?? "body"] as Record<string, unknown>[];
      let made = 0;
      list.forEach((solid, at) => {
        if (!pattern.test(solid.id as string) || solid.kind !== "ellipsoid") return;
        const [a, b, c] = solid.radii as number[];
        if (b !== c) throw new Error(`${solid.id}: radii ${a}, ${b}, ${c} are no lens about u`);
        const profile = Array.from({ length: points }, (_, k) => [round(-a! * Math.cos((Math.PI * k) / (points - 1))), k === 0 || k === points - 1 ? 0 : round(b! * Math.sin((Math.PI * k) / (points - 1)))]);
        const next: Record<string, unknown> = {};
        for (const [key, value] of Object.entries(solid)) {
          if (key === "kind") Object.assign(next, { kind: "revolve", profile, axis: [1, 0, 0], origin: [0, 0, 0] });
          else if (key !== "radii") next[key] = value;
        }
        list[at] = next;
        made += 1;
      });
      report.push(`1:${scale} ${op.list ?? "body"} /${op.lens}/: ${made} lenses of ${points} profile points`);
    }
  } else if (op.decision !== undefined) {
    for (const scale of scales) {
      const decisions = asset.volumes[`${scale}`].decisions as { en: string; de: string }[];
      if (decisions.some((d) => d.en === op.decision!.en)) continue;
      const at = op.instead === undefined ? -1 : decisions.findIndex((d) => d.en.startsWith(op.instead!));
      if (op.instead !== undefined && at < 0) throw new Error(`1:${scale}: no decision starts with "${op.instead}"`);
      if (at < 0) decisions.push(op.decision);
      else decisions[at] = op.decision;
      report.push(`1:${scale} decision ${at < 0 ? "added" : `${at} rewritten`}`);
    }
  } else if (op.unset !== undefined) {
    const keys = steps(op.unset), last = keys.pop()!, holder = keys.reduce((node, key) => step(node, key, op.unset!), asset);
    if (!(last in holder)) continue;
    if (JSON.stringify(holder[last]) !== JSON.stringify(op.was)) throw new Error(`${op.unset}: is ${JSON.stringify(holder[last])}, expected ${JSON.stringify(op.was)}`);
    delete holder[last];
    report.push(`unset ${op.unset}`);
  } else if (op.set !== undefined) {
    const keys = steps(op.set), last = keys.pop()!, holder = keys.reduce((node, key) => step(node, key, op.set!), asset);
    if (JSON.stringify(holder[last]) === JSON.stringify(op.value)) continue;
    if (JSON.stringify(holder[last]) !== JSON.stringify(op.was)) throw new Error(`${op.set}: is ${JSON.stringify(holder[last])}, expected ${JSON.stringify(op.was)}`);
    holder[last] = op.value;
    report.push(`set ${op.set}`);
  }
}
const second = read.text.split("\n", 2)[1] ?? "";
const inline: string[] = [];
const keep = (value: unknown): string => `@@${inline.push(JSON.stringify(value)) - 1}@@`;
if (!/"vertices": \[\n/u.test(read.text)) for (const volume of Object.values(asset.volumes) as Record<string, unknown>[]) for (const list of ["body", "thermal", "structure"]) for (const solid of volume[list] as Record<string, unknown>[]) if (solid.kind === "polyhedron") Object.assign(solid, { vertices: keep(solid.vertices), triangles: keep(solid.triangles) });
writeFileSync(values.out!, JSON.stringify(read.doc, null, second.length - second.trimStart().length).replace(/"@@(\d+)@@"/gu, (_, at) => inline[Number(at)]!) + (read.text.endsWith("\n") ? "\n" : ""));
console.log(`${asset.id}: reference quality ${JSON.stringify(quality)}\n${report.join("\n")}\n-> ${values.out}`);
