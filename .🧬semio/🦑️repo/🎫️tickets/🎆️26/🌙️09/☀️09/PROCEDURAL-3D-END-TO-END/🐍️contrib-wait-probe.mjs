
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "contrib-wait");
mkdirSync(outDir, { recursive: true });
const interesting = [];
let invoke = 0;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => {
  const text = msg.text();
  if (/invokeExtension/i.test(text)) invoke += 1;
  if (/contribution|setContributions|invokeExtension|extension-not|scoped from|meshes/i.test(text)) {
    interesting.push({ t: Date.now(), level: msg.type(), text: text.slice(0, 1500) });
    if (/contribution|setContributions|invokeExtension|scoped from/i.test(text)) console.log(msg.type(), text.slice(0, 240));
  }
});
const t0 = Date.now();
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(25000);
const shot = await page.evaluate(() => {
  const hosts = [...document.querySelectorAll("[data-preview-host],[data-surface-id],canvas")].slice(0, 8).map((el) => ({
    id: el.id,
    meshes: el.getAttribute("data-meshes") || el.dataset?.meshes || null,
    status: el.getAttribute("data-status") || null,
    text: (el.innerText || "").slice(0, 200),
  }));
  const body = document.body.innerText.slice(0, 1500);
  const preview = window.__SEMIO_PREVIEW_HOSTS__ ?? null;
  return { title: document.title, body, hosts, preview };
});
const result = { elapsedMs: Date.now() - t0, invoke, interesting, shot };
writeFileSync(join(outDir, "wait.json"), JSON.stringify(result, (_, v) => typeof v === "bigint" ? Number(v) : v, 2));
console.log("elapsed", result.elapsedMs, "invoke", invoke, "hits", interesting.length, "title", shot.title);
await browser.close();
