/** 🔍️ Smoke probe for the puzzle3d React dev app: shell ready, no shell error, console clean, add object, undo, add again. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6013/?plugin=puzzle3d";
const OUT = process.env.PROBE_OUT ?? "smoke-probe.txt";
const BOOT_MS = Number(process.env.PROBE_BOOT_MS ?? 600_000);
const lines: string[] = [];
const log = (s: string) => { const l = `${((Date.now() - t0) / 1000).toFixed(1)}s ${s}`; lines.push(l); console.log(l); writeFileSync(OUT, lines.join("\n") + "\n"); };
const t0 = Date.now();
const errors: string[] = [];
const verdicts: string[] = [];
const verdict = (name: string, ok: boolean, detail = "") => { verdicts.push(`${ok ? "PASS" : "FAIL"} ${name} ${detail}`); log(`VERDICT ${ok ? "PASS" : "FAIL"} ${name} ${detail}`); };

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("pageerror", (e) => errors.push("PAGEERROR " + String(e).slice(0, 400)));
page.on("console", (m) => { const t = m.text(); if (/\[DEBUG\] /.test(t)) log("console " + t.slice(0, 300)); if (m.type() === "error" && !t.startsWith("[DEBUG] ")) errors.push(t.slice(0, 400)); });
page.on("response", (r) => { if (r.status() >= 400 && r.status() !== 404) errors.push(`http ${r.status()} ${decodeURIComponent(r.url()).slice(0, 200)}`); });
page.on("requestfailed", (r) => errors.push(`requestfailed ${r.failure()?.errorText} ${decodeURIComponent(r.url()).slice(0, 200)}`));

await page.goto(BASE, { waitUntil: "domcontentloaded", timeout: 120_000 });
const gl = await page.evaluate(() => { const c = document.createElement("canvas"); const g = c.getContext("webgl2") as WebGL2RenderingContext | null; const e = g?.getExtension("WEBGL_debug_renderer_info"); return g && e ? String(g.getParameter(e.UNMASKED_RENDERER_WEBGL)) : "none"; });
log(`renderer=${gl}`);

let state = "booting";
while (Date.now() - t0 < BOOT_MS) {
  state = await page.evaluate(() => {
    const el = document.querySelector("[data-shell-id]");
    if (!el) return "no-shell";
    if (el.hasAttribute("data-shell-error")) return "error:" + (el.getAttribute("data-shell-error") ?? "").slice(0, 300);
    if (el.hasAttribute("data-shell-not-found")) return "not-found";
    return el.hasAttribute("data-shell-ready") ? "ready" : "booting";
  });
  if (state === "ready" || state.startsWith("error") || state === "not-found") break;
  await page.waitForTimeout(3_000);
}
log(`shell state=${state}`);
verdict("shell-ready", state === "ready", state);
verdict("no-shell-error", !state.startsWith("error"), state);
if (state !== "ready") { log("errors: " + JSON.stringify([...new Set(errors)].slice(0, 10))); await page.screenshot({ path: OUT.replace(/\.txt$/, ".png") }).catch(() => {}); await browser.close(); process.exit(1); }

const count = () => page.evaluate(() => {
  const els = [...document.querySelectorAll("[data-instances-json]")] as HTMLElement[];
  let n = 0; for (const el of els) { try { n = Math.max(n, JSON.parse(el.getAttribute("data-instances-json") || "[]").length); } catch { /* ignore */ } }
  return { n, hosts: els.length };
});
const settle = async (pred: (n: number) => boolean, ms = 60_000) => { const s = Date.now(); let v = await count(); while (!pred(v.n) && Date.now() - s < ms) { await page.waitForTimeout(500); v = await count(); } return { ...v, ok: pred(v.n), waitedMs: Date.now() - s }; };
await page.waitForTimeout(3_000);
const skip = page.getByText(/^(Skip|Überspringen)$/);
if (await skip.count()) await skip.first().click({ force: true }).catch(() => {});

log("tabs: " + JSON.stringify(await page.$$eval('[role="tab"]', (els) => els.map((e) => `${e.id}=${(e as HTMLElement).innerText.trim()}`).slice(0, 30))));
const kindsTab = page.locator('[role="tab"]').filter({ hasText: /kinds|catalog|bauteil|arten/i }).first();
if (await kindsTab.count()) await kindsTab.click({ force: true }).catch(() => {});
await page.waitForTimeout(1_500);
const before = await count();
log(`before=${JSON.stringify(before)}`);
const row = page.locator('[id^="puzzle3d-play-kinds"] [role="treeitem"] [data-slot="tree-label"]').first();
verdict("catalogue-row-present", (await row.count()) > 0, `rows=${await row.count()}`);
await row.click({ force: true, timeout: 5_000 }).catch((e) => log("click failed " + String(e).slice(0, 200)));
const added = await settle((n) => n > before.n);
verdict("add-object-applies", added.ok, `before=${before.n} after=${added.n} waitedMs=${added.waitedMs}`);

const undo = () => page.evaluate(() => { const r = document.getElementById("framework.history.undo"); const b = (r?.querySelector("button") as HTMLElement | null) ?? (r as HTMLElement | null); b?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true })); return !!b; });
log(`undo-button=${await undo()}`);
let undone = await settle((n) => n === before.n, 30_000);
if (!undone.ok) { await page.keyboard.press("Meta+z"); undone = await settle((n) => n === before.n, 30_000); }
verdict("undo-restores", undone.ok, `expected=${before.n} got=${undone.n} waitedMs=${undone.waitedMs}`);

await row.click({ force: true, timeout: 5_000 }).catch(() => {});
const again = await settle((n) => n > before.n);
verdict("second-poll-cycle-add", again.ok, `after=${again.n} waitedMs=${again.waitedMs}`);

const bad = [...new Set(errors)];
verdict("console-clean", bad.length === 0, JSON.stringify(bad.slice(0, 8)));
await page.screenshot({ path: OUT.replace(/\.txt$/, ".png") }).catch(() => {});
log(`SUMMARY pass=${verdicts.filter((v) => v.startsWith("PASS")).length} fail=${verdicts.filter((v) => v.startsWith("FAIL")).length}`);
await browser.close();
process.exit(verdicts.some((v) => v.startsWith("FAIL")) ? 1 : 0);
