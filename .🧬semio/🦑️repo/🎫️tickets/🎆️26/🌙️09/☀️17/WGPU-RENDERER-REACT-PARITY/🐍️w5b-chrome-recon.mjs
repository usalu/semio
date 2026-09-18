/** 🔎️ Chrome recon — the selector census `🐍️parity-interact-probe.mjs`'s React driver is written from.
 *
 * Boots ONE renderer and dumps what its chrome actually publishes: every `data-*` attribute name in the
 * document with a sample, every element carrying an `id`/`role` that looks like shell chrome, and (for the
 * wgpu target) the whole `semioWgpuIntrospection.dumpChrome()` hit registry. Nothing is clicked — this is
 * the "what is there to aim at" pass, so the journey steps are written against evidence rather than guesses.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6013/?plugin=puzzle3d SEMIO_PROBE_OUT=w5b-recon-react bun 🐍️w5b-chrome-recon.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6013/?plugin=puzzle3d";
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 240);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w5b-recon");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: process.env.SEMIO_PROBE_HEADED !== "1", args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (message) => lines.push(`${Date.now() - t0} ${message.type()} ${message.text().slice(0, 4000)}`));
page.on("pageerror", (error) => lines.push(`${Date.now() - t0} pageerror ${String(error).slice(0, 2000)}`));
page.on("worker", (worker) => worker.on("console", (message) => lines.push(`${Date.now() - t0} worker ${message.type()} ${message.text().slice(0, 4000)}`)));
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {}
});
await page.goto(url, { waitUntil: "domcontentloaded" });

for (let second = 0; second < bootSeconds; second += 1) {
  await page.waitForTimeout(1000);
  const ready = await page.evaluate(() => document.documentElement.hasAttribute("data-semio-os-ready") || typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function").catch(() => false);
  if (ready && second > 6) break;
}

const dom = await page
  .evaluate(() => {
    const attributes = {};
    for (const element of document.querySelectorAll("*")) {
      for (const attribute of element.attributes) {
        if (!attribute.name.startsWith("data-") && attribute.name !== "role") continue;
        const bucket = (attributes[attribute.name] ??= { count: 0, samples: [] });
        bucket.count += 1;
        if (bucket.samples.length < 12 && !bucket.samples.includes(attribute.value)) bucket.samples.push(attribute.value.slice(0, 80));
      }
    }
    const describe = (element) => ({
      tag: element.tagName.toLowerCase(),
      id: element.id || undefined,
      role: element.getAttribute("role") ?? undefined,
      slot: element.getAttribute("data-slot") ?? undefined,
      label: (element.getAttribute("aria-label") ?? element.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 60) || undefined,
      pressed: element.getAttribute("aria-pressed") ?? undefined,
      expanded: element.getAttribute("aria-expanded") ?? undefined,
      rect: (() => {
        const rect = element.getBoundingClientRect();
        return [Math.round(rect.x), Math.round(rect.y), Math.round(rect.width), Math.round(rect.height)];
      })(),
    });
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      attributes,
      buttons: [...document.querySelectorAll("button")].map(describe).slice(0, 140),
      ided: [...document.querySelectorAll("[id]")].map(describe).slice(0, 220),
      slots: [...document.querySelectorAll("[data-slot]")].map(describe).slice(0, 160),
      surfaces: [...document.querySelectorAll("[data-surface-id]")].map((element) => element.getAttribute("data-surface-id")),
      canvases: document.querySelectorAll("canvas").length,
      ledger: globalThis.__semioInputLedger ?? null,
    };
  })
  .catch((error) => ({ error: String(error).slice(0, 400) }));

const chrome = await page
  .evaluate(async () => {
    const beacon = globalThis.semioWgpuIntrospection;
    if (!beacon) return null;
    const read = async (name, argument) => {
      try {
        const raw = await beacon[name]?.(argument);
        return raw ? JSON.parse(raw) : null;
      } catch (error) {
        return { error: String(error).slice(0, 300) };
      }
    };
    return { chrome: await read("dumpChrome"), structure: await read("dumpStructure") };
  })
  .catch((error) => ({ error: String(error).slice(0, 400) }));

await page.screenshot({ path: join(outDir, "recon.png"), type: "png" }).catch(() => {});
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "recon.json"), JSON.stringify({ url, dom, chrome }, null, 2));
console.log(`[DEBUG] recon url=${url} ready=${dom?.ready ?? "?"} error=${dom?.error ?? "-"} buttons=${dom?.buttons?.length ?? 0} slots=${Object.keys(dom?.attributes ?? {}).length} wgpuHits=${chrome?.chrome?.hits?.length ?? "n/a"} out=${outDir}`);
await browser.close();
