/** 🎞️ Ticket tool (work package O2): plays species documents on the real stage (`openStage`/`advance`/`frameOf`), forces every activity with a one-species cast and writes the frames, drawn by the React target's `depictionMarkup` with the product's own CSS, as an HTML sheet of frame bursts per activity.
 *
 * Usage (from the repository root):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_o2_stage_sheet.ts" --out <directory> [--scale 2] [--only idle,walk,…] <species.json>...
 * then `node …/wp_o2_shoot.mjs <directory>` screenshots every section of every sheet and measures how far the drawing reaches beyond the box of its species.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { MENAGERIE_SCHEMA, type Actor, type ActorFrame, type Clip, type Menagerie, type Species, type Stage, type StageEvent } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { pathToFileURL } from "node:url";

const ENGINE = process.env.WP_O2_ENGINE ? resolve(process.env.WP_O2_ENGINE) : join(import.meta.dir, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const engine = (path: string): string => pathToFileURL(join(ENGINE, path)).href;
const { clipTicks } = (await import(engine("🔨️modules/🎞️animation/🟦️.ts"))) as typeof import("../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎞️animation/🟦️.ts");
const { advance, frameOf, openStage } = (await import(engine("🔨️modules/🎪️stage/🟦️.ts"))) as typeof import("../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts");
const { depictionMarkup } = (await import(engine("🎯️targets/⚛️react/🔨️modules/🖌️depiction/🟦️.ts"))) as typeof import("../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🖌️depiction/🟦️.ts");

const args = process.argv.slice(2);
const option = (name: string, fallback: string): string => {
  const index = args.indexOf(`--${name}`);
  if (index < 0) return fallback;
  const [, value] = args.splice(index, 2);
  return value ?? fallback;
};
const out = resolve(option("out", "wp-o2"));
const scale = Number(option("scale", "2"));
const only = option("only", "");
const files = args;
if (files.length === 0) throw new Error("no species files given");

const CSS = readFileSync(join(ENGINE, "🎯️targets/⚛️react/🎨️.css"), "utf8");
const WIDTH = 900;
const FLOOR = 300;
const HOME = 450;
const THEMES = [
  { name: "light", base: "#f7f3e3", panel: "#c9c8bd", ink: "#001117", border: "#7b827d" },
  { name: "dark", base: "#001117", panel: "#1d2b2f", ink: "#f7f3e3", border: "#7b827d" },
] as const;

type Shot = { readonly label: string; readonly frame: ActorFrame; readonly dx: number; readonly dy: number };
type Row = { readonly title: string; readonly shots: readonly Shot[]; readonly reach: number; readonly lead: number; readonly drop: number; readonly zoom: number; readonly marks: boolean };

/** 🧪️ A stage on which exactly one actor of `kind` stands at home, facing right, whole, with resting pupils and no blink in sight. */
function stageOf(menagerie: Menagerie, kind: Species, seed: number): Stage {
  const hover = kind.locomotion.hover ?? 0;
  let stage = advance(menagerie, openStage(seed), [
    { kind: "surveyed", width: WIDTH, height: FLOOR, surfaces: [{ id: "floor", x0: 0, x1: WIDTH, y: FLOOR }], keepouts: [] },
    { kind: "tuned", mode: "lively" },
    { kind: "summoned", species: [kind.id] },
    { kind: "ticked", ticks: 64 },
  ]);
  stage = forced(stage, { x: HOME, y: FLOOR - hover, goal: HOME, facing: 1, opacity: 1, activity: "idle", clip: kind.repertoire.idle?.[0] ?? null, since: stage.tick, until: stage.tick + 100000, blink: stage.tick + 100000, partner: null, perch: "floor", vx: 0, vy: 0, gaze: { x: 0.3, y: 0, vx: 0, vy: 0 }, mood: 0.3 });
  return stage;
}

/** ✍️ The stage with fields of its only actor replaced. */
function forced(stage: Stage, patch: Partial<Actor>): Stage {
  const actor = stage.actors[0];
  if (actor === undefined) throw new Error("the actor never arrived");
  return { ...stage, actors: [{ ...actor, ...patch }] };
}

/** 📸️ The frames of the only actor at the listed ticks after now (ascending), and the stage after the last of them. */
function burst(menagerie: Menagerie, start: Stage, ticks: readonly number[], events: readonly StageEvent[] = []): { shots: Shot[]; stage: Stage } {
  let stage = events.length > 0 ? advance(menagerie, start, events) : start;
  const shots: Shot[] = [];
  let passed = 0;
  for (const tick of ticks) {
    for (; passed < tick; passed++) stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    const frame = frameOf(menagerie, stage).actors[0];
    const actor = stage.actors[0];
    if (frame === undefined || actor === undefined) break;
    shots.push({ label: `+${tick} ${actor.activity}${actor.clip === null ? "" : ` ${actor.clip}`}`, frame, dx: frame.x - HOME, dy: frame.y - (FLOOR - 0) });
  }
  return { shots, stage };
}

const spread = (length: number, count: number, tail: readonly number[] = []): number[] => {
  const ticks: number[] = [];
  for (let index = 0; index < count; index++) ticks.push(Math.round((length * index) / (count - 1)));
  for (const extra of tail) ticks.push(length + extra);
  return [...new Set(ticks)].sort((a, b) => a - b);
};

/** 🎬️ Every row of the sheet of one species. */
function rowsOf(kind: Species): Row[] {
  const menagerie: Menagerie = { schema: MENAGERIE_SCHEMA, id: "probe", title: { en: "Probe", de: "Probe" }, species: [kind], bonds: [], casts: [] };
  const hover = kind.locomotion.hover ?? 0;
  const base = stageOf(menagerie, kind, 7);
  const now = base.tick;
  const clip = (id: string | undefined): Clip | null => kind.clips.find((candidate) => candidate.id === id) ?? null;
  const rows: Row[] = [];
  const wanted = (name: string): boolean => only === "" || only.split(",").includes(name);
  const push = (name: string, title: string, shots: readonly Shot[], reach = 0, drop = 0, zoom = scale, marks = false, lead = 0): void => {
    if (wanted(name)) rows.push({ title, shots, reach, lead, drop, zoom, marks });
  };

  const still = advance(menagerie, base, [{ kind: "tuned", mode: "still" }]);
  const rest = frameOf(menagerie, still).actors[0]!;
  const looks: Shot[] = [{ label: "rest (still mode)", frame: rest, dx: 0, dy: -hover }];
  for (const [label, x, y] of [["look left", -300, -40], ["look right", 300, -40], ["look up", 0, -260], ["look down-left", -60, 60]] as const) {
    const looked = burst(menagerie, base, [70], [{ kind: "pointed", x: HOME + x, y: FLOOR - hover - kind.size.height * 0.6 + y }]);
    looks.push({ ...looked.shots[0]!, label });
  }
  const mirrored = burst(menagerie, forced(base, { facing: -1 }), [70], [{ kind: "pointed", x: HOME - 300, y: FLOOR - 60 }]);
  looks.push({ ...mirrored.shots[0]!, label: "mirrored, look left" });
  const mirroredRight = burst(menagerie, forced(base, { facing: -1 }), [70], [{ kind: "pointed", x: HOME + 300, y: FLOOR - 60 }]);
  looks.push({ ...mirroredRight.shots[0]!, label: "mirrored, look right" });
  push("rest", "rest and gaze", looks);
  push("small", "the same at 1×", looks, 0, 0, 1);
  push("blink", "blink", burst(menagerie, forced(base, { blink: now + 1 }), [0, 2, 4, 6, 7, 8, 10, 12, 14]).shots);

  const idle = clip(kind.repertoire.idle?.[0]);
  if (idle !== null) push("idle", `idle · ${idle.id} · ${idle.seconds}s loop`, burst(menagerie, base, spread(clipTicks(idle), 12)).shots);

  for (const id of kind.repertoire.fidget ?? []) {
    const fidget = clip(id);
    if (fidget === null) continue;
    const length = clipTicks(fidget);
    const started = forced(base, { activity: "fidget", clip: id, since: now, until: now + (fidget.loop ? 96 : length), mood: 0.5 });
    push("fidget", `fidget · ${id} · ${fidget.seconds}s ${fidget.loop ? "loop" : "once"}`, burst(menagerie, started, spread(length, 20, [2, 6])).shots);
  }

  const gait = clip(kind.repertoire.walk?.[0] ?? (kind.locomotion.gait === "hop" ? kind.repertoire.hop?.[0] : kind.repertoire.idle?.[0]));
  if (gait !== null) {
    const length = clipTicks(gait);
    const cycles = 2;
    const far = (kind.locomotion.speed * length * 4) / 64;
    const walking = forced(base, { activity: "walk", clip: gait.id, goal: HOME + far, since: now, until: now + 1920, mood: 0.4 });
    const reach = (kind.locomotion.speed * length * cycles) / 64;
    push("walk", `walk · ${gait.id} · ${gait.seconds}s · ${kind.locomotion.gait} at ${kind.locomotion.speed}px/s · stride/cycle ${((kind.locomotion.speed * length) / 64).toFixed(1)}px (ground marks every 10px)`, burst(menagerie, walking, spread(length * cycles, 17)).shots, reach, 0, scale, true);
    const turning = forced(base, { activity: "walk", clip: gait.id, goal: HOME - far, since: now, until: now + 1920, mood: 0.4 });
    push("turn", "turning round, then walking left", burst(menagerie, turning, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 16, 20, 24, 28]).shots, 0, 0, scale, false, 16);
    const short = forced(base, { activity: "walk", clip: gait.id, goal: HOME + (kind.locomotion.speed * length * 1) / 64, since: now, until: now + 1920, mood: 0.4 });
    push("stop", "the end of a walk (one cycle, fade out into idle)", burst(menagerie, short, spread(length + 12, 12)).shots, (kind.locomotion.speed * length) / 64);
  }

  for (const activity of ["greet", "cuddle", "squabble", "sulk"] as const) {
    for (const id of kind.repertoire[activity] ?? []) {
      const played = clip(id);
      if (played === null) continue;
      const span = activity === "sulk" ? 256 : 208;
      const started = forced(base, { activity, clip: id, since: now, until: now + span });
      push(activity, `${activity} · ${id} · ${played.seconds}s ${played.loop ? "loop" : "once"} · shown over ${(span / 64).toFixed(2)}s`, burst(menagerie, started, spread(span, 20, [2, 6])).shots);
    }
  }

  const sleep = clip(kind.repertoire.sleep?.[0]);
  if (sleep !== null) {
    const started = forced(base, { activity: "sleep", clip: sleep.id, since: now, until: now + clipTicks(sleep) + 16, mood: 0.1 });
    push("sleep", `sleep · ${sleep.id} · ${sleep.seconds}s`, burst(menagerie, started, spread(clipTicks(sleep), 10, [12, 20, 28])).shots);
  }

  const land = clip(kind.repertoire.land?.[0]);
  const falling = forced(base, { activity: "fall", perch: null, y: FLOOR - hover - 70, vy: 0, clip: kind.repertoire.fall?.[0] ?? kind.repertoire.idle?.[0] ?? null, since: now, until: now + 640, mood: 0.3 });
  const fallen = burst(menagerie, falling, [0, 4, 8, 12, 14, 16, 17, 18, 19, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 44, 48, 52]);
  push("land", `fall and land · ${land === null ? "no clip" : `${land.id} · ${land.seconds}s`}`, fallen.shots, 0, 70);
  return rows;
}

/** 🧱️ One frame in its cell: the edge it stands on, the box of its species at its feet, the drawing as the layer places it. */
function cellMarkup(kind: Species, row: Row, shot: Shot): string {
  const zoom = row.zoom;
  const hover = kind.locomotion.hover ?? 0;
  const margin = 16;
  const width = (kind.size.width + margin * 2 + row.reach + row.lead) * zoom;
  const height = (kind.size.height + hover + margin + 10 + row.drop) * zoom + 14;
  const ground = (kind.size.height + hover + margin + row.drop) * zoom;
  const feetX = (margin + kind.size.width / 2 + row.lead + shot.dx) * zoom;
  const feetY = ground + shot.dy * zoom;
  const frame = shot.frame;
  const style = `transform: translate(${feetX.toFixed(2)}px, ${feetY.toFixed(2)}px) scale(${frame.facing * zoom}, ${zoom}); opacity: ${frame.opacity}; --pet-body: ${kind.palette.body}; --pet-accent: ${kind.palette.accent}; --pet-detail: ${kind.palette.detail}`;
  const drawing = depictionMarkup(kind, frame).replace("<svg ", `<svg style="${style}" `);
  let marks = "";
  if (row.marks) for (let x = 0; x * zoom < width; x += 10) marks += `<i class="mark" style="left:${x * zoom}px;top:${ground}px"></i>`;
  const box = `<i class="box" style="left:${feetX - (kind.size.width / 2) * zoom}px;top:${feetY - kind.size.height * zoom}px;width:${kind.size.width * zoom}px;height:${kind.size.height * zoom}px"></i>`;
  return `<figure class="cell" data-feet-x="${feetX.toFixed(2)}" data-feet-y="${feetY.toFixed(2)}" data-zoom="${zoom}" data-hover="${hover}" data-width="${kind.size.width}" data-height="${kind.size.height}" style="width:${width}px;height:${height}px"><i class="card" style="top:${ground}px"></i>${marks}${box}${drawing}<figcaption>${shot.label}</figcaption></figure>`;
}

mkdirSync(out, { recursive: true });
for (const file of files) {
  const kind = JSON.parse(readFileSync(file, "utf8")) as Species;
  const rows = rowsOf(kind);
  let body = "";
  for (const theme of THEMES) {
    rows.forEach((row, index) => {
      body += `<section class="sheet ${theme.name}" data-name="${kind.id}-${theme.name}-${String(index).padStart(2, "0")}" data-title="${kind.id} · ${row.title}" style="--foreground:${theme.ink};--base:${theme.base};--panel:${theme.panel};--border:${theme.border}"><h2>${kind.id} · ${row.title} · ${theme.name}</h2><div class="row">${row.shots.map((shot) => cellMarkup(kind, row, shot)).join("")}</div></section>`;
    });
  }
  const html = `<!doctype html><meta charset="utf-8"><title>${kind.id}</title><style>${CSS}
body{margin:0;background:#666;font:10px monospace}
.sheet{display:inline-block;background:var(--base);color:var(--foreground);padding:4px 6px 6px;margin:0 0 6px}
.sheet h2{margin:0 0 3px;font:11px monospace}
.row{display:flex;gap:3px;flex-wrap:wrap;max-width:1580px}
.cell{position:relative;margin:0;overflow:hidden;background:var(--base);outline:1px solid color-mix(in srgb, var(--foreground) 12%, transparent)}
.cell .card{position:absolute;left:0;right:0;bottom:14px;background:var(--panel);border-top:1px solid var(--border)}
.cell .mark{position:absolute;width:1px;height:5px;background:var(--foreground);opacity:.6}
.cell .box{position:absolute;box-sizing:border-box;border:1px dashed color-mix(in srgb, var(--foreground) 30%, transparent)}
.cell figcaption{position:absolute;left:1px;bottom:0;height:13px;font:9px monospace;white-space:nowrap}
</style>${body}`;
  writeFileSync(join(out, `${kind.id}.html`), html);
  console.log(`wrote ${join(out, `${kind.id}.html`)} (${rows.length} rows)`);
}
