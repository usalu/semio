/** 🎐️ Gives a hanging part its height in the plan (or its depth in the front), a step of the chain of a plant before `carry.ts`: `bun prism.ts <id|file.json> [--out file] [--write]`, configured by `prism/<id>.json` (`view`: `plan` or `front`; `bones`: the bones of the parts; `scales`: optional list of representation keys).
 *
 * A curtain of a weeping tree hangs from its limb and swings about its top. Its side elevation turns as a whole, so its lower end travels far; its flat plan outline is one sheet that the engine carries at the height of the middle of the part, so in the wind the plan reaches only half as far as the elevation (cause D of `multiview-diagnosis.md`, here the whole-figure `sync-forward`). Depth plates would repeat the long outline of a curtain many times; the file of such a tree must not grow. Instead every flat filled shape the view draws on a named bone becomes what the part is: an `extruded` solid along the bone axis nearest to the viewing direction, from the upper to the lower end of the side elevation of the part (the elevation twin of the same id, else every elevation shape of the bone), whose profile is the drawn outline itself. Seen in the rest pose the prism is its profile, so the drawing does not change and no point is added; in a turned pose the bone carries the whole height, and the view shows the footprint of the swung curtain.
 * The profile coordinates are solved from the engine (probes with the same solid, as `🧰️silhouette/R4-P/redraw.ts` does). A shape whose path is not made of straight segments is refused by name. The run is idempotent: a shape that is a solid already is left as it is.
 * @see ../MR/gate.ts — the measurement of the matrix
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));

type P = [number, number];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const config = JSON.parse(readFileSync(join(dirname(Bun.main), "prism", `${asset.id}.json`), "utf8"));
const view: "plan" | "front" = config.view ?? "plan";
const pose = { kind: "pose", pose: asset.presentation.defaultPose } as const;
const round = (value: number): number => Number(value.toFixed(4));
let made = 0, refused = 0, worst = 0;
for (const key of Object.keys(asset.representations)) {
  if (engine.authoredOf(asset, key) === undefined || (config.scales !== undefined && !config.scales.map(String).includes(key))) continue;
  const rep = asset.representations[key];
  const list = rep.geometry[view] as any[];
  const wanted = list.filter((shape) => config.bones.includes(shape.bone) && shape.solid === undefined && shape.filled === true && shape.role !== "ground" && (shape.kind === "poly" ? shape.closed === true : shape.kind === "path"));
  if (wanted.length === 0) continue;
  const state: any = evaluate(asset, pose, engine.scaleSelector(Number(key)));
  const towards = (a: { x: number; y: number; z: number }): number => Math.abs(view === "front" ? a.x : a.y);
  const jobs: { shape: any; solid: any; target: P[][] }[] = [];
  for (const shape of wanted) {
    if (shape.kind === "path" && /[^MLZ\s\d.eE+-]/u.test(shape.d)) { console.log(`REFUSED ${asset.id} 1:${key} ${view} ${shape.id} on ${shape.bone}: its path is not made of straight segments`); refused++; continue; }
    const f = state.bones[shape.bone].frame as Record<"o" | "u" | "v" | "w", { x: number; y: number; z: number }>;
    const sweep = (["w", "v", "u"] as const).reduce((best, k) => (towards(f[k]) > towards(f[best]) + 1e-9 ? k : best), "w" as "u" | "v" | "w");
    const elevation = (state.shapes.elevation as any[]).filter((candidate) => candidate.shape.bone === shape.bone && candidate.shape.role !== "ground");
    const twins = elevation.filter((candidate) => candidate.shape.id === shape.id);
    const points = (twins.length > 0 ? twins : elevation).flatMap((candidate) => candidate.rings.flat());
    if (points.length === 0 || sweep === "w") { console.log(`REFUSED ${asset.id} 1:${key} ${view} ${shape.id} on ${shape.bone}: ${points.length === 0 ? "the side elevation does not draw the bone" : "the bone lies across the view"}`); refused++; continue; }
    const along = points.map((p: any) => (p.x - f.o.x) * f[sweep].x + (p.y - f.o.y) * f[sweep].y);
    const evaluated = (state.shapes[view] as any[]).find((candidate) => candidate.shape === shape || (candidate.shape.id === shape.id && candidate.shape.bone === shape.bone));
    jobs.push({ shape, solid: { kind: "extruded", sweep, z0: round(Math.min(...along)), z1: round(Math.max(...along)) }, target: evaluated.rings.map((ring: any[]) => ring.map((p) => [p.x, p.y] as P)) });
  }
  const probes = jobs.flatMap((job, i) => ([[0, 0], [1, 0], [0, 1]] as P[]).map((at, n) => ({ id: `probe-${i}-${n}`, bone: job.shape.bone, role: "body", z: 0, filled: true, tone: 0.5, kind: "poly", points: [[at[0] - 1e-4, at[1] - 1e-4], [at[0] + 1e-4, at[1] - 1e-4], [at[0] + 1e-4, at[1] + 1e-4], [at[0] - 1e-4, at[1] + 1e-4]], closed: true, solid: job.solid })));
  rep.geometry[view] = probes;
  const probed: any = evaluate(asset, pose, engine.scaleSelector(Number(key)));
  rep.geometry[view] = list;
  const image = new Map<string, P>();
  for (const candidate of probed.shapes[view]) { const all = (candidate.rings as { x: number; y: number }[][]).flat(); image.set(candidate.shape.id, [(Math.min(...all.map((p) => p.x)) + Math.max(...all.map((p) => p.x))) / 2, (Math.min(...all.map((p) => p.y)) + Math.max(...all.map((p) => p.y))) / 2]); }
  jobs.forEach((job, i) => {
    const [o, x, y] = [0, 1, 2].map((n) => image.get(`probe-${i}-${n}`)!) as [P, P, P];
    const [a, b, c, d] = [x[0] - o[0], y[0] - o[0], x[1] - o[1], y[1] - o[1]];
    const det = a * d - b * c;
    if (Math.abs(det) < 1e-9) { console.log(`REFUSED ${asset.id} 1:${key} ${view} ${job.shape.id} on ${job.shape.bone}: the prism is seen edge-on`); refused++; return; }
    const local = job.target.map((ring) => { const closed = ring.length > 1 && Math.hypot(ring[0]![0] - ring[ring.length - 1]![0], ring[0]![1] - ring[ring.length - 1]![1]) < 1e-9 ? ring.slice(0, -1) : ring; return closed.map(([px, py]) => [round((d * (px - o[0]) - b * (py - o[1])) / det), round((-c * (px - o[0]) + a * (py - o[1])) / det)] as P); });
    delete job.shape.d; delete job.shape.points; delete job.shape.closed;
    if (local.length === 1) Object.assign(job.shape, { kind: "poly", points: local[0], closed: true });
    else Object.assign(job.shape, { kind: "path", d: local.map((ring) => `M ${ring.map((p) => `${p[0]} ${p[1]}`).join(" L ")} Z`).join(" ") });
    job.shape.solid = job.solid;
    made++;
  });
  const after: any = evaluate(asset, pose, engine.scaleSelector(Number(key)));
  for (const job of jobs) {
    if (job.shape.solid === undefined) continue;
    const drawn = (after.shapes[view] as any[]).find((candidate) => candidate.shape.id === job.shape.id && candidate.shape.bone === job.shape.bone);
    const box = (rings: { x: number; y: number }[][] | P[][]): number[] => { const all = (rings as any[][]).flat().map((p: any) => (Array.isArray(p) ? p : [p.x, p.y])); return [Math.min(...all.map((p) => p[0])), Math.max(...all.map((p) => p[0])), Math.min(...all.map((p) => p[1])), Math.max(...all.map((p) => p[1]))]; };
    const [now, was] = [box(drawn.rings), box(job.target)];
    worst = Math.max(worst, ...now.map((value, k) => Math.abs(value - was[k]!)));
  }
  console.log(`${asset.id} 1:${key} ${view}: ${jobs.filter((job) => job.shape.solid !== undefined).length} shapes are prisms now (${[...new Set(jobs.map((job) => job.shape.bone))].join(", ")})`);
}
const written = rewrite(text, previous, next);
console.log(`${asset.id}: ${made} prisms, ${refused} refused, the drawn extents in the default pose move by at most ${(worst * 1000).toFixed(1)} mm; ${written === text ? "unchanged" : `changed (${text.length} -> ${written.length} characters)`}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || (out !== undefined && !existsSync(out))) writeFileSync(out ?? file, written);
}
