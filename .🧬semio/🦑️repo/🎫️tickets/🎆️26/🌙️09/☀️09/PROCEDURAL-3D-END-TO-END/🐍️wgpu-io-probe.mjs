/** 🚪️ wgpu IO SURFACE — can a user EXPORT and IMPORT a generation3d document on the wgpu renderer?
 *
 * The wgpu twin of `🐍️io-surface-probe.mjs` (React, 6018). The two verbs are declared by the artifact
 * itself — `exportDocument` (`mod+shift+e`, one required `format` arg) and `importDocumentRequest`
 * (`mod+o`) — so this probe drives them the way a user reaches them on a canvas renderer that has no
 * DOM menu: by their own CHORDS, and then through whatever retained form the shell stages for the
 * required argument, addressed at its published rect (`dumpStructure` `mounted_layout` + the window's
 * own `[DEBUG] wgpu-shell dock plan` origin) exactly as `🐍️wgpu-deferred-commit-probe.mjs` proved.
 *
 * The hops it reports, each a console line or a browser event — never an impression:
 *   • `frame input action … action=exportDocument` — the verb left the input authority;
 *   • `wgpu-shell command … settled` / `plugin_exchange actionId=…` — the guest admitted it;
 *   • `gen3d command action=exportDocument` — the guest RAN it;
 *   • a `download` browser event, or the segmented-download sink's own trace — the bytes crossing.
 *
 * A headless browser can refuse to present a download; that is recorded separately from the crossing,
 * so "no file on disk" is never confused with "the verb never ran".
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-verify/io bun 🐍️wgpu-io-probe.mjs
 * @see 🐍️io-surface-probe.mjs, 📓️react-end-to-end-verification-2026-09-13.md §3
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&example=hexagonal-mushroom-column";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-verify/io");
const bootBudget = Number(process.env.SEMIO_PROBE_BOOT ?? 120);
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 20);
mkdirSync(outDir, { recursive: true });

/** 🔺️ One real ASCII STL triangle, so the import picker is answered with a genuine file. */
const STL = ["solid probe", "facet normal 0 0 1", "  outer loop", "    vertex 0 0 0", "    vertex 1 0 0", "    vertex 0 1 0", "  endloop", "endfacet", "endsolid probe", ""].join("\n");
const stlPath = join(outDir, "probe-triangle.stl");
writeFileSync(stlPath, STL);

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const downloads = [];
const fileChoosers = [];

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {}
  // 👁️ The USER-FACING witness. A headless Chromium may refuse to present a save dialog, so "no file on
  // disk" would otherwise be indistinguishable from "no bytes were ever produced" — which is exactly the
  // confusion this journey spent a ticket in. The page's own `<a download>` click is wrapped and the blob
  // behind its `href` read back (magic + size), and the `<input type=file>` click is wrapped too, so a
  // picker the page opens is witnessed even where no `filechooser` event fires.
  const witness = { anchors: [], inputs: [], objectUrls: 0 };
  globalThis.__semioIoWitness = witness;
  const blobByUrl = new Map();
  const createObjectUrl = URL.createObjectURL.bind(URL);
  URL.createObjectURL = (object) => {
    const href = createObjectUrl(object);
    witness.objectUrls += 1;
    blobByUrl.set(href, object);
    return href;
  };
  const anchorClick = HTMLAnchorElement.prototype.click;
  HTMLAnchorElement.prototype.click = function semioWitnessedAnchorClick() {
    const download = this.getAttribute("download");
    if (download !== null) {
      const href = this.getAttribute("href") ?? "";
      const record = { t: Date.now(), download, type: this.getAttribute("type"), bytes: null, hex: null, text: null, error: null };
      witness.anchors.push(record);
      const blob = blobByUrl.get(href);
      if (blob && typeof blob.arrayBuffer === "function") {
        void blob
          .arrayBuffer()
          .then((buffer) => {
            const view = new Uint8Array(buffer);
            record.bytes = view.byteLength;
            record.hex = [...view.slice(0, 8)].map((byte) => byte.toString(16).padStart(2, "0")).join(" ");
            record.text = new TextDecoder("utf-8", { fatal: false }).decode(view.slice(0, 96));
          })
          .catch((error) => {
            record.error = String(error).slice(0, 200);
          });
      } else {
        record.error = "no blob behind href";
      }
      console.log(`[DEBUG] PROBE anchor-download name=${download} type=${this.getAttribute("type")}`);
    }
    return anchorClick.call(this);
  };
  const inputClick = HTMLInputElement.prototype.click;
  HTMLInputElement.prototype.click = function semioWitnessedInputClick() {
    if (this.type === "file") {
      witness.inputs.push({ t: Date.now(), accept: this.accept, multiple: this.multiple });
      console.log(`[DEBUG] PROBE file-input-click accept=${this.accept} multiple=${this.multiple}`);
    }
    return inputClick.call(this);
  };
});
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));
page.on("download", async (download) => {
  const suggested = download.suggestedFilename();
  const target = join(outDir, suggested);
  let bytes = null;
  try {
    await download.saveAs(target);
    bytes = existsSync(target) ? statSync(target).size : null;
  } catch (error) {
    lines.push(`${at()} PROBE download save failed ${String(error).slice(0, 200)}`);
  }
  downloads.push({ t: at(), filename: suggested, bytes, path: target });
  lines.push(`${at()} PROBE download ${suggested} bytes=${bytes}`);
});
page.on("filechooser", async (chooser) => {
  fileChoosers.push({ t: at(), multiple: chooser.isMultiple() });
  lines.push(`${at()} PROBE filechooser opened`);
  await chooser.setFiles(stlPath).catch((error) => lines.push(`${at()} PROBE filechooser setFiles failed ${String(error).slice(0, 200)}`));
});

const has = (needle) => lines.filter((line) => line.includes(needle));
const witness = () => page.evaluate(() => JSON.parse(JSON.stringify(globalThis.__semioIoWitness ?? { anchors: [], inputs: [], objectUrls: 0 }))).catch(() => null);
const pump = async (seconds) => {
  for (let tick = 0; tick < seconds * 5; tick += 1) {
    await page.waitForTimeout(200);
    await page.mouse.move(3 + (tick % 2), 3).catch(() => {});
  }
};

const dumpAll = () =>
  page
    .evaluate(async () => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return null;
      const root = JSON.parse(await beacon.dumpStructure());
      const per = {};
      for (const id of root.windowIds ?? []) {
        try {
          per[id] = JSON.parse(await beacon.dumpStructure(id));
        } catch {
          per[id] = null;
        }
      }
      return { windowIds: root.windowIds ?? [], per };
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

/** 🎯️ Every published node whose path or text matches, with its page rect. */
const locateAll = (snapshot, plan, test) => {
  const found = [];
  for (const id of snapshot?.windowIds ?? []) {
    const body = plan[id];
    if (!body) continue;
    for (const node of snapshot.per[id]?.nodes ?? []) {
      const path = String(node.path ?? "");
      const text = String(node.text ?? "");
      if (!test(path, text)) continue;
      const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
      if (rw <= 0 || rh <= 0) continue;
      found.push({ windowId: id, path, kind: node.kind ?? null, text: node.text ?? null, page: [body.x + rx + rw / 2, body.y + ry + rh / 2], rect: [body.x + rx, body.y + ry, rw, rh] });
    }
  }
  return found;
};

const isMac = process.platform === "darwin";
const mod = isMac ? "Meta" : "Control";

/** ⌨️ A chord pressed on the canvas, which is where the wgpu shell reads keyboard input. */
const chord = async (keys) => {
  await page.mouse.move(720, 500).catch(() => {});
  await page.waitForTimeout(150);
  await page.keyboard.press(keys);
  await page.waitForTimeout(200);
};

const report = { url, startedAt: new Date().toISOString(), steps: [] };
const note = async (step, detail) => {
  report.steps.push({ step, t: at(), ...detail });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1400)}`);
  writeFileSync(join(outDir, "results.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
};

/** 📜️ Every hop one verb crossed, counted only in the console slice the gesture produced. */
const hops = (from, action) => {
  const slice = lines.slice(from);
  const hit = (needle) => slice.filter((line) => line.includes(needle)).length;
  return {
    leftTheInputAuthority: slice.filter((line) => line.includes("frame input action") && line.includes(`action=${action}`)).length,
    dispatched: slice.filter((line) => line.includes("wgpu-shell command") && line.includes(action)).length,
    admittedByTheGuest: hit(`plugin_exchange actionId=${action}`),
    guestRanTheCommand: hit(`gen3d command action=${action}`),
    interactiveJob: hit("interactive-job") + hit("interactiveJob"),
    segmentedDownload: hit("segmented-download") + hit("segmentedDownload"),
    dispatchFailed: hit("frame deferred action failed") + hit("dispatch failed"),
    keyDown: hit("handle_event KeyDown"),
    sample: slice.filter((line) => line.includes(action)).slice(0, 6).map((line) => line.slice(0, 400)),
  };
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 400)}`));

let booted = null;
for (let tick = 0; tick < bootBudget * 2; tick += 1) {
  await page.mouse.move(3 + (tick % 3), 3).catch(() => {});
  await page.waitForTimeout(500);
  if (has("boot_shell leave").length) {
    booted = at();
    break;
  }
}
await note("boot", { bootShellLeaveMs: booted, windowIds: (await dumpAll())?.windowIds ?? [] });
if (booted === null) {
  writeFileSync(join(outDir, "results.json"), JSON.stringify({ ...report, fatal: "never booted" }, null, 2));
  await browser.close();
  process.exit(1);
}
await pump(settleSeconds);

// ── EXPORT ───────────────────────────────────────────────────────────────────────────────────────
{
  const from = lines.length;
  const before = await dumpAll();
  await chord(`${mod}+Shift+E`);
  await pump(10);
  const after = await dumpAll();
  const plan = dockPlan();
  const beforeNodes = new Set((before?.windowIds ?? []).flatMap((id) => (before.per[id]?.nodes ?? []).map((n) => `${id}:${n.path}`)));
  const staged = locateAll(after, plan, (path, text) => /format|export|stl|dwg|confirm|submit|run|apply/i.test(`${path} ${text}`)).filter((node) => !beforeNodes.has(`${node.windowId}:${node.path}`));
  await note("export:chord", {
    hops: hops(from, "exportDocument"),
    stagedNodes: staged.map((n) => ({ path: n.path, kind: n.kind, text: n.text, page: n.page })).slice(0, 20),
    newNodeCount: (after?.windowIds ?? []).reduce((sum, id) => sum + (after.per[id]?.nodes ?? []).filter((n) => !beforeNodes.has(`${id}:${n.path}`)).length, 0),
    downloads: [...downloads],
  });

  // ⌨️ `exportDocument` declares a required `format` arg, so `dispatch_app_keybinding`'s P4 rule makes
  // the FIRST press expand that action's panel and the SECOND — `already_expanded` — execute it with
  // the staged/validated args. A cold keystroke never silent-fires a default, so one press proves
  // nothing about the verb; the pair is the user's actual route.
  const second = lines.length;
  await chord(`${mod}+Shift+E`);
  await pump(25);
  await note("export:second-chord", { hops: hops(second, "exportDocument"), downloads: [...downloads], witness: await witness(), effectDrops: has('unmapped effect "download-media-export"').length });
}

// ── IMPORT ───────────────────────────────────────────────────────────────────────────────────────
{
  const from = lines.length;
  const graphBefore = has("wgpu node-graph geometry surface=").at(-1) ?? null;
  await chord(`${mod}+O`);
  await pump(20);
  const graphAfter = has("wgpu node-graph geometry surface=").at(-1) ?? null;
  await note("import:chord", {
    hops: { request: hops(from, "importDocumentRequest"), chunk: hops(from, "importDocument") },
    fileChoosers: [...fileChoosers],
    witness: await witness(),
    graphChanged: graphBefore !== graphAfter,
    graphNodesBefore: (graphBefore?.match(/"id"/g) ?? []).length,
    graphNodesAfter: (graphAfter?.match(/"id"/g) ?? []).length,
    // 🧬️ The import fixture's OWN three widgets, read off the node-graph census the shell publishes —
    // the same `imported-source`/`imported-geometry`/`imported-preview` graph React's import produces.
    importedNodes: ["imported-source", "imported-geometry", "imported-preview"].filter((id) => String(graphAfter ?? "").includes(`"${id}"`)),
  });
}

await page.screenshot({ path: join(outDir, "final.png") }).catch(() => {});
report.finishedAt = new Date().toISOString();
report.seconds = Math.round(at() / 1000);
report.downloads = downloads;
report.fileChoosers = fileChoosers;
report.witness = await witness();
report.counts = Object.fromEntries(["exportDocument", "importDocument", "importDocumentRequest", "frame input action", "plugin_exchange actionId=", "gen3d command action=", "handle_event KeyDown", "panicked", "pageerror"].map((needle) => [needle, has(needle).length]));
writeFileSync(join(outDir, "results.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] io DONE ${JSON.stringify(report.counts)} downloads=${downloads.length} choosers=${fileChoosers.length}`);
await browser.close();
