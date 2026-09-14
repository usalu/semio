/** 🚪️ wgpu IO EFFECTS — does `Export Document…` hand the USER bytes, and does `Import Document…` open a
 * real picker and replace the graph, on the wgpu renderer?
 *
 * The difference from `🐍️wgpu-io-probe.mjs` (which proved only that both verbs CROSS to the guest):
 * this probe witnesses the USER-FACING outcome on the page itself, so a headless Chromium that
 * refuses to present a download dialog can still prove the bytes.
 *
 *   • `<a download>` — `HTMLAnchorElement.prototype.click` is wrapped in an init script and the blob
 *     behind `href` is read back: filename, MIME, byte length and the first 8 bytes as hex. An ASCII
 *     STL must begin `73 6f 6c 69 64` (`solid`), a DWG `41 43 31 30 31 35` (`AC1015`).
 *   • Playwright's own `download` event, recorded separately — "no dialog" is never confused with
 *     "no bytes".
 *   • `<input type=file>` — `HTMLInputElement.prototype.click` is wrapped too, so a picker the page
 *     opens is witnessed even if no `filechooser` event fires; `page.on("filechooser")` answers it
 *     with a real ASCII STL triangle.
 *   • the graph after the import — `imported-source`/`imported-geometry`/`imported-preview`.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-io/run bun 🐍️wgpu-io-effects-probe.mjs
 * @see 🐍️wgpu-io-probe.mjs, 📓️wgpu-end-to-end-verification-2026-09-14.md §C
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&example=hexagonal-mushroom-column";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-io/run");
const bootBudget = Number(process.env.SEMIO_PROBE_BOOT ?? 120);
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 60);
const skipImport = process.env.SEMIO_PROBE_SKIP_IMPORT === "1";
const skipExport = process.env.SEMIO_PROBE_SKIP_EXPORT === "1";
mkdirSync(outDir, { recursive: true });

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
      const record = { t: Date.now(), download, type: this.getAttribute("type"), href: href.slice(0, 64), bytes: null, hex: null, text: null, error: null };
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
      console.log(`[DEBUG] PROBE anchor-download name=${download} type=${this.getAttribute("type")} hrefKind=${href.slice(0, 5)}`);
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
const pump = async (seconds) => {
  for (let tick = 0; tick < seconds * 5; tick += 1) {
    await page.waitForTimeout(200);
    await page.mouse.move(3 + (tick % 2), 3).catch(() => {});
  }
};
const witness = () => page.evaluate(() => JSON.parse(JSON.stringify(globalThis.__semioIoWitness ?? { anchors: [], inputs: [], objectUrls: 0 }))).catch(() => null);

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

const isMac = process.platform === "darwin";
const mod = isMac ? "Meta" : "Control";
const chord = async (keys) => {
  await page.mouse.move(720, 500).catch(() => {});
  await page.waitForTimeout(150);
  await page.keyboard.press(keys);
  await page.waitForTimeout(200);
};

const report = { url, startedAt: new Date().toISOString(), steps: [] };
const flush = () => {
  writeFileSync(join(outDir, "results.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
};
const note = async (step, detail) => {
  report.steps.push({ step, t: at(), ...detail });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1600)}`);
  flush();
};

const hops = (from, action) => {
  const slice = lines.slice(from);
  const hit = (needle) => slice.filter((line) => line.includes(needle)).length;
  return {
    leftTheInputAuthority: slice.filter((line) => line.includes("frame input action") && line.includes(`action=${action}`)).length,
    admittedByTheGuest: hit(`plugin_exchange actionId=${action}`),
    guestRanTheCommand: hit(`gen3d command action=${action}`),
    leftoverStash: slice.filter((line) => line.includes("wgpu-bridge effects leftover")).map((line) => line.slice(line.indexOf("leftover"), line.indexOf("leftover") + 80)),
    effectDropped: hit("wgpu-shell effect dropped"),
    unmapped: slice.filter((line) => line.includes("unmapped effect")).map((line) => line.slice(0, 160)),
    downloadRefused: slice.filter((line) => line.includes("wgpu-shell download")).map((line) => line.slice(0, 200)),
    dispatchFailed: hit("frame deferred action failed") + hit("dispatch failed"),
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
  report.fatal = "never booted";
  flush();
  await browser.close();
  process.exit(1);
}
await pump(settleSeconds);

if (!skipExport) {
  const from = lines.length;
  await chord(`${mod}+Shift+E`);
  await pump(10);
  await note("export:first-chord", { hops: hops(from, "exportDocument"), witness: await witness() });
  const second = lines.length;
  await chord(`${mod}+Shift+E`);
  await pump(30);
  await note("export:second-chord", { hops: hops(second, "exportDocument"), witness: await witness(), downloads: [...downloads] });
}

if (!skipImport) {
  const from = lines.length;
  const graphBefore = has("wgpu node-graph geometry surface=").at(-1) ?? null;
  await chord(`${mod}+O`);
  await pump(30);
  const graphAfter = has("wgpu node-graph geometry surface=").at(-1) ?? null;
  const importedIds = ["imported-source", "imported-geometry", "imported-preview"].filter((id) => has(id).length > 0);
  await note("import:chord", {
    hops: { request: hops(from, "importDocumentRequest"), chunk: hops(from, "importDocument") },
    fileChoosers: [...fileChoosers],
    witness: await witness(),
    graphChanged: graphBefore !== graphAfter,
    importedIdsSeenInConsole: importedIds,
  });
}

await page.screenshot({ path: join(outDir, "final.png") }).catch(() => {});
report.finishedAt = new Date().toISOString();
report.seconds = Math.round(at() / 1000);
report.downloads = downloads;
report.fileChoosers = fileChoosers;
report.witness = await witness();
flush();
console.log(`[DEBUG] io-effects DONE downloads=${downloads.length} choosers=${fileChoosers.length} anchors=${report.witness?.anchors.length ?? -1} inputs=${report.witness?.inputs.length ?? -1}`);
await browser.close();
