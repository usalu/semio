import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "restage-eval");
mkdirSync(outDir, { recursive: true });
const hits = [];
const timeline = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (msg) => {
  const text = msg.text();
  if (/contribution|setContributions|invokeExtension|unknown kind|extension-not|scoped from|requester|flow\.|meshes=/i.test(text)) {
    hits.push({ t: Date.now(), level: msg.type(), text: text.slice(0, 1500) });
    console.log(msg.type(), text.slice(0, 260));
  }
});
const snap = async () => page.evaluate(() => {
  const parse = (raw) => {
    if (!raw) return null;
    try { return JSON.parse(raw); } catch { return String(raw).slice(0, 400); }
  };
  const hosts = [...document.querySelectorAll(".semio-world-3d-host, [data-meshes-json], [data-status-json]")].map((el) => {
    const status = parse(el.getAttribute("data-status-json"));
    const meshesRaw = el.getAttribute("data-meshes-json");
    let meshCount = 0;
    try {
      const v = meshesRaw ? JSON.parse(meshesRaw) : [];
      meshCount = Array.isArray(v) ? v.length : 0;
    } catch { meshCount = 0; }
    return {
      surfaceId: el.getAttribute("data-surface-id"),
      meshCount,
      status,
      w: el.offsetWidth,
      h: el.offsetHeight,
    };
  });
  const body = (document.body?.innerText || "").replace(/\s+/g, " ").slice(0, 900);
  return { title: document.title, meshCount: hosts.reduce((n, h) => n + h.meshCount, 0), hosts, body };
});
const t0 = Date.now();
await page.goto(url, { waitUntil: "domcontentloaded" });
let last = null;
for (let i = 0; i < 45; i++) {
  await page.waitForTimeout(2000);
  last = await snap();
  const fault = JSON.stringify(last.hosts).slice(0, 800);
  const row = { i, elapsedMs: Date.now() - t0, meshCount: last.meshCount, title: last.title, fault };
  timeline.push({ i, elapsedMs: row.elapsedMs, meshCount: last.meshCount, hosts: last.hosts });
  const blob = JSON.stringify(last.hosts);
  const unknown = blob.includes("unknown kind");
  const contributed = blob.includes("extension-not-contributed");
  console.log(`[DEBUG] tick ${i} t=${row.elapsedMs} meshes=${last.meshCount} unknown=${unknown} notContributed=${contributed}`);
  if (last.meshCount > 0 || unknown) break;
}
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
const json = JSON.stringify({ hits, timeline, last }, (_, v) => typeof v === "bigint" ? Number(v) : v, 2);
writeFileSync(join(outDir, "eval.json"), json);
console.log("DONE meshes", last?.meshCount, "ticks", timeline.length, "hits", hits.length);
await browser.close();
