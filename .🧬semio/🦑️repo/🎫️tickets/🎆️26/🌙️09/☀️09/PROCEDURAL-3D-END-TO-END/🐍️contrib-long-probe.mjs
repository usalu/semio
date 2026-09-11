
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "contrib-long");
mkdirSync(outDir, { recursive: true });
const hits = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => {
  const text = msg.text();
  if (/contribution|setContributions|invokeExtension|scoped from|deferred effects|requester missing/i.test(text)) {
    hits.push({ t: Date.now(), level: msg.type(), text: text.slice(0, 1200) });
    console.log(msg.type(), text.slice(0, 220));
  }
});
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(40000);
const snap = await page.evaluate(() => {
  const q = (sel) => Array.from(document.querySelectorAll(sel));
  const parseLen = (raw) => {
    if (!raw) return 0;
    try {
      const v = JSON.parse(raw);
      return Array.isArray(v) ? v.length : v && typeof v === "object" ? Object.keys(v).length : 0;
    } catch { return 0; }
  };
  const parseJson = (raw) => {
    if (!raw) return null;
    try { return JSON.parse(raw); } catch { return String(raw).slice(0, 400); }
  };
  const hosts = q(".semio-world-3d-host, [data-meshes-json], [data-status-json]").map((el) => ({
    surfaceId: el.getAttribute("data-surface-id"),
    meshes: parseLen(el.getAttribute("data-meshes-json")),
    instances: parseLen(el.getAttribute("data-instances-json")),
    status: parseJson(el.getAttribute("data-status-json")),
    w: el.offsetWidth,
    h: el.offsetHeight,
  }));
  return {
    title: document.title,
    meshCount: hosts.reduce((n, h) => n + h.meshes, 0),
    previewHosts: hosts,
    body: (document.body?.innerText || "").replace(/\s+/g, " ").slice(0, 800),
  };
});
const json = JSON.stringify({ hits, snap }, (_, v) => typeof v === "bigint" ? Number(v) : v, 2);
writeFileSync(join(outDir, "long.json"), json);
console.log("meshes", snap.meshCount, "hits", hits.length, "title", snap.title);
await browser.close();
