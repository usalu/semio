/** 🧾️ wgpu DEFERRED-ACTION COMMIT probe — does an action a frame's input phase took out of the
 * bounded input authority still reach the guest when that frame is superseded?
 *
 * Boots `?plugin=generation3d&mode=generate` on the coordinator's serve, presses the retained
 * `Add Generation` row so the Form has real controls, then drives TWO commits that both cross the
 * frame's deferred-action queue: the inline rename editor's `Trigger::Commit` (`renameGeneration`)
 * and a Form slider DRAG (`updateGenerationValues`, one commit per intermediate value). For each it
 * reports, from the console alone, the three hops that matter — the action leaving the input
 * authority (`frame input action`), the completing frame installing it (`frame deferred install
 * carries an action`), and the guest admitting it (`plugin_exchange actionId=…`, armed through
 * `SEMIO_RUNTIME_DIAGNOSTICS`) — plus the published node text and the preview's world3d census
 * before and after, which is what a user actually sees.
 *
 * Every rect comes from the two published sources only: the node's own `dumpStructure` rect plus its
 * window origin in the shell's `dock plan` trace. Never a guessed pixel.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-deferred/commit-1 bun 🐍️wgpu-deferred-commit-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const origin = process.env.SEMIO_PROBE_ORIGIN ?? "http://127.0.0.1:6118";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-deferred/commit-1");
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "hexagonal-mushroom-column";
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 45);
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {}
});
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
      for (const id of root.windowIds ?? []) {
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

/** 🎯️ One published node's page rect, from its own rect plus its window's dock origin. */
const locate = (snapshot, plan, needle) => {
  for (const id of snapshot?.windowIds ?? []) {
    const body = plan[id];
    if (!body) continue;
    for (const node of snapshot.per[id]?.nodes ?? []) {
      const path = String(node.path ?? "");
      if (!path.includes(needle)) continue;
      const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
      if (rw <= 0 || rh <= 0) continue;
      return { windowId: id, path, kind: node.kind ?? null, text: node.text ?? null, rect: [body.x + rx, body.y + ry, rw, rh], page: [body.x + rx + rw / 2, body.y + ry + rh / 2] };
    }
  }
  return null;
};

/** 🎯️ Jiggles at one point until the shell's own `pointer hit` trace resolves `needle` there.
 *
 * 🩸️ The retained hit registry is DRAINED one entry per frame-build boundary step and rebuilt by the
 * chrome walk, so consecutive pointer samples at the SAME point alternate `targets=40 hit=Some(..)`
 * and `targets=0 hit=None`. A press issued blind lands on the empty half about half the time, the
 * shell routes no retained press at all, and a `PointerDown`-seeded gesture (Slider, Ring) never
 * seats its capture — measured in `🗑️generated/wgpu-deferred/commit-1`. */
const armHit = async (x, y, needle, tries = 60) => {
  for (let attempt = 0; attempt < tries; attempt += 1) {
    await page.mouse.move(x + (attempt % 2 === 0 ? -0.25 : 0.25), y);
    await page.waitForTimeout(110);
    const last = has("os_host pointer hit").at(-1) ?? "";
    if (last.includes("hit=Some(") && last.includes(needle)) return true;
  }
  return false;
};

const press = async (needle, settle = 8) => {
  await pump(2);
  const target = locate(await dumpAll(), dockPlan(), needle);
  if (!target) return null;
  await page.mouse.move(target.page[0], target.page[1]);
  await page.waitForTimeout(300);
  await page.mouse.click(target.page[0], target.page[1]);
  await pump(settle);
  return target;
};

/** 🖼️ The pixels of one docked pane, so "the preview changed" is what a user sees and not a count.
 * `state-meshes`/`instances` do not move when a generation's HEIGHT does — the same three meshes are
 * re-tessellated — so a census alone can never witness this commit. */
const paneShot = async (windowId) => {
  const body = dockPlan()[windowId];
  if (!body) return null;
  const clip = { x: Math.max(0, Math.round(body.x)), y: Math.max(0, Math.round(body.y)), width: Math.round(body.w), height: Math.round(body.h) };
  return await page.screenshot({ clip }).catch(() => null);
};

/** 🔬️ How many bytes of two same-sized pane shots differ, as a fraction of the smaller one. */
const paneDelta = (before, after) => {
  if (!before || !after) return null;
  const length = Math.min(before.length, after.length);
  let differing = 0;
  for (let index = 0; index < length; index += 1) if (before[index] !== after[index]) differing += 1;
  return { bytes: length, differing, fraction: Number((differing / Math.max(1, length)).toFixed(4)), sameLength: before.length === after.length };
};

const world3d = () => {
  const byId = {};
  for (const line of has("world3d surface=")) {
    const surface = /world3d surface=(\S+)/.exec(line)?.[1];
    if (!surface) continue;
    byId[surface] = { instances: Number(/ instances=(\d+)/.exec(line)?.[1] ?? 0), lines: Number(/ lines=(\d+)/.exec(line)?.[1] ?? 0), stateMeshes: Number(/ state-meshes=(\d+)/.exec(line)?.[1] ?? 0), stateDraws: Number(/ state-draws=(\d+)/.exec(line)?.[1] ?? 0) };
  }
  return byId;
};

/** 📜️ Every hop of one commit, counted only in the console slice the gesture produced. */
const hops = (from, action) => {
  const slice = lines.slice(from);
  const grab = (needle) => slice.filter((line) => line.includes(needle));
  return {
    leftTheInputAuthority: grab(`frame input action`).filter((line) => line.includes(`action=${action}`)).length,
    installedByACompletingFrame: grab("frame deferred install carries an action").length,
    admittedByTheGuest: grab(`plugin_exchange actionId=${action}`).length,
    guestRanTheCommand: grab(`command action=${action}`).length,
    dispatchFailed: grab("frame deferred action failed").length,
    frameFaults: grab("frame fault recorded").slice(0, 10),
    supersessions: grab("frame build superseded").length,
  };
};

const url = `${origin}/?plugin=generation3d&mode=generate&example=${encodeURIComponent(example)}`;
await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 400)}`));
await pump(settleSeconds);

const verdicts = { example, rename: null, slider: null };

// 🧬️ A generation first: without one the Form has a placeholder and no control at all.
const added = await press("procedural3d-play-generate.add-generation", 12);
verdicts.addGeneration = Boolean(added);
const seeded = await dumpAll();
verdicts.formNodes = (seeded?.per?.["generation3d-generate-form"]?.nodes ?? []).map((node) => ({ path: node.path, kind: node.kind, text: node.text }));

// ✏️ Commit 1 — the inline rename editor's `Trigger::Commit`.
{
  const editor = await press("generation.generation-1.rename", 4);
  const mark = lines.length;
  let typedText = null;
  if (editor) {
    typedText = "Balcony";
    await page.keyboard.press("Control+A").catch(() => {});
    await page.keyboard.type(typedText, { delay: 30 });
    await page.keyboard.press("Enter");
    await pump(14);
  }
  const after = await dumpAll();
  const shown = (after?.per?.["generation3d-generations"]?.nodes ?? []).find((node) => String(node.path ?? "").endsWith(".rename"))?.text ?? null;
  verdicts.rename = { editorPressed: Boolean(editor), before: editor?.text ?? null, typed: typedText, shown, committed: Boolean(shown && shown !== editor?.text), hops: hops(mark, "renameGeneration") };
}

// 🎚️ Commit 2 — a Form slider DRAG, which commits every intermediate value.
{
  const before = await dumpAll();
  const slider = locate(before, dockPlan(), "generate.form.height.slider");
  const censusBefore = world3d();
  // 🎛️ A CONTROL first: two shots of the same idle pane, so "the preview changed" is measured
  // against what an untouched preview does rather than against zero.
  const previewIdleA = await paneShot("generation3d-generate-preview");
  await pump(4);
  const previewIdleB = await paneShot("generation3d-generate-preview");
  const previewBefore = previewIdleB;
  const mark = lines.length;
  let dragged = null;
  let armed = false;
  let seated = 0;
  if (slider) {
    const [x, y, w, h] = slider.rect;
    const from = [x + w * 0.25, y + h / 2];
    const to = [x + w * 0.8, y + h / 2];
    for (let attempt = 0; attempt < 6 && seated === 0; attempt += 1) {
      armed = await armHit(from[0], from[1], "generate.form.height.slider");
      await page.mouse.move(from[0], from[1]);
      await page.mouse.down();
      for (let step = 1; step <= 6; step += 1) {
        await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 6, to[1]);
        await page.waitForTimeout(90);
      }
      await page.mouse.up();
      await pump(6);
      seated = has("retained press").filter((line) => line.includes("kind=Slider") && line.includes("down=true")).length;
    }
    dragged = { from, to, armed, seatedPresses: seated };
    await pump(16);
  }
  const previewAfter = await paneShot("generation3d-generate-preview");
  if (previewIdleA) writeFileSync(join(outDir, "preview-idle-a.png"), previewIdleA);
  if (previewIdleB) writeFileSync(join(outDir, "preview-before.png"), previewIdleB);
  if (previewAfter) writeFileSync(join(outDir, "preview-after.png"), previewAfter);
  let after = await dumpAll();
  for (let retry = 0; retry < 4 && !after?.per?.["generation3d-generate-form"]; retry += 1) {
    await pump(2);
    after = await dumpAll();
  }
  const censusAfter = world3d();
  verdicts.slider = {
    previewPixelsIdleControl: paneDelta(previewIdleA, previewIdleB),
    previewPixels: paneDelta(previewBefore, previewAfter),
    found: Boolean(slider),
    path: slider?.path ?? null,
    kind: slider?.kind ?? null,
    dragged,
    fieldBefore: (before?.per?.["generation3d-generate-form"]?.nodes ?? []).find((node) => String(node.path ?? "").endsWith("generate.form.height"))?.text ?? null,
    fieldAfter: (after?.per?.["generation3d-generate-form"]?.nodes ?? []).find((node) => String(node.path ?? "").endsWith("generate.form.height"))?.text ?? null,
    censusBefore,
    censusAfter,
    censusChanged: JSON.stringify(censusBefore) !== JSON.stringify(censusAfter),
    hops: hops(mark, "updateGenerationValues"),
  };
  verdicts.structureAfter = after;
}

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "verdicts.json"), JSON.stringify(verdicts, null, 2));
await page.screenshot({ path: join(outDir, "final.png") }).catch(() => {});
await browser.close();
const brief = { addGeneration: verdicts.addGeneration, rename: { committed: verdicts.rename?.committed, shown: verdicts.rename?.shown, ...verdicts.rename?.hops }, slider: { found: verdicts.slider?.found, seated: verdicts.slider?.dragged?.seatedPresses, idleControl: verdicts.slider?.previewPixelsIdleControl, previewPixels: verdicts.slider?.previewPixels, ...verdicts.slider?.hops } };
console.log(`[DEBUG] deferred-commit ${JSON.stringify(brief)}`);
