/** 🪜️ wgpu INPUT HOP probe — which hop a DOM input dies at, on the live 6118 surface.
 *
 * Drives one viewport resize, one hover, one click, `f` and `Tab` against
 * `?plugin=generation3d&mode=generate`, marks each in the console stream, and prints back every
 * non-heartbeat line the shell produced between them. Read it against the renderer's own `[DEBUG]`
 * ladder — each rung is a different owner, so the first rung that stops printing names the defect:
 *
 *   os_host handle_event …            🪟️winit-app  `WindowDelegate::handle_event`  (the wire arrived)
 *   os_host drain events …            🪟️winit-app  `build_and_publish_snapshot`    (queued as an apply)
 *   apply_pending_step blocked: …     🧊️renderer   `RuntimeMailbox`                (mailbox admission)
 *   apply_step DispatchEvents …       🧊️renderer   `RuntimeApply::apply_step`      (the apply ran)
 *   start_dispatch …                  🧊️renderer   `AppRuntime::start_dispatch`    (a future was spawned)
 *   os_host dispatch_normalized_event 🪟️winit-app  `dispatch_normalized_event`     (Ui::dispatch_event)
 *   wgpu-shell command …              🐚️Shell                                      (an action settled)
 *
 * It also prints every `dock plan` line, so a resize that did not reach `handle_metrics` is visible,
 * and the hovered/selected state of the Generations window's rows after the click.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_SETTLE=40 bun 🐍️wgpu-dispatch-hop-probe.mjs
 */
import { chromium } from "playwright";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&mode=generate";
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 40);
const t0 = Date.now();
const at = () => Date.now() - t0;
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.text().slice(0, 400)}`));
const mark = (label) => lines.push(`${at()} MARK ${label}`);
await page.goto(url, { waitUntil: "domcontentloaded" });
for (let s = 0; s < settle; s += 1) await page.waitForTimeout(1000);
mark("settled");
await page.setViewportSize({ width: 1200, height: 800 });
await page.waitForTimeout(4000);
mark("resized");
await page.mouse.move(160, 138);
await page.waitForTimeout(1500);
await page.mouse.move(160, 140);
await page.waitForTimeout(2500);
mark("hovered");
await page.mouse.click(160, 138);
await page.waitForTimeout(4000);
mark("clicked");
await page.keyboard.press("f");
await page.waitForTimeout(2500);
mark("key-f");
await page.keyboard.press("Tab");
await page.waitForTimeout(2500);
mark("key-tab");
const dumps = await page.evaluate(async () => {
  const b = globalThis.semioWgpuIntrospection;
  const s = await b.dumpStructure("generation3d-generations");
  return s ? JSON.parse(s) : null;
});
const hovered = (dumps?.nodes ?? []).filter((n) => n.state && JSON.stringify(n.state).includes("true")).map((n) => ({ p: n.path, s: n.state }));
console.log(JSON.stringify({ marks: lines.filter((l) => l.includes("MARK")), dockPlans: lines.filter((l) => l.includes("dock plan")), afterSettle: lines.filter((l) => Number(l.split(" ")[0]) > settle * 1000).slice(0, 60), hovered, alert: await page.evaluate(() => document.querySelector('[role="alert"]')?.textContent?.slice(0,200) ?? null) }, null, 2));
await browser.close();
