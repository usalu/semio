/** 🧬️ GENERATION-PUBLICATION probe — boots 6118 straight into generate mode, presses the retained
 * `Add Generation` row at its OWN published rect (`dumpStructure` body rect + the shell's
 * `[DEBUG] wgpu-shell dock plan` window origin, the addressing `🐍️wgpu-hit-probe.mjs` proved), and then
 * reads, hop by hop, what the press produced: the retained press, the typed-operation command, the
 * guest's own publication trace, the re-rendered surfaces, and finally the three generate windows'
 * retained structure — the roster the user sees.
 *
 * A hop is never inferred from a screenshot: every verdict below is a console line or a published
 * retained node. Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-generation/run-1 bun 🐍️wgpu-generation-publication-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const origin = process.env.SEMIO_PROBE_ORIGIN ?? "http://127.0.0.1:6118";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-generation/run-1");
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "hexagonal-mushroom-column";
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 45);
const afterSeconds = Number(process.env.SEMIO_PROBE_AFTER ?? 40);
const rowNeedle = process.env.SEMIO_PROBE_ROW ?? "add-generation";
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
if (process.env.SEMIO_PROBE_GUEST_DIAGNOSTICS === "1") await page.addInitScript(() => { try { globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));
const has = (needle) => lines.filter((line) => line.includes(needle));

const dumpAll = () =>
  page
    .evaluate(async () => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return null;
      const root = JSON.parse(await beacon.dumpStructure());
      const per = {};
      const stats = {};
      const panels = ["framework.panel.history", "framework.panel.artifact", "framework.panel.inspection"];
      for (const id of [...(root.windowIds ?? []), ...panels]) {
        try {
          per[id] = JSON.parse(await beacon.dumpStructure(id));
        } catch {
          per[id] = null;
        }
        try {
          stats[id] = JSON.parse(await beacon.dumpFrameStats(id));
        } catch {
          stats[id] = null;
        }
      }
      return { windowIds: root.windowIds ?? [], per, stats };
    })
    .catch(() => null);

const dockPlan = () => {
  const line = has("wgpu-shell dock plan").at(-1);
  const plan = {};
  if (!line) return plan;
  for (const token of line.split(" ").slice(1)) {
    const match = /^(.+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)$/.exec(token);
    if (match) plan[match[1]] = { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) };
  }
  return plan;
};

const pump = async (seconds) => {
  for (let tick = 0; tick < seconds * 5; tick += 1) {
    await page.waitForTimeout(200);
    await page.mouse.move(3 + (tick % 2), 3).catch(() => {});
  }
};

/** 🗂️ The roster the user reads: every generations-window node under the `generations` section. */
const roster = (snapshot) => (snapshot?.per?.["generation3d-generations"]?.nodes ?? []).map((node) => String(node.path ?? "")).filter((path) => path.includes("generations"));
const formNodes = (snapshot) => (snapshot?.per?.["generation3d-generate-form"]?.nodes ?? []).map((node) => ({ path: node.path, text: node.text, kind: node.kind }));

const world3dTraces = () => {
  const byId = {};
  for (const line of has("world3d surface=")) {
    const surface = /world3d surface=(\S+)/.exec(line)?.[1];
    if (!surface) continue;
    byId[surface] = { instances: Number(/ instances=(\d+)/.exec(line)?.[1] ?? 0), lines: Number(/ lines=(\d+)/.exec(line)?.[1] ?? 0), stateMeshes: Number(/ state-meshes=(\d+)/.exec(line)?.[1] ?? 0) };
  }
  return byId;
};

const scenePasses = (snapshot) => Math.max(0, ...Object.values(snapshot?.stats ?? {}).map((stat) => stat?.scenePasses ?? 0));

/** 🎯️ The page rect of the first retained node whose published path contains `needle`, from the two
 * published sources only: the body's own `mounted_layout` rect plus its window's origin in the
 * shell's `dock plan` trace. Never a guessed pixel. */
const locate = (snapshot, plan, needle) => {
  for (const id of snapshot?.windowIds ?? []) {
    const body = plan[id];
    if (!body) continue;
    for (const node of snapshot.per[id]?.nodes ?? []) {
      const path = String(node.path ?? "");
      if (!path.includes(needle)) continue;
      const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
      if (rw <= 0 || rh <= 0) continue;
      return { windowId: id, path, text: node.text ?? null, page: [body.x + rx + rw / 2, body.y + ry + rh / 2] };
    }
  }
  return null;
};

/** 🎯️ Settles the pointer on one point and reports what the shell's own `pointer hit` trace resolved
 * there — an OBSERVATION, not an arming loop.
 *
 * 🩸️ This used to JIGGLE 0.25 px up to 60 times until the trace answered the row, because the retained
 * hit registry was one vector the frame build drained while `hit_at` scanned it: consecutive samples at
 * the SAME point alternated `targets=42 hit=Some(TreeItem, "…add-generation")` and `targets=0 hit=None`.
 * `📓️wgpu-hit-registry-drain-2026-09-14.md` made the registry a retained authority (the last COMPLETE
 * frame stays resolvable while a build retires and re-mints a second buffer), so a user's single move is
 * enough again — and a probe that kept arming would hide the next regression of exactly this defect. */
const settleHit = async (x, y, needle, tries = 20) => {
  await page.mouse.move(x, y);
  for (let attempt = 0; attempt < tries; attempt += 1) {
    await page.waitForTimeout(110);
    const last = has("os_host pointer hit").at(-1) ?? "";
    if (last.includes("hit=Some(") && last.includes(needle)) return attempt + 1;
  }
  return 0;
};

/** 🖱️ Presses one published node and lets the shell settle. Answers the target it pressed. */
const press = async (needle, settle = 6) => {
  const plan = dockPlan();
  const target = locate(await dumpAll(), plan, needle);
  if (!target) return null;
  await pump(2);
  const armed = await settleHit(target.page[0], target.page[1], needle.split("/").at(-1) ?? needle);
  lines.push(`${at()} PROBE settled ${needle} samples=${armed}`);
  await page.mouse.click(target.page[0], target.page[1]);
  await pump(settle);
  return { ...target, armed };
};

/** 🗂️ The roster the way a user reads it: one entry per generation row, with the name its inline
 * editor shows when that row is the selected one. */
const rosterRows = (snapshot) =>
  (snapshot?.per?.["generation3d-generations"]?.nodes ?? [])
    .map((node) => String(node.path ?? ""))
    .filter((path) => /generation\.generation-\d+$/u.test(path))
    .map((path) => path.slice(path.lastIndexOf(".generation.") + ".generation.".length));

const renameEditor = (snapshot) => (snapshot?.per?.["generation3d-generations"]?.nodes ?? []).find((node) => String(node.path ?? "").endsWith(".rename")) ?? null;

const url = `${origin}/?plugin=generation3d&mode=generate&example=${encodeURIComponent(example)}`;
await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 400)}`));
await pump(settleSeconds);

const before = await dumpAll();
const plan = dockPlan();
lines.push(`${at()} PROBE dockPlan ${JSON.stringify(plan)}`);
lines.push(`${at()} PROBE rosterBefore ${JSON.stringify(roster(before))}`);

const target = (() => {
  for (const id of before?.windowIds ?? []) {
    const body = plan[id];
    if (!body) continue;
    for (const node of before.per[id]?.nodes ?? []) {
      const path = String(node.path ?? "");
      if (!path.includes(rowNeedle)) continue;
      const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
      if (rw <= 0 || rh <= 0) continue;
      return { windowId: id, path, rect: node.rect, page: [body.x + rx + rw / 2, body.y + ry + rh / 2] };
    }
  }
  return null;
})();
lines.push(`${at()} PROBE target ${JSON.stringify(target)}`);

const mark = lines.length;
let clicked = null;
if (target) {
  await page.mouse.move(target.page[0], target.page[1]);
  await page.waitForTimeout(300);
  await page.mouse.click(target.page[0], target.page[1]);
  clicked = { point: target.page };
  lines.push(`${at()} PROBE click ${JSON.stringify(clicked)}`);
}
await pump(afterSeconds);

const after = await dumpAll();
lines.push(`${at()} PROBE rosterAfter ${JSON.stringify(roster(after))}`);

/** 🧬️ The roster journey a user performs next: a SECOND generation, then `selectGeneration` back onto
 * the first row, then `renameGeneration` typed into that row's own inline editor. Every step is
 * driven through the published retained node, never a synthetic dispatch. */
const journey = { ran: false };
if (process.env.SEMIO_PROBE_JOURNEY === "1" && roster(after).length > roster(before).length) {
  journey.ran = true;
  journey.rosterAfterFirstAdd = rosterRows(after);
  journey.selectedAfterFirstAdd = renameEditor(after)?.path ?? null;

  await press("procedural3d-play-generate.add-generation", 10);
  const twoRows = await dumpAll();
  journey.rosterAfterSecondAdd = rosterRows(twoRows);
  journey.selectedAfterSecondAdd = renameEditor(twoRows)?.path ?? null;

  const firstRow = journey.rosterAfterFirstAdd[0];
  journey.selectPressed = Boolean(await press(`generation.${firstRow}`, 10));
  const selected = await dumpAll();
  journey.selectedAfterSelect = renameEditor(selected)?.path ?? null;
  journey.selectMovedTheEditor = Boolean(journey.selectedAfterSelect?.includes(firstRow));

  const editor = await press(`generation.${firstRow}.rename`, 4);
  journey.renameEditorPressed = Boolean(editor);
  journey.renameBefore = editor?.text ?? null;
  if (editor) {
    await page.keyboard.press("Control+A").catch(() => {});
    await page.keyboard.type("Balcony Study", { delay: 30 });
    await page.keyboard.press("Enter");
    await pump(12);
  }
  const renamed = await dumpAll();
  journey.renameAfter = renameEditor(renamed)?.text ?? null;
  journey.renamed = journey.renameAfter === "Balcony Study";
  journey.rosterAfterRename = rosterRows(renamed);
  lines.push(`${at()} PROBE journey ${JSON.stringify(journey)}`);
}

const sincePress = lines.slice(mark);
const grab = (needle) => sincePress.filter((line) => line.includes(needle)).slice(0, 40);
const verdicts = {
  retainedPress: grab("retained press").length,
  typedOperation: grab("typed-operation").length,
  renderSurface: grab("renderSurface").length,
  faults: sincePress.filter((line) => /fault|error|panic|unreachable/i.test(line)).slice(0, 40),
  rosterBefore: roster(before),
  rosterAfter: roster(after),
  rosterGrew: roster(after).length > roster(before).length,
  formBefore: formNodes(before),
  formAfter: formNodes(after),
  scenePassesAfter: scenePasses(after),
  world3d: world3dTraces(),
  selected: grab("addGeneration").slice(0, 20),
  journey,
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "since-press.txt"), sincePress.join("\n"));
writeFileSync(join(outDir, "verdicts.json"), JSON.stringify(verdicts, null, 2));
writeFileSync(join(outDir, "structure-after.json"), JSON.stringify(after, null, 2));
await page.screenshot({ path: join(outDir, "final.png") }).catch(() => {});
await browser.close();
console.log(`[DEBUG] generation-publication target=${target ? "yes" : "no"} rosterBefore=${verdicts.rosterBefore.length} rosterAfter=${verdicts.rosterAfter.length} grew=${verdicts.rosterGrew} formAfter=${verdicts.formAfter.length} scenePasses=${verdicts.scenePassesAfter} world3d=${JSON.stringify(verdicts.world3d)} journey=${JSON.stringify(journey)}`);
