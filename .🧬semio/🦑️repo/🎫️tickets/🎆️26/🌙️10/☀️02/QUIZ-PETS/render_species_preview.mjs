/** 🖼️ Ticket tool: renders species documents to PNG contact sheets so rigs, clips, states, tricks and gear can be judged by eye, on the light and the dark theme.
 *
 * ## Usage (from the repository root)
 *
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/render_species_preview.mjs" --out <file.png> [options] <species.json>...
 *
 * | Option | Meaning |
 * |---|---|
 * | `--out <file.png>` | where the sheet goes (default `species-preview.png`); the page itself is written beside it as `.html` with its script `.js` |
 * | `--scale <n>` | screen pixels per pet pixel (default 3; 1 shows the pets as large as on a page) |
 * | `--states` | every state: the rest pose under its tint, and its overlay clip at phase 0.25 with the particles of its emitter |
 * | `--tricks` | every trick's clip at four phases with a sample of its emitter's particles (in the tint of the state it starts from), then the purr |
 * | `--gear` | the clips of the new activities with what they are played with: `hang` and `tumble` tilted about the grip and the middle, `glide` under the open parachute (the species' own canopy or the plain one, also opening and overshooting), `aim` with the gun, `reel` on a rope, `climb` and `slide` at a wall (and on a standing ladder for a species that owns one), `mantle` over a corner, `carry` with a ladder (lying and raised), `push` against a block, `dizzy`, `shrug`, `scoot` |
 * | `--clips a,b` | only these clips at four phases (`--clips all`: every clip); as in the first round |
 * | `--themes light,dark` | the grounds to draw on (default both; the plain clips are drawn on the first only) |
 * | `--pages <px>` | the most a PNG is tall, in page pixels (default 1600, so a picture can be looked at without shrinking): a taller sheet is cut between rows into `<file>-1.png`, `<file>-2.png`, …; 0 never cuts |
 * | `--width <px>` | the width of the page (default 1500) |
 *
 * Without `--states`, `--tricks`, `--gear` and `--clips` a sheet holds everything; with any of them only what is named.
 * The rest row (rest, looks, blink, facing left; both themes) heads every species, with a line that says what the
 * document holds and whether the product's validation accepts it.
 *
 * Examples:
 *
 *   node "<this file>" --out "<ticket>/🗑️generated/h1/sunny.png" "🎓️teaching/🏛️architecture/🐾️pets/☀️sunny/🔣️.json"
 *   node "<this file>" --out sheet.png --gear --scale 2 "🎓️teaching/🏛️architecture/🐾️pets/🏠️housy/🔣️.json" "🎓️teaching/🏛️architecture/🐾️pets/🌡️thermy/🔣️.json"
 *   node "<this file>" --out sheet.png --states --tricks "🎓️teaching/🏛️architecture/🐾️pets/☁️cloudy/🔣️.json"
 *   node "<this file>" --out sheet.png --clips hang,tumble --scale 4 "<ticket>/reference-species.json"
 *
 * ## What is drawn, and by what
 *
 * The sheet is drawn by the product itself: this script bundles `render_species_preview.entry.ts` with `bun build` and
 * loads it into a page next to the product's stylesheet. The core samples the clips (`sampleClip`), solves the rigs
 * (`solveRig`) and moves the particles (`particlesOf` of `🔨️modules/✨️effects`); the React target depicts the pets
 * (`depict`, `paint`), their gear (`equip`, `paintTools`, `paintLadders`), their particles (`paintEffects`) and their
 * tints (`tintPalette`). So what a sheet shows is what the layer will show. A clip is shown alone on the rest pose
 * (the stage runs the idle loop underneath).
 *
 * A species document of the first round still renders: what it lacks is filled in for the sheet (no states, tricks,
 * emitters or gear, a grip at nine tenths of its height); its header then counts the fields validation misses.
 *
 * Notes for artists:
 * - `grip` is the height of the scruff above the feet in pixels: the red dot of the `hang` cells, where the parachute
 *   and the rope of `reel` take hold. Whole-drawing tilt turns the pet about it.
 * - `canopy` is drawn in the accent with an ink outline, around the point that floats 0.7 heights above the grip: put
 *   the middle of its rim at (0, 0) and the canopy above it (negative y). The four cords run to the rim: the leftmost
 *   and the rightmost point of the shape and two points between them. Without `canopy` the plain striped dome is used
 *   (1.5 widths wide, 0.45 heights high).
 * - Particles of an emitter are shown at a moment that suits its motion: a burst along its life over the four phases,
 *   a ring one lap on, a stream (fall, rise, drift) once it is full.
 * - The gun sits at 0.42 widths ahead and half the height up; a carried ladder is held 0.45 heights up, behind the pet.
 */
import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { chromium } from "playwright";

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, "../../../../../../..");
const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  if (index < 0) return fallback;
  const [, value] = args.splice(index, 2);
  return value;
};
const flag = (name) => {
  const index = args.indexOf(`--${name}`);
  if (index < 0) return false;
  args.splice(index, 1);
  return true;
};
const out = resolve(option("out", "species-preview.png"));
const scale = Number(option("scale", "3"));
const only = option("clips", "");
const themes = option("themes", "light,dark").split(",");
const pages = Number(option("pages", "1600"));
const width = Number(option("width", "1500"));
const chosen = ["states", "tricks", "gear"].filter((section) => flag(section));
if (only !== "") chosen.unshift("clips");
const sections = ["rest", ...(chosen.length === 0 ? ["clips", "states", "tricks", "gear"] : chosen)];
const files = args;
if (files.length === 0) throw new Error("no species files given");

const species = files.map((file) => {
  const document = JSON.parse(readFileSync(file, "utf8"));
  return { ...document, states: document.states ?? [], tricks: document.tricks ?? [], emitters: document.emitters ?? [], gear: document.gear ?? [], grip: document.grip ?? Math.round(document.size.height * 0.9), repertoire: document.repertoire ?? {} };
});

mkdirSync(dirname(out), { recursive: true });
const stem = out.replace(/\.png$/i, "");
const quoted = (text) => `"${text}"`;
const built = spawnSync(["bun", "build", quoted(join(here, "render_species_preview.entry.ts")), "--outfile", quoted(`${stem}.js`), "--target", "browser"].join(" "), { cwd: root, encoding: "utf8", shell: true });
if (built.status !== 0) throw new Error(`bun build failed:\n${built.stdout}\n${built.stderr}`);
const css = readFileSync(join(root, "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🎨️.css"), "utf8");
const preview = { species, scale, sections, clips: only === "" || only === "all" ? null : only.split(","), themes };

const html = `<!doctype html><meta charset="utf-8"><style>${css}</style><style>
body{margin:8px;background:#888;font:12px monospace}
h2{margin:10px 0 4px;color:#fff;font-size:13px}
h3{margin:10px 0 3px;color:#fff;font-size:12px}
h4{margin:5px 0 2px;color:#fff;font-size:11px;font-weight:normal}
.row{display:flex;gap:4px;margin-bottom:4px;flex-wrap:wrap;align-items:flex-end}
.row.light{--ground:#f7f3e3;--panel:#c9c8bd;--foreground:#001117}
.row.dark{--ground:#001117;--panel:#1d2b2f;--foreground:#f7f3e3}
.cell{position:relative;margin:0;overflow:hidden;background:var(--ground);color:var(--foreground)}
.decor{position:absolute;left:0;top:0}
.decor .panel{fill:var(--panel)}
.decor .box{fill:none;stroke:var(--foreground);stroke-opacity:.18;stroke-dasharray:2 2;stroke-width:.5}
.decor .wall{fill:none;stroke:var(--foreground);stroke-opacity:.6;stroke-width:1.5}
.decor .block{fill:var(--panel);stroke:var(--foreground);stroke-width:1.5}
figcaption{position:absolute;left:${2 * scale}px;right:${scale}px;bottom:${scale}px;font:${Math.max(8, 4.4 * scale)}px/1.15 monospace;max-height:${13 * scale}px;overflow:hidden}
i{position:absolute;width:${Math.max(4, 1.6 * scale)}px;height:${Math.max(4, 1.6 * scale)}px;margin:${-Math.max(2, 0.8 * scale)}px;border-radius:50%;background:#ff344f}
</style><body><script>const PREVIEW=${JSON.stringify(preview).replace(/</g, "\\u003c")};</script><script src="./${stem.split(/[\\/]/).pop()}.js"></script>`;
writeFileSync(`${stem}.html`, html);

const browser = await chromium.launch();
try {
  const tab = await browser.newPage({ viewport: { width, height: 900 } });
  const faults = [];
  tab.on("pageerror", (error) => faults.push(String(error)));
  tab.on("console", (message) => {
    if (message.type() === "error") faults.push(message.text());
  });
  await tab.goto(pathToFileURL(`${stem}.html`).href);
  await tab.waitForSelector("body[data-ready]", { timeout: 20000 }).catch(() => faults.push("the sheet never became ready"));
  if (faults.length > 0) throw new Error(faults.join("\n"));
  const layout = await tab.evaluate(() => ({ height: document.documentElement.scrollHeight, width: document.documentElement.scrollWidth, breaks: [...document.querySelectorAll("h2, h3, .band")].map((block) => Math.floor(block.getBoundingClientRect().top + window.scrollY) - 2) }));
  if (pages <= 0 || layout.height <= pages) {
    await tab.screenshot({ path: out, fullPage: true });
    console.log(`wrote ${out}`);
  } else {
    const cuts = [0];
    for (const [index, top] of layout.breaks.entries()) {
      const next = layout.breaks[index + 1] ?? layout.height;
      if (next - cuts[cuts.length - 1] > pages && top > cuts[cuts.length - 1]) cuts.push(top);
    }
    cuts.push(layout.height);
    for (let part = 0; part + 1 < cuts.length; part++) {
      const file = `${stem}-${part + 1}.png`;
      await tab.screenshot({ path: file, fullPage: true, clip: { x: 0, y: cuts[part], width: layout.width, height: cuts[part + 1] - cuts[part] } });
      console.log(`wrote ${file}`);
    }
  }
} finally {
  await browser.close();
}
