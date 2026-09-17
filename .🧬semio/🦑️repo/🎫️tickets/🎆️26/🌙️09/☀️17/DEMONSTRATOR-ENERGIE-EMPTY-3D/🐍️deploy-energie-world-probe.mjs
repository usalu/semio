/** 🔍 Quick probe: energie pane World3d mesh counts on deployed demonstrator. */
import { chromium } from "playwright";

const url = process.env.SEMIO_PROBE_URL ?? "https://v6.demonstrator.entwerfen.mit-bestand.de/%F0%9F%8C%90%EF%B8%8F.html#energie";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const lines = [];
page.on("console", (m) => lines.push(`${m.type()} ${m.text().slice(0, 500)}`));

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });
for (let i = 0; i < 120; i++) {
  await page.waitForTimeout(1000);
  const snap = await page.evaluate(() => {
    const worlds = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
      let ic = null;
      let mc = null;
      try { ic = JSON.parse(el.getAttribute("data-instances-json") ?? "null")?.length ?? null; } catch {}
      try { mc = JSON.parse(el.getAttribute("data-meshes-json") ?? "null")?.length ?? null; } catch {}
      const r = el.getBoundingClientRect();
      return {
        id: el.getAttribute("data-surface-id"),
        ic,
        mc,
        canvases: el.querySelectorAll("canvas").length,
        w: Math.round(r.width),
        h: Math.round(r.height),
      };
    });
    return {
      hash: location.hash,
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      worlds,
      treeitems: document.querySelectorAll('[role="treeitem"]').length,
    };
  });
  if (snap.worlds.some((w) => w.id === "window:energy.model.3d" && (w.ic ?? 0) > 0)) {
    console.log(JSON.stringify({ done: true, afterSec: i + 1, snap, consoleTail: lines.slice(-15) }, null, 2));
    await browser.close();
    process.exit(0);
  }
  if (i === 119) console.log(JSON.stringify({ done: false, snap, consoleTail: lines.slice(-30) }, null, 2));
}
await browser.close();
process.exit(1);
