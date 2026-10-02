/** 🎬️ Ticket tool (work packages O): plays every activity of a species on the real stage (`openStage`, `advance`, `frameOf`) with scripted events and writes, per species and theme, an HTML sheet of frame bursts drawn by the product's `depictionMarkup`, plus a text report of numbers the eye cannot judge (pupil travel, loop and rest seams, foot slip, pops).
 *
 * Usage (from the repository root): bun "<ticket>/wp_o1_stage_sheets.ts" --out <directory> [--scale 2] [--partner <species id>] [--rows <word,word>] <species id>...
 *
 * Paths are found by their ASCII names, so no emoji is typed here. The sheets are screenshot by `wp_o1_shoot.mjs`.
 */
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const args = process.argv.slice(2);
const option = (name: string, fallback: string): string => {
  const index = args.indexOf(`--${name}`);
  if (index < 0) return fallback;
  const [, value] = args.splice(index, 2);
  return value ?? fallback;
};
const ticket = import.meta.dir;
const root = resolve(ticket, "..", "..", "..", "..", "..", "..", "..");
const plain = (name: string): string => name.replace(/[^\x20-\x7e]/gu, "");
const seek = (base: string, ...names: string[]): string => {
  let path = base;
  for (const name of names) {
    const found = readdirSync(path).find((entry) => plain(entry) === name);
    if (found === undefined) throw new Error(`no ${name} in ${path}`);
    path = join(path, found);
  }
  return path;
};
const out = resolve(option("out", join(seek(ticket, "generated"), "wp-o1")));
const scale = Number(option("scale", "2"));
const partnerId = option("partner", "");
const only = option("rows", "").split(",").filter((word) => word.length > 0);
const wantedIds = args;
if (wantedIds.length === 0) throw new Error("no species ids given");

const product = seek(root, "framework", "products", "pets");
const core = await import(pathToFileURL(seek(product, "packages", "typescript", ".ts")).href);
const { depictionMarkup } = await import(pathToFileURL(seek(product, "targets", "react", "modules", "depiction", ".ts")).href);
const paints = readFileSync(seek(product, "targets", "react", ".css"), "utf8");
const menagerieDirectory = seek(root, "teaching", "architecture", "pets");
const speciesOf = (id: string): any => JSON.parse(readFileSync(seek(menagerieDirectory, id, ".json"), "utf8"));

const { openStage, advance, frameOf, clipTicks, hopOf, speciesIssues } = core;
const FLOOR = 250;
const HOME = 300;

type Shot = { label: string; frame: any; stage: any };
type Row = { title: string; note: string; shots: Shot[]; wide: boolean; unit?: number };

const menagerieOf = (kinds: any[]): any => ({ schema: "semio.pets.menagerie/v1", id: "preview", title: { en: "Preview", de: "Vorschau" }, species: kinds, bonds: [], casts: [{ scene: "home", core: kinds.map((kind) => kind.id), rotation: [] }] });
const patched = (stage: any, id: string, fields: object): any => ({ ...stage, actors: stage.actors.map((actor: any) => (actor.species === id ? { ...actor, ...fields } : actor)) });
const actorOf = (stage: any, id: string): any => stage.actors.find((actor: any) => actor.species === id);
const clipOf = (kind: any, id: string): any => kind.clips.find((clip: any) => clip.id === id);
const hoverOf = (kind: any): number => (kind.locomotion.gait === "float" ? (kind.locomotion.hover ?? 0) : 0);

/** 🏁️ A stage on which the kinds stand idle on a wide floor (and a ledge to hop on), awake, not blinking, undisturbed. */
const baseStage = (menagerie: any, kinds: any[], ledge: boolean): any => {
  const surfaces = [{ id: "floor", x0: 0, x1: 900, y: FLOOR }];
  if (ledge) surfaces.push({ id: "ledge", x0: HOME + 30, x1: HOME + 330, y: FLOOR - 34 });
  let stage = advance(menagerie, openStage(7), [
    { kind: "surveyed", width: 900, height: FLOOR, surfaces, keepouts: [] },
    { kind: "summoned", species: kinds.map((kind) => kind.id) },
  ]);
  for (let guard = 0; guard < 2000 && (stage.actors.length < kinds.length || stage.actors.some((actor: any) => actor.opacity < 1)); guard++) stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
  if (stage.actors.length < kinds.length) throw new Error("the cast did not arrive");
  kinds.forEach((kind, index) => {
    stage = patched(stage, kind.id, { x: HOME + index * 260, goal: HOME + index * 260, y: FLOOR - hoverOf(kind), perch: "floor", facing: index === 0 ? 1 : -1, activity: "idle", since: stage.tick, until: 1e9, partner: null, clip: kind.repertoire.idle?.[0] ?? null, blink: 1e9, mood: 0.3, gaze: { x: 0, y: 0, vx: 0, vy: 0 }, vx: 0, vy: 0, opacity: 1, leaving: false });
  });
  return stage;
};

/** 🎞️ Plays `ticks` ticks from `stage` and keeps the frames at the offsets in `at`. */
const play = (menagerie: any, stage: any, at: number[], events: (offset: number) => any[] = () => []): { shots: Shot[]; stages: any[] } => {
  const shots: Shot[] = [];
  const stages: any[] = [stage];
  const last = Math.max(...at);
  let current = stage;
  for (let offset = 0; offset <= last; offset++) {
    if (offset > 0) {
      current = advance(menagerie, current, [...events(offset), { kind: "ticked", ticks: 1 }]);
      stages.push(current);
    }
    if (at.includes(offset)) shots.push({ label: `t${offset}`, frame: frameOf(menagerie, current), stage: current });
  }
  return { shots, stages };
};
const spread = (ticks: number, count: number): number[] => [...new Set(Array.from({ length: count }, (_, index) => Math.round((index * ticks) / count)))];

/** 📏️ The largest move of any bone frame between two consecutive stages of one actor (pixels of a point 10 px from every bone origin, plus the feet). */
const jumpOf = (menagerie: any, kind: any, before: any, after: any): number => {
  const one = frameOf(menagerie, before).actors.find((actor: any) => actor.species === kind.id);
  const two = frameOf(menagerie, after).actors.find((actor: any) => actor.species === kind.id);
  if (!one || !two) return 0;
  let most = 0;
  for (let bone = 0; bone < kind.bones.length; bone++) {
    for (const [px, py] of [[0, 0], [10, 0], [0, 10]] as const) {
      const at = (frame: any): [number, number] => {
        const m = frame.bones.slice(bone * 6, bone * 6 + 6);
        return [frame.facing * (m[0] * px + m[2] * py + m[4]) + frame.x, m[1] * px + m[3] * py + m[5] + frame.y];
      };
      const [ax, ay] = at(one);
      const [bx, by] = at(two);
      most = Math.max(most, Math.hypot(bx - ax - (two.x - one.x), by - ay - (two.y - one.y)));
    }
  }
  return most;
};

const THEMES = [
  { name: "light", ground: "#f7f3e3", card: "#ffffff", line: "#001117", ink: "#001117" },
  { name: "dark", ground: "#001117", card: "#0c2026", line: "#f7f3e3", ink: "#f7f3e3" },
];

const cellMarkup = (kinds: any[], target: any, shot: Shot, wide: boolean, theme: (typeof THEMES)[number], unit: number): string => {
  const frame = shot.frame;
  const mine = frame.actors.find((actor: any) => actor.species === target.id);
  if (!mine) return `<figure class="cell"><figcaption>${shot.label} (gone)</figcaption></figure>`;
  const margin = 16;
  const others = wide ? frame.actors : [mine];
  const lefts = others.map((actor: any) => actor.x - kinds.find((kind) => kind.id === actor.species).size.width / 2);
  const rights = others.map((actor: any) => actor.x + kinds.find((kind) => kind.id === actor.species).size.width / 2);
  const tallest = Math.max(...others.map((actor: any) => kinds.find((kind) => kind.id === actor.species).size.height + hoverOf(kinds.find((kind) => kind.id === actor.species))));
  const left = Math.min(...lefts) - margin;
  const right = Math.max(...rights) + margin;
  const ground = FLOOR;
  const top = Math.min(mine.y - target.size.height, ground - tallest) - margin - 6;
  const bottom = ground + 12;
  const width = right - left;
  const height = bottom - top;
  let marks = "";
  for (let x = Math.ceil(left / 8) * 8; x < right; x += 8) marks += `<line x1="${x}" y1="${ground}" x2="${x}" y2="${ground + (x % 40 === 0 ? 5 : 2.5)}" stroke="${theme.line}" stroke-opacity="0.5" stroke-width="0.4"/>`;
  let drawn = "";
  for (const actor of others) {
    const kind = kinds.find((candidate) => candidate.id === actor.species);
    const inner = depictionMarkup(kind, actor).replace(/^<svg[^>]*>/u, "").replace(/<\/svg>$/u, "");
    drawn += `<g class="pet-g${actor === mine ? " target" : ""}" data-facing="${actor.facing}" transform="translate(${actor.x} ${actor.y}) scale(${actor.facing} 1)" opacity="${actor.opacity}" style="--pet-body:${kind.palette.body};--pet-accent:${kind.palette.accent};--pet-detail:${kind.palette.detail}">${inner}</g>`;
  }
  const box = `<rect x="${mine.x - target.size.width / 2}" y="${mine.y - target.size.height}" width="${target.size.width}" height="${target.size.height}" fill="none" stroke="${theme.line}" stroke-opacity="0.35" stroke-dasharray="2 2" stroke-width="0.4"/>`;
  const state = actorOf(shot.stage, target.id);
  return `<figure class="cell" data-ox="${mine.x - left}" data-oy="${mine.y - top}" data-unit="${unit}"><svg xmlns="http://www.w3.org/2000/svg" width="${width * unit}" height="${height * unit}" viewBox="${left} ${top} ${width} ${height}">
<rect x="${left}" y="${ground}" width="${width}" height="${bottom - ground}" fill="${theme.card}"/><line x1="${left}" y1="${ground}" x2="${right}" y2="${ground}" stroke="${theme.line}" stroke-width="0.6"/>${marks}${box}${drawn}</svg><figcaption>${shot.label} ${state ? state.activity : ""}</figcaption></figure>`;
};

const pageOf = (kinds: any[], target: any, rows: Row[], theme: (typeof THEMES)[number]): string => {
  let body = "";
  let page = "";
  let count = 0;
  let height = 0;
  const flush = (): void => {
    if (page) body += `<section class="page" data-page="${count++}">${page}</section>`;
    page = "";
    height = 0;
  };
  for (const row of rows) {
    const unit = row.unit ?? scale;
    const partner = kinds.find((kind) => kind.id !== target.id);
    const across = ((row.wide && partner ? target.size.width + partner.size.width + 6 : target.size.width) + 32) * unit + 3;
    const lines = Math.ceil(row.shots.length / Math.max(1, Math.floor(1544 / across)));
    const tall = lines * ((Math.max(target.size.height + hoverOf(target), row.wide && partner ? partner.size.height + hoverOf(partner) : 0) + 40) * unit + 16) + 24;
    if (height > 0 && height + tall > 1180) flush();
    page += `<div class="row" data-row="${row.title}"><h3>${target.id} · ${row.title} <small>${row.note}</small></h3><div class="cells">${row.shots.map((shot) => cellMarkup(kinds, target, shot, row.wide, theme, unit)).join("")}</div></div>`;
    height += tall;
  }
  flush();
  const strip = paints.replace(/\.pet\s*\{[^}]*\}/u, "").replace(/\.pet-layer\s*\{[^}]*\}/gu, "");
  return `<!doctype html><meta charset="utf-8"><title>${target.id} ${theme.name}</title><style>${strip}
:root{--pet-ink:${theme.ink}}body{margin:0;background:${theme.ground};color:${theme.ink};font:11px monospace}
.page{padding:6px 8px;width:1560px;box-sizing:border-box;background:${theme.ground}}.row{margin-bottom:6px}h3{margin:2px 0;font-size:12px}small{font-weight:normal;opacity:.7}
.cells{display:flex;flex-wrap:wrap;gap:3px}.cell{margin:0}figcaption{font-size:9px;opacity:.8}.pet-g{stroke-linejoin:round;stroke-linecap:round}</style>${body}`;
};

const report: string[] = [];
const say = (line: string): void => {
  report.push(line);
};

for (const id of wantedIds) {
  const target = speciesOf(id);
  const partner = speciesOf(partnerId || (id === "sunny" ? "cloudy" : "sunny"));
  const issues = speciesIssues(target);
  const solo = menagerieOf([target]);
  const duo = menagerieOf([target, partner]);
  const rows: Row[] = [];
  const base = baseStage(solo, [target], false);
  const idleClip = clipOf(target, target.repertoire.idle?.[0]);
  const lines: string[] = [];
  const note = (line: string): void => {
    lines.push(line);
  };
  note(`# ${id} — ${target.size.width}×${target.size.height}, ${target.locomotion.gait} ${target.locomotion.speed} px/s${target.locomotion.hover ? `, hover ${target.locomotion.hover}` : ""}; validator issues: ${issues.length === 0 ? "none" : JSON.stringify(issues)}`);
  for (const eye of target.face.eyes) note(`eye ${eye.id}: radius ${eye.radius}, pupil ${eye.pupil} (${Math.round((eye.pupil / eye.radius) * 100)} %), travel ${(eye.radius - eye.pupil - 0.25).toFixed(2)} px, at (${eye.x}, ${eye.y}) on ${eye.bone}`);
  for (const [activity, clips] of Object.entries(target.repertoire)) note(`repertoire ${activity}: ${(clips as string[]).map((clip) => `${clip} (${clipOf(target, clip)?.seconds}s ${clipOf(target, clip)?.loop ? "loop" : "once"})`).join(", ")}`);
  for (const clip of target.clips) {
    const ends: string[] = [];
    for (const track of clip.tracks) {
      const rest = track.channel === "scaleX" || track.channel === "scaleY" ? 1 : 0;
      const first = track.keys[0].value;
      const final = track.keys[track.keys.length - 1].value;
      if (!clip.loop && (first !== rest || final !== rest)) ends.push(`${track.bone}.${track.channel} ${first}→${final}`);
    }
    if (ends.length > 0) note(`one-shot ${clip.id} does not start/end at rest: ${ends.join("; ")}`);
  }

  const measure = (title: string, stages: any[]): void => {
    let most = 0;
    let where = 0;
    for (let index = 1; index < stages.length; index++) {
      const jump = jumpOf(solo.species.length === 1 ? solo : duo, target, stages[index - 1], stages[index]);
      if (jump > most) {
        most = jump;
        where = index;
      }
    }
    note(`pop ${title}: largest move in one tick ${most.toFixed(2)} px at t${where} of ${stages.length - 1}`);
  };

  const still = [
    { label: "ahead", events: [] as any[] },
    { label: "look ←", events: [{ kind: "pointed", x: HOME - 200, y: FLOOR - 30 }] },
    { label: "look →", events: [{ kind: "pointed", x: HOME + 200, y: FLOOR - 30 }] },
    { label: "look ↑", events: [{ kind: "pointed", x: HOME, y: FLOOR - 250 }] },
    { label: "look ↘", events: [{ kind: "pointed", x: HOME + 150, y: FLOOR + 120 }] },
  ];
  const restShots: Shot[] = [];
  for (const facing of [1, -1]) {
    for (const look of still) {
      let stage = patched(base, id, { facing });
      stage = advance(solo, stage, look.events);
      for (let tick = 0; tick < 48; tick++) stage = advance(solo, stage, [...look.events, { kind: "ticked", ticks: 1 }]);
      restShots.push({ label: `${facing === 1 ? "" : "mirrored "}${look.label}`, frame: frameOf(solo, stage), stage });
    }
  }
  for (const mood of [-0.8, 1]) {
    const stage = patched(base, id, { mood });
    restShots.push({ label: `mood ${mood}`, frame: frameOf(solo, stage), stage });
  }
  rows.push({ title: "rest, gaze, mood (idle loop running)", note: "pointer left, right, up, down-right; mirrored; sad and happy mouth", shots: restShots, wide: false });
  rows.push({ title: "the same at scale 1", note: "what a learner sees", shots: restShots, wide: false, unit: 1 });

  const blink = play(solo, patched(base, id, { blink: base.tick + 2 }), [0, 3, 5, 7, 9, 11, 13, 15]);
  rows.push({ title: "blink", note: "12 ticks", shots: blink.shots, wide: false });

  if (idleClip) {
    const ticks = clipTicks(idleClip);
    const idle = play(solo, base, [...spread(ticks, 12), ticks]);
    rows.push({ title: `idle · ${idleClip.id}`, note: `${idleClip.seconds}s loop`, shots: idle.shots, wide: false });
    measure(`idle ${idleClip.id}`, idle.stages);
  }

  for (const activity of ["fidget", "greet", "cuddle", "squabble", "sulk", "sleep"]) {
    for (const clipId of target.repertoire[activity] ?? []) {
      const clip = clipOf(target, clipId);
      const ticks = clipTicks(clip);
      const span = activity === "fidget" && !clip.loop ? ticks : activity === "fidget" ? 96 : activity === "sleep" ? ticks + 16 : Math.max(192, Math.min(ticks * 2, 320));
      const social = activity === "greet" || activity === "cuddle" || activity === "squabble" || activity === "sulk";
      let stage: any;
      let menagerie = solo;
      if (social) {
        menagerie = duo;
        stage = baseStage(duo, [target, partner], false);
        const gap = (target.size.width + partner.size.width) / 2 + 6;
        const partnerClip = partner.repertoire[activity]?.[0] ?? null;
        stage = patched(stage, partner.id, { x: HOME + gap, goal: HOME + gap, facing: activity === "sulk" ? 1 : -1, activity, since: stage.tick, until: stage.tick + span, partner: id, clip: partnerClip });
        stage = patched(stage, id, { facing: activity === "sulk" ? -1 : 1, activity, since: stage.tick, until: stage.tick + span, partner: partner.id, clip: clipId });
      } else {
        stage = patched(base, id, { activity, since: base.tick, until: base.tick + span, clip: clipId });
      }
      const at = [...spread(span, 13), span - 6, span - 3, span, span + 3, span + 8];
      const played = play(menagerie, stage, [...new Set(at)].sort((a, b) => a - b));
      rows.push({ title: `${activity} · ${clipId}`, note: `${clip.seconds}s ${clip.loop ? "loop" : "once"}, played for ${span} ticks (${(span / 64).toFixed(2)} s), then what follows`, shots: played.shots, wide: social });
      const whole = play(menagerie, stage, [span + 12]);
      let most = 0;
      let where = 0;
      for (let index = 1; index < whole.stages.length; index++) {
        const jump = jumpOf(menagerie, target, whole.stages[index - 1], whole.stages[index]);
        if (index >= span - 9 && jump > most) {
          most = jump;
          where = index;
        }
      }
      let peak = 0;
      for (let index = 1; index < span - 9; index++) peak = Math.max(peak, jumpOf(menagerie, target, whole.stages[index - 1], whole.stages[index]));
      note(`pop ${activity} ${clipId}: in the clip ≤ ${peak.toFixed(2)} px per tick; at its end (last 9 ticks and 12 after) ≤ ${most.toFixed(2)} px at t${where} of ${span}`);
    }
  }

  const gaitClipId = target.repertoire.walk?.[0] ?? (target.locomotion.gait === "hop" ? target.repertoire.hop?.[0] : target.repertoire.idle?.[0]);
  const gaitClip = gaitClipId ? clipOf(target, gaitClipId) : null;
  if (gaitClip) {
    const ticks = clipTicks(gaitClip);
    const far = (target.locomotion.speed * ticks * 3) / 64 + 20;
    const stage = patched(base, id, { activity: "walk", since: base.tick, until: base.tick + 1920, goal: HOME + far, clip: gaitClipId });
    const walk = play(solo, stage, [...spread(ticks, 8).map((tick) => tick + 8), ...spread(ticks, 8).map((tick) => tick + 8 + ticks)]);
    rows.push({ title: `walk · ${gaitClipId}`, note: `${gaitClip.seconds}s loop at ${target.locomotion.speed} px/s = ${((target.locomotion.speed * ticks) / 64).toFixed(1)} px per cycle; ground marks every 8 px`, shots: walk.shots, wide: false });
    const cycle = play(solo, stage, [8 + ticks * 2]);
    for (const part of target.parts) {
      if (!/leg|foot|skate|paw/u.test(part.id) || (part.shape.kind !== "line" && part.shape.kind !== "path")) continue;
      const bone = target.bones.findIndex((candidate: any) => candidate.id === part.bone);
      let tip = { x: 0, y: -Infinity };
      if (part.shape.kind === "line") tip = part.shape.y2 >= part.shape.y1 ? { x: part.shape.x2, y: part.shape.y2 } : { x: part.shape.x1, y: part.shape.y1 };
      else {
        const numbers = (part.shape.d.match(/-?\d*\.?\d+/gu) ?? []).map(Number);
        for (let index = 0; index + 1 < numbers.length; index += 2) if (numbers[index + 1] > tip.y) tip = { x: numbers[index], y: numbers[index + 1] };
      }
      const trail: string[] = [];
      let lowest = -Infinity;
      for (let offset = 8 + ticks; offset <= 8 + ticks * 2; offset += Math.max(1, Math.round(ticks / 12))) {
        const frame = frameOf(solo, cycle.stages[offset]).actors[0];
        const m = frame.bones.slice(bone * 6, bone * 6 + 6);
        const tipX = m[0] * tip.x + m[2] * tip.y + m[4];
        const tipY = m[1] * tip.x + m[3] * tip.y + m[5];
        lowest = Math.max(lowest, tipY);
        trail.push(`${(frame.x + tipX - HOME).toFixed(1)}/${tipY.toFixed(1)}`);
      }
      note(`walk ${gaitClipId} ${part.id}: tip world x / y over one cycle: ${trail.join("  ")} (lowest ${lowest.toFixed(2)})`);
    }
    const back = patched(base, id, { activity: "walk", since: base.tick, until: base.tick + 1920, goal: HOME - 60, clip: gaitClipId });
    const turn = play(solo, back, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 16, 22, 28]);
    rows.push({ title: "turning round, then walking back", note: "8 ticks squeezed through a line", shots: turn.shots, wide: false });
    const short = (target.locomotion.speed * 40) / 64;
    const stop = play(solo, patched(base, id, { activity: "walk", since: base.tick, until: base.tick + 1920, goal: HOME + short, clip: gaitClipId }), [28, 32, 34, 36, 38, 40, 42, 44, 48]);
    rows.push({ title: "coming to rest after a walk", note: "the gait fades out over the last 8 ticks", shots: stop.shots, wide: false });
    measure("walk start, cycle, stop", play(solo, patched(base, id, { activity: "walk", since: base.tick, until: base.tick + 1920, goal: HOME + short, clip: gaitClipId }), [52]).stages);
  }

  const drop = patched(base, id, { activity: "fall", perch: null, y: FLOOR - hoverOf(target) - 70, vy: 0, since: base.tick, until: base.tick + 640, clip: target.repertoire.fall?.[0] ?? target.repertoire.idle?.[0] ?? null });
  const dropped = play(solo, drop, [80]);
  const landed = dropped.stages.findIndex((stage: any) => actorOf(stage, id)?.activity === "land");
  if (landed >= 0) {
    const landClip = clipOf(target, actorOf(dropped.stages[landed], id).clip);
    const at = [0, Math.max(0, landed - 6), Math.max(0, landed - 3), landed, landed + 2, landed + 4, landed + 6, landed + 9, landed + 12, landed + 15, landed + 18, landed + 21, landed + 24, landed + 27, landed + 32, landed + 40].filter((tick) => tick <= 80);
    const fall = play(solo, drop, [...new Set(at)]);
    rows.push({ title: `fall and land · ${landClip?.id ?? "no clip"}`, note: `dropped from 70 px; lands at t${landed}; ${landClip ? `${landClip.seconds}s once` : ""}`, shots: fall.shots, wide: false });
    measure("fall and land", dropped.stages.slice(landed));
  } else note("fall: never landed within 80 ticks");

  const hopStage = baseStage(solo, [target], true);
  const from = { x: HOME, y: FLOOR - hoverOf(target) };
  const to = { x: HOME + 70, y: FLOOR - 34 };
  if (target.locomotion.gait === "float") {
    const dx = to.x - from.x;
    const dy = to.y - hoverOf(target) - from.y;
    const ticks = Math.floor((Math.sqrt(dx * dx + dy * dy) * 64) / (2 * target.locomotion.speed) + 0.5);
    const glide = play(solo, patched(hopStage, id, { activity: "hop", perch: null, vx: (dx * 64) / ticks, vy: (dy * 64) / ticks, goal: to.x, since: hopStage.tick, until: hopStage.tick + ticks, clip: target.repertoire.hop?.[0] ?? target.repertoire.idle?.[0] ?? null }), spread(ticks + 30, 10));
    rows.push({ title: "glide to a ledge (hop of a floater)", note: `${ticks} ticks`, shots: glide.shots, wide: false });
  } else {
    const hop = hopOf(from, to);
    if (hop) {
      const flight = play(solo, patched(hopStage, id, { activity: "hop", perch: null, vx: hop.vx, vy: hop.vy, goal: to.x, since: hopStage.tick, until: hopStage.tick + hop.ticks, clip: target.repertoire.hop?.[0] ?? target.repertoire.idle?.[0] ?? null }), [...spread(hop.ticks, 8), hop.ticks, hop.ticks + 4, hop.ticks + 10, hop.ticks + 20, hop.ticks + 34]);
      rows.push({ title: `hop onto a ledge · ${target.repertoire.hop?.[0] ?? "no hop clip (idle loop)"}`, note: `${hop.ticks} ticks in the air, then land`, shots: flight.shots, wide: false });
    } else note("hop: out of reach");
  }

  mkdirSync(out, { recursive: true });
  const shown = only.length === 0 ? rows : rows.filter((row) => only.some((word) => row.title.includes(word)));
  for (const theme of THEMES) writeFileSync(join(out, `${id}-${theme.name}.html`), pageOf([target, partner], target, shown, theme));
  writeFileSync(join(out, `${id}-numbers.txt`), `${lines.join("\n")}\n`);
  writeFileSync(join(out, `${id}-size.json`), JSON.stringify(target.size));
  say(lines.join("\n"));
  console.log(`wrote ${id}: ${rows.length} rows`);
}
console.log(report.join("\n\n"));
