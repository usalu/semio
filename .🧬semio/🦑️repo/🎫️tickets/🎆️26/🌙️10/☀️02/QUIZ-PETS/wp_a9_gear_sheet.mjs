/** 📸️ Ticket tool (work package A9): photographs the product's own gear drawings — parachute, rope and hook, grappling gun, carried and standing ladders —, the tilt of a drawing about its pivot, state tints and particles, in all their parameter ranges, on the light and the dark theme, next to real pets.
 *
 * It bundles `wp_a9_gear_sheet.entry.ts` (which imports the modules of the React target) with `bun build`, writes a page
 * that loads the product's stylesheet, and takes one PNG at the size pets have on screen (×1) and one enlarged.
 *
 * Usage (from the repository root):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_a9_gear_sheet.mjs" [--out <directory>] [--zooms 1,3] [--species <a.json>,<b.json>]
 *
 * The first species is the small one (default: the reference rig, 40 × 30), the second the large one (default: housy,
 * 48 × 52); a third is made from the first with a canopy of its own.
 */
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
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
const out = resolve(option("out", join(here, "🗑️generated", "a9")));
const zooms = option("zooms", "1,3").split(",").map(Number);
const files = option("species", [join(here, "reference-species.json"), join(root, "🎓️teaching/🏛️architecture/🐾️pets/🏠️housy/🔣️.json")].join(",")).split(",");

const species = files.map((file) => {
  const document = JSON.parse(readFileSync(file, "utf8"));
  return { ...document, grip: document.grip ?? Math.round(document.size.height * 0.9) };
});
const span = species[0].size.width * 0.75;
species.push({ ...species[0], id: `${species[0].id}-canopy`, canopy: { kind: "path", d: `M ${-span} 0 Q ${-span} -20 0 -24 Q ${span} -20 ${span} 0 L ${span / 3} -5 L 0 0 L ${-span / 3} -5 Z` } });

mkdirSync(out, { recursive: true });
const bundle = join(out, "gear-sheet.js");
const built = spawnSync("bun", ["build", join(here, "wp_a9_gear_sheet.entry.ts"), "--outfile", bundle, "--target", "browser"], { cwd: root, encoding: "utf8", shell: process.platform === "win32" });
if (built.status !== 0) throw new Error(`bun build failed: ${built.stdout}\n${built.stderr}`);
const css = readFileSync(join(root, "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🎨️.css"), "utf8");

const page = (zoom) => `<!doctype html><meta charset="utf-8"><style>${css}</style><style>
body{margin:8px;background:#888;font:11px monospace}
.sheet{padding:6px 8px;margin-bottom:8px}
.sheet.light{background:#f7f3e3;color:#001117;--foreground:#001117}
.sheet.dark{background:#001117;color:#f7f3e3;--foreground:#f7f3e3}
h2{margin:8px 0 4px;font-size:11px;font-weight:normal;opacity:.8}
.row{display:flex;gap:6px;flex-wrap:wrap;align-items:flex-end}
.cell{position:relative;margin:0;overflow:hidden;outline:1px dashed color-mix(in srgb,currentColor 25%,transparent)}
.ground{position:absolute;left:0;right:0;height:1px;background:currentColor;opacity:.35}
figcaption{position:absolute;left:2px;right:2px;bottom:1px;font-size:9px;line-height:1.1;opacity:.85}
i{position:absolute;width:4px;height:4px;margin:-2px 0 0 -2px;border-radius:50%}
i.pivot{background:#ff344f}
i.mark{background:#8282dd}
</style><body><script>const SHEET=${JSON.stringify({ species, zoom, themes: ["light", "dark"] })};</script><script src="./gear-sheet.js"></script>`;

const browser = await chromium.launch();
try {
  for (const zoom of zooms) {
    const file = join(out, `gear-sheet-x${zoom}.html`);
    writeFileSync(file, page(zoom));
    const tab = await browser.newPage({ viewport: { width: zoom === 1 ? 1500 : 1640, height: 900 }, deviceScaleFactor: 1 });
    const faults = [];
    tab.on("pageerror", (error) => faults.push(String(error)));
    tab.on("console", (message) => {
      if (message.type() === "error") faults.push(message.text());
    });
    await tab.goto(pathToFileURL(file).href);
    await tab.waitForSelector("body[data-ready]", { timeout: 10000 }).catch(() => faults.push("the sheet never became ready"));
    if (faults.length > 0) throw new Error(faults.join("\n"));
    for (const theme of ["light", "dark"]) {
      const shot = join(out, `gear-sheet-x${zoom}-${theme}.png`);
      await tab.locator(`section.sheet.${theme}`).screenshot({ path: shot });
      process.stdout.write(`wrote ${shot}\n`);
      if (zoom === 1) continue;
      const bands = tab.locator(`section.sheet.${theme} .band`);
      for (let band = 0; band < (await bands.count()); band++) {
        const strip = join(out, `gear-sheet-x${zoom}-${theme}-row${band + 1}.png`);
        await bands.nth(band).screenshot({ path: strip });
        process.stdout.write(`wrote ${strip}\n`);
      }
    }
    await tab.close();
  }
} finally {
  await browser.close();
}
