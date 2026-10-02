/** 🖼️ Ticket tool: renders species documents to a PNG contact sheet (rest pose plus every clip at four phases, on the light and the dark theme) so rigs can be judged by eye.
 *
 * Usage (from the repository root):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/render_species_preview.mjs" --out <file.png> [--scale 3] [--clips idle,walk] <species.json>...
 *
 * The drawing rules mirror 📓️design.md §6.2; this tool uses the platform's Math and is not part of the product.
 */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  if (index < 0) return fallback;
  const [, value] = args.splice(index, 2);
  return value;
};
const out = resolve(option("out", "species-preview.png"));
const scale = Number(option("scale", "3"));
const only = option("clips", "");
const files = args;
if (files.length === 0) throw new Error("no species files given");

const THEMES = [
  { name: "light", base: "#f7f3e3", panel: "#c9c8bd", ink: "#001117", paper: "#f7f3e3" },
  { name: "dark", base: "#001117", panel: "#1d2b2f", ink: "#f7f3e3", paper: "#f7f3e3" },
];
const PHASES = [0, 0.25, 0.5, 0.75];

const bezier = (x1, y1, x2, y2, t) => {
  let low = 0;
  let high = 1;
  for (let step = 0; step < 40; step++) {
    const middle = (low + high) / 2;
    const x = 3 * (1 - middle) * (1 - middle) * middle * x1 + 3 * (1 - middle) * middle * middle * x2 + middle * middle * middle;
    if (x < t) low = middle;
    else high = middle;
  }
  const s = (low + high) / 2;
  return 3 * (1 - s) * (1 - s) * s * y1 + 3 * (1 - s) * s * s * y2 + s * s * s;
};

const sampleTrack = (track, phase) => {
  const keys = track.keys;
  if (phase <= keys[0].at) return keys[0].value;
  for (let index = 0; index + 1 < keys.length; index++) {
    const from = keys[index];
    const to = keys[index + 1];
    if (phase <= to.at) {
      const local = (phase - from.at) / (to.at - from.at);
      const eased = from.ease ? bezier(...from.ease, local) : local;
      return from.value + (to.value - from.value) * eased;
    }
  }
  return keys[keys.length - 1].value;
};

const poseOf = (species, clip, phase) => {
  const pose = new Map(species.bones.map((bone) => [bone.id, { x: 0, y: 0, rotation: 0, scaleX: 1, scaleY: 1 }]));
  if (clip) for (const track of clip.tracks) pose.get(track.bone)[track.channel] = sampleTrack(track, phase);
  return pose;
};

const compose = (p, l) => [p[0] * l[0] + p[2] * l[1], p[1] * l[0] + p[3] * l[1], p[0] * l[2] + p[2] * l[3], p[1] * l[2] + p[3] * l[3], p[0] * l[4] + p[2] * l[5] + p[4], p[1] * l[4] + p[3] * l[5] + p[5]];

const solve = (species, pose) => {
  const world = new Map();
  for (const bone of species.bones) {
    const delta = pose.get(bone.id);
    const angle = (((bone.rotation ?? 0) + delta.rotation) * Math.PI) / 180;
    const sin = Math.sin(angle);
    const cos = Math.cos(angle);
    const local = [cos * delta.scaleX, sin * delta.scaleX, -sin * delta.scaleY, cos * delta.scaleY, bone.x + delta.x, bone.y + delta.y];
    world.set(bone.id, bone.parent === undefined ? local : compose(world.get(bone.parent), local));
  }
  return world;
};

const matrix = (m) => `matrix(${m.map((value) => +value.toFixed(4)).join(" ")})`;
const escape = (text) => String(text).replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/"/g, "&quot;");

const shapeMarkup = (shape, attributes) => {
  if (shape.kind === "path") return `<path d="${escape(shape.d)}" ${attributes}/>`;
  if (shape.kind === "ellipse") return `<ellipse cx="${shape.cx}" cy="${shape.cy}" rx="${shape.rx}" ry="${shape.ry}" ${attributes}/>`;
  if (shape.kind === "rect") return `<rect x="${shape.x}" y="${shape.y}" width="${shape.width}" height="${shape.height}" rx="${shape.radius ?? 0}" ${attributes}/>`;
  return `<line x1="${shape.x1}" y1="${shape.y1}" x2="${shape.x2}" y2="${shape.y2}" ${attributes}/>`;
};

const faceMarkup = (species, world, colour, look) => {
  const face = species.face;
  let markup = "";
  for (const eye of face.eyes) {
    const reach = Math.max(0, eye.radius - eye.pupil - 0.25);
    markup += `<g transform="${matrix(world.get(eye.bone))}"><g transform="translate(${eye.x} ${eye.y}) scale(1 ${1 - 0.9 * look.lid})">`;
    markup += `<circle r="${eye.radius}" fill="${colour("paper")}" stroke="${colour("ink")}" stroke-width="1.5"/>`;
    markup += `<circle cx="${look.x * reach}" cy="${look.y * reach}" r="${eye.pupil}" fill="#001117"/></g></g>`;
  }
  if (face.mouth) {
    const mouth = face.mouth;
    markup += `<g transform="${matrix(world.get(mouth.bone))}"><path d="M ${mouth.x - mouth.width / 2} ${mouth.y} Q ${mouth.x} ${mouth.y + mouth.width * 0.5 * look.mood} ${mouth.x + mouth.width / 2} ${mouth.y}" fill="none" stroke="${colour("ink")}" stroke-width="1.5" stroke-linecap="round"/></g>`;
  }
  return markup;
};

const speciesMarkup = (species, theme, clip, phase, look) => {
  const colour = (paint) => (paint === "none" ? "none" : paint === "ink" ? theme.ink : paint === "paper" ? theme.paper : species.palette[paint]);
  const world = solve(species, poseOf(species, clip, phase));
  let markup = "";
  let faced = false;
  const face = () => {
    if (!faced) markup += faceMarkup(species, world, colour, look);
    faced = true;
  };
  for (const part of species.parts) {
    const attributes = `fill="${colour(part.fill)}" stroke="${colour(part.stroke)}" stroke-width="${part.stroke === "none" ? 0 : (part.strokeWidth ?? 2)}" stroke-linejoin="round" stroke-linecap="round"`;
    markup += `<g transform="${matrix(world.get(part.bone))}">${shapeMarkup(part.shape, attributes)}</g>`;
    if (species.face.above === part.id) face();
  }
  if (species.face.above === undefined) face();
  face();
  return markup;
};

const cellMarkup = (species, theme, clip, phase, label, look) => {
  const margin = 14;
  const hover = species.locomotion.hover ?? 0;
  const width = species.size.width + margin * 2;
  const height = species.size.height + hover + margin * 2 + 12;
  const ground = species.size.height + hover + margin;
  return `<figure style="margin:0;background:${theme.base};color:${theme.ink}"><svg xmlns="http://www.w3.org/2000/svg" width="${width * scale}" height="${height * scale}" viewBox="0 0 ${width} ${height}">
<rect x="0" y="${ground}" width="${width}" height="${height - ground}" fill="${theme.panel}"/>
<rect x="${margin}" y="${ground - hover - species.size.height}" width="${species.size.width}" height="${species.size.height}" fill="none" stroke="${theme.ink}" stroke-opacity="0.18" stroke-dasharray="2 2" stroke-width="0.5"/>
<g transform="translate(${width / 2} ${ground - hover})">${speciesMarkup(species, theme, clip, phase, look)}</g>
<text x="2" y="${height - 3}" font-size="5" font-family="monospace" fill="${theme.ink}">${escape(label)}</text></svg></figure>`;
};

let body = "";
for (const file of files) {
  const species = JSON.parse(readFileSync(file, "utf8"));
  const wanted = only ? only.split(",") : species.clips.map((clip) => clip.id);
  body += `<h2>${escape(species.id)} · ${escape(species.name?.en ?? "")} · ${escape(species.name?.de ?? "")} · ${species.size.width}×${species.size.height} · ${escape(species.locomotion.gait)}</h2>`;
  for (const theme of THEMES) {
    body += `<div class="row">`;
    body += cellMarkup(species, theme, undefined, 0, `rest (${theme.name})`, { x: 0, y: 0, lid: 0, mood: 0.4 });
    body += cellMarkup(species, theme, undefined, 0, "look left, sad", { x: -1, y: 0.2, lid: 0, mood: -0.8 });
    body += cellMarkup(species, theme, undefined, 0, "look right, happy", { x: 1, y: -0.2, lid: 0, mood: 1 });
    body += cellMarkup(species, theme, undefined, 0, "blink", { x: 0, y: 0, lid: 1, mood: 0.4 });
    body += `<span style="display:inline-block;transform:scaleX(-1)">${cellMarkup(species, theme, undefined, 0, "", { x: 0.6, y: 0, lid: 0, mood: 0.4 })}</span>`;
    body += `</div>`;
    if (theme.name !== "light") continue;
    for (const clip of species.clips) {
      if (!wanted.includes(clip.id)) continue;
      body += `<div class="row">`;
      for (const phase of PHASES) body += cellMarkup(species, theme, clip, phase, `${clip.id} @${phase} (${clip.seconds}s${clip.loop ? ", loop" : ""})`, { x: 0, y: 0, lid: 0, mood: 0.4 });
      body += `</div>`;
    }
  }
}

const html = `<!doctype html><meta charset="utf-8"><style>body{margin:8px;background:#888;font:12px monospace}h2{margin:10px 0 4px;color:#fff;font-size:13px}.row{display:flex;gap:4px;margin-bottom:4px;flex-wrap:wrap}</style>${body}`;
mkdirSync(dirname(out), { recursive: true });
const page = out.replace(/\.png$/i, ".html");
writeFileSync(page, html);
const browser = await chromium.launch();
try {
  const tab = await browser.newPage({ viewport: { width: 1500, height: 900 } });
  await tab.goto(pathToFileURL(page).href);
  await tab.screenshot({ path: out, fullPage: true });
} finally {
  await browser.close();
}
console.log(`wrote ${out}`);
