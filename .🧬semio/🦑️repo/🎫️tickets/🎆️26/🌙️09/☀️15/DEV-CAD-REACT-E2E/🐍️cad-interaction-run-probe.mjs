import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 🤝️ Runs model-definition interactions end to end in the served cad page: type the interaction's
// label into the pane window's "Action" search (the engagement HUD's possibles list), click ground
// points, type scalar entries, and confirm the committed object landed (mesh count, history) with the
// engagement preview lane painted in between.
//
//   SEMIO_PROBE_RUNS: JSON array of {surface, label, picks:[[fx,fy],…], entries:["3"], out:"place-column"}
//   (fractions of the pane rect). Defaults to the Building "Place Column" run.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6020/?plugin=cad";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 45);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "interaction-run");
mkdirSync(outDir, { recursive: true });
const defaultRuns = [{ surface: "window:cad-play-building", label: "Place Column", picks: [[0.45, 0.55], [0.6, 0.62]], entries: [], out: "place-column" }];
const runs = process.env.SEMIO_PROBE_RUNS ? JSON.parse(process.env.SEMIO_PROBE_RUNS) : defaultRuns;
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 1200)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1200)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(bootSeconds * 1000);
const report = {};
const note = (key, value) => { report[key] = value; lines.push(`${Date.now() - t0} probe ${key} ${JSON.stringify(value).slice(0, 900)}`); };
const host = (surface) => page.evaluate((s) => {
  const el = document.querySelector(`[data-surface-id="${s}"]`);
  if (!el) return null;
  const parse = (name) => { try { return JSON.parse(el.getAttribute(name) ?? "null"); } catch { return null; } };
  const r = el.getBoundingClientRect();
  const meshes = parse("data-meshes-json") ?? [];
  return { rect: { x: r.x, y: r.y, w: r.width, h: r.height }, meshes: meshes.length, meshIds: meshes.map((m) => m.id).slice(-3), instances: (parse("data-instances-json") ?? []).length, guest: parse("data-guest-selection-json"), preview: parse("data-engagement-preview-json") };
}, surface);
// 🪟 The window that owns a surface: its "Action" search input and the HUD text.
const windowOf = (surface) => page.evaluate((s) => {
  // 🪟 `[data-slot="window-body"]` is the pane's own chrome: its Actions toggle, Action search and HUD.
  const win = document.querySelector(`[data-surface-id="${s}"]`)?.closest('[data-slot="window-body"]');
  if (!win) return null;
  const input = win.querySelector('input[placeholder="Action"]');
  const ir = input?.getBoundingClientRect();
  const hud = win.querySelector('[data-slot="engagement"]');
  return { input: ir ? { placeholder: input.placeholder, value: input.value, x: ir.x, y: ir.y, w: ir.width, h: ir.height } : null, hud: hud?.innerText?.replace(/\s+/g, " ").slice(0, 400) ?? null, heading: win.querySelector('[data-slot="engagement-step-heading"]')?.textContent?.trim() ?? null, options: [...win.querySelectorAll('[data-slot="engagement-options"] button')].map((b) => b.textContent?.trim()).slice(0, 12) };
}, surface);
const possibles = () => page.evaluate(() => [...document.querySelectorAll('[data-slot="search-autocomplete"] [cmdk-item]')].map((e) => e.textContent?.trim().slice(0, 60)));
const historyLines = () => lines.filter((l) => /history patch applied|Committed|engagement/i.test(l)).slice(-6).map((l) => l.slice(0, 260));
for (const run of runs) {
  const key = run.out ?? run.label;
  const r = {};
  const rnote = (k, v) => { r[k] = v; lines.push(`${Date.now() - t0} probe ${key}.${k} ${JSON.stringify(v).slice(0, 900)}`); };
  try {
    const before = await host(run.surface);
    if (!before) throw new Error(`no surface ${run.surface}`);
    rnote("before", { meshes: before.meshes, instances: before.instances });
    let win = await windowOf(run.surface);
    if (!win?.input) {
      // 🎛️ The engagement chrome (Action search + HUD) sits behind the window's "Actions" button.
      const button = await page.evaluate((s) => { const b = [...(document.querySelector(`[data-surface-id="${s}"]`)?.closest('[data-slot="window-body"]')?.querySelectorAll("button") ?? [])].find((x) => x.textContent?.trim() === "Actions"); const rr = b?.getBoundingClientRect(); return rr ? { x: rr.x + rr.width / 2, y: rr.y + rr.height / 2 } : null; }, run.surface);
      if (!button) throw new Error(`no Actions button in ${run.surface}`);
      await page.mouse.click(button.x, button.y);
      await page.waitForTimeout(1500);
      win = await windowOf(run.surface);
    }
    if (!win?.input) throw new Error(`no Action input for ${run.surface}: ${JSON.stringify(win)}`);
    // 🔎 Type the label into the window's Action search and confirm the top-ranked possible. The
    // shell PascalCases drafts and treats Space as "activate the top match", so the label goes in
    // without its spaces ("PlaceColumn"), exactly as the inline completion spells it.
    const draft = run.label.replace(/\s+/g, "");
    for (let attempt = 0; attempt < 3; attempt++) {
      // ⌨️ The guest echoes `engagement-input` back into the draft; a slow echo can truncate a fast
      // draft, so re-type until the field holds the whole label before confirming.
      await page.mouse.click(win.input.x + win.input.w / 2, win.input.y + win.input.h / 2);
      await page.keyboard.press("Meta+A");
      await page.keyboard.type(draft, { delay: 40 });
      await page.waitForTimeout(1200);
      win = await windowOf(run.surface);
      rnote(`draft:${attempt}`, win?.input?.value);
      if (win?.input?.value === draft) break;
    }
    rnote("possibles", await possibles());
    await page.keyboard.press("Enter");
    await page.waitForTimeout(2500);
    win = await windowOf(run.surface);
    rnote("hudAfterStart", { heading: win?.heading, hud: win?.hud, options: win?.options });
    const started = await host(run.surface);
    rnote("guestAfterStart", started.guest);
    await page.screenshot({ path: join(outDir, `${key}-started.png`), type: "png" });
    const pane = before.rect;
    const previews = [];
    for (const [index, [fx, fy]] of run.picks.entries()) {
      const x = pane.x + pane.w * fx;
      const y = pane.y + pane.h * fy;
      await page.mouse.move(x, y);
      await page.waitForTimeout(900);
      const hover = await host(run.surface);
      previews.push({ beforePick: index, preview: (hover.preview ?? []).map((i) => `${i.kind}${i.role ? ":" + i.role : ""}${i.position ? "@" + i.position.map((v) => Number(v).toFixed(2)).join(",") : ""}`), raw: hover.preview });
      if (index === run.picks.length - 1) await page.screenshot({ path: join(outDir, `${key}-rubber-band.png`), type: "png" });
      await page.mouse.click(x, y);
      await page.waitForTimeout(1800);
      const after = await windowOf(run.surface);
      previews.push({ afterPick: index, heading: after?.heading, hud: after?.hud?.slice(0, 160) });
    }
    rnote("previews", previews);
    for (const entry of run.entries ?? []) {
      win = await windowOf(run.surface);
      await page.mouse.click(win.input.x + win.input.w / 2, win.input.y + win.input.h / 2);
      await page.keyboard.type(String(entry), { delay: 20 });
      await page.keyboard.press("Enter");
      await page.waitForTimeout(2500);
      const after = await windowOf(run.surface);
      rnote(`entry:${entry}`, { heading: after?.heading, hud: after?.hud?.slice(0, 160) });
    }
    for (const option of run.options ?? []) {
      // 🔘 Click a HUD option button (e.g. "Accept") by label.
      const target = await page.evaluate(([s, label]) => {
        const w = document.querySelector(`[data-surface-id="${s}"]`)?.closest('[data-slot="window-body"]');
        const b = [...(w?.querySelectorAll('[data-slot="engagement-options"] button') ?? [])].find((x) => (x.textContent ?? "").trim() === label);
        const rr = b?.getBoundingClientRect();
        return rr ? { x: rr.x + rr.width / 2, y: rr.y + rr.height / 2 } : null;
      }, [run.surface, option]);
      rnote(`option:${option}`, target);
      if (target) { await page.mouse.click(target.x, target.y); await page.waitForTimeout(2500); }
    }
    await page.waitForTimeout(1500);
    const after = await host(run.surface);
    win = await windowOf(run.surface);
    rnote("after", { meshes: after.meshes, instances: after.instances, newMeshIds: after.meshIds, guest: after.guest, heading: win?.heading, hud: win?.hud?.slice(0, 200) });
    rnote("history", historyLines());
    await page.screenshot({ path: join(outDir, `${key}-committed.png`), type: "png" });
    r.ok = after.meshes > before.meshes || (run.expectMeshes === "same" && after.meshes === before.meshes);
  } catch (error) {
    rnote("error", String(error).slice(0, 500));
    await page.screenshot({ path: join(outDir, `${key}-error.png`), type: "png" }).catch(() => {});
  }
  report[key] = r;
}
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
const faults = lines.filter((l) => /pageerror|dropped action|Unknown action|fixed-capacity|action failed|typed-operation completion effects failed|guest fault|unreachable/i.test(l));
console.log("DONE faults", faults.length, "runs", Object.entries(report).map(([k, v]) => `${k}:${v.ok ? "ok" : "FAIL"}`).join(" "));
for (const f of faults.slice(0, 8)) console.log("FAULT", f.slice(0, 300));
console.log(JSON.stringify(report, null, 1).slice(0, 9000));
await browser.close();
