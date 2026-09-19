/** 🧩️ Slice-B3d cad EXTENSION proof — verbatim reuse of `26/09/15/DEV-CAD-REACT-E2E`'s
 * `🐍️cad-interaction-run-probe.mjs` (only the playwright import path, the output folder and this header
 * differ), pointed at a model-definition interaction that exists ONLY because a cad extension
 * contributed it: the `🏢️aec-building` extension's `building.*` assets paint the Building pane's
 * suggestions, so a committed "Place Column" there is the extension's own action running end to end.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 🤝️ Runs model-definition interactions end to end in the served cad page: type the interaction's
// label into the pane window's "Action" search (the engagement HUD's possibles list), click ground
// points, type scalar entries, and confirm the committed object landed (mesh count, history) with the
// engagement preview lane painted in between.
//
//   SEMIO_PROBE_RUNS: JSON array of {surface, label, picks:[[fx,fy],…], entries:["3"], out:"place-column"}
//   or, when order matters, steps:[{pick:[fx,fy]}, {entry:"2"}, {entry:""}, {option:"Accept"}]; an optional
//   preSelect:[fx,fy] clicks an object before the session starts (a plain `interactionSelect` pick)
//   (fractions of the pane rect — keep fx ≥ 0.5: the opened Actions panel with its HUD covers the
//   pane's left ~40 %, and a pick under it lands on an action row instead of the ground).
//   Defaults to the Building "Place Column" run.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6020/?plugin=cad";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 45);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "b3d-cad-extension");
mkdirSync(outDir, { recursive: true });
const defaultRuns = [{ surface: "window:cad-play-building", label: "Place Column", picks: [[0.55, 0.55], [0.7, 0.62]], entries: [], out: "place-column" }];
const runs = process.env.SEMIO_PROBE_RUNS ? JSON.parse(process.env.SEMIO_PROBE_RUNS) : defaultRuns;
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 1200)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1200)}`));
page.setDefaultNavigationTimeout(180_000);
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });
await page.waitForTimeout(bootSeconds * 1000);
const report = {};
const note = (key, value) => { report[key] = value; lines.push(`${Date.now() - t0} probe ${key} ${JSON.stringify(value).slice(0, 900)}`); };
const host = (surface) => page.evaluate((s) => {
  const el = document.querySelector(`[data-surface-id="${s}"]`);
  if (!el) return null;
  const parse = (name) => { try { return JSON.parse(el.getAttribute(name) ?? "null"); } catch { return null; } };
  const r = el.getBoundingClientRect();
  const meshes = parse("data-meshes-json") ?? [];
  const selection = parse("data-selection-json");
  return { rect: { x: r.x, y: r.y, w: r.width, h: r.height }, meshes: meshes.length, meshIds: meshes.map((m) => m.id).slice(-3), instances: (parse("data-instances-json") ?? []).length, guest: parse("data-guest-selection-json"), sessionActive: selection?.engagementSessionActive ?? null, preview: parse("data-engagement-preview-json") };
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
    // 🛑️ Escape in the Action line aborts whatever session the pane still holds (`onAbort` →
    // `engagementAbort`), so every run starts from the pane's own interaction list.
    await page.mouse.click(win.input.x + win.input.w / 2, win.input.y + win.input.h / 2);
    await page.keyboard.press("Escape");
    await page.waitForTimeout(1200);
    let movedId = null;
    if (run.preSelect) {
      // 🎯 A plain pick (no session) selects the object under the pointer through `interactionSelect`;
      // several candidate spots are tried until one lands on an object.
      const pane0 = before.rect;
      const candidates = Array.isArray(run.preSelect[0]) ? run.preSelect : [run.preSelect];
      for (const [fx, fy] of candidates) {
        await page.mouse.click(pane0.x + pane0.w * fx, pane0.y + pane0.h * fy);
        await page.waitForTimeout(1500);
        const guest = (await host(run.surface)).guest;
        if (guest?.selectedIds?.length) { movedId = guest.selectedIds[0]; rnote("preSelect", { at: [fx, fy], selectedIds: guest.selectedIds }); break; }
      }
      if (!movedId) rnote("preSelect", { selectedIds: [] });
    }
    const meshDigest = async (id) => page.evaluate(([s, target]) => {
      const el = document.querySelector(`[data-surface-id="${s}"]`);
      const meshes = JSON.parse(el?.getAttribute("data-meshes-json") ?? "[]");
      const mesh = meshes.find((m) => m.id === target);
      const positions = mesh?.data?.positions ?? null;
      const instance = JSON.parse(el?.getAttribute("data-instances-json") ?? "[]").find((i) => i.id === target);
      // 🧭 A transform lands as the instance pose (position/rotation/scale) or, for a re-tessellated
      // solid, in the mesh positions — record both.
      return { count: positions?.length ?? 0, head: (positions ?? []).slice(0, 6).map((v) => Number(v).toFixed(3)), pose: instance ? { position: instance.position, rotation: instance.rotation, scale: instance.scale } : null };
    }, [run.surface, id]);
    const digestBefore = movedId ? await meshDigest(movedId) : null;
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
    rnote("guestAfterStart", { guest: started.guest, sessionActive: started.sessionActive });
    await page.screenshot({ path: join(outDir, `${key}-started.png`), type: "png" });
    const pane = before.rect;
    const previews = [];
    const steps = run.steps ?? [...(run.picks ?? []).map((pick) => ({ pick })), ...(run.entries ?? []).map((entry) => ({ entry })), ...(run.options ?? []).map((option) => ({ option }))];
    let pickIndex = 0;
    for (const step of steps) {
      if (step.pick) {
        const index = pickIndex++;
        const x = pane.x + pane.w * step.pick[0];
        const y = pane.y + pane.h * step.pick[1];
        await page.mouse.move(x, y);
        await page.waitForTimeout(900);
        const hover = await host(run.surface);
        previews.push({ beforePick: index, sessionActive: hover.sessionActive, preview: (hover.preview ?? []).map((i) => `${i.kind}${i.role ? ":" + i.role : ""}${i.position ? "@" + i.position.map((v) => Number(v).toFixed(2)).join(",") : ""}`), raw: hover.preview });
        if (index === 1) await page.screenshot({ path: join(outDir, `${key}-rubber-band.png`), type: "png" });
        await page.mouse.click(x, y);
        await page.waitForTimeout(1800);
        const after = await windowOf(run.surface);
        previews.push({ afterPick: index, heading: after?.heading, hud: after?.hud?.slice(0, 160) });
      } else if (step.entry !== undefined) {
        win = await windowOf(run.surface);
        await page.mouse.click(win.input.x + win.input.w / 2, win.input.y + win.input.h / 2);
        if (step.entry !== "") await page.keyboard.type(String(step.entry), { delay: 40 });
        await page.waitForTimeout(600);
        await page.keyboard.press("Enter");
        await page.waitForTimeout(2500);
        const after = await windowOf(run.surface);
        previews.push({ entry: step.entry, heading: after?.heading, hud: after?.hud?.slice(0, 160) });
      } else if (step.undo) {
        // ↩️ ⌘Z on the page body: the framework's undo of the last document edit (the commit).
        await page.mouse.click(pane.x + pane.w * 0.95, pane.y + 4);
        await page.keyboard.press("Escape");
        await page.keyboard.press("Meta+Z");
        await page.waitForTimeout(2500);
        const h = await host(run.surface);
        previews.push({ undo: true, meshes: h.meshes, hud: (await windowOf(run.surface))?.hud?.slice(0, 120) });
      } else if (step.option) {
        // 🔘 Click a HUD option button by label.
        const target = await page.evaluate(([s, label]) => {
          const w = document.querySelector(`[data-surface-id="${s}"]`)?.closest('[data-slot="window-body"]');
          const b = [...(w?.querySelectorAll('[data-slot="engagement-options"] button') ?? [])].find((x) => (x.textContent ?? "").trim() === label);
          const rr = b?.getBoundingClientRect();
          return rr ? { x: rr.x + rr.width / 2, y: rr.y + rr.height / 2 } : null;
        }, [run.surface, step.option]);
        if (target) { await page.mouse.click(target.x, target.y); await page.waitForTimeout(2500); }
        const after = await windowOf(run.surface);
        previews.push({ option: step.option, found: Boolean(target), hud: after?.hud?.slice(0, 160) });
      }
    }
    rnote("steps", previews);
    await page.waitForTimeout(1500);
    const after = await host(run.surface);
    win = await windowOf(run.surface);
    rnote("after", { meshes: after.meshes, instances: after.instances, newMeshIds: after.meshIds, guest: after.guest, heading: win?.heading, hud: win?.hud?.slice(0, 200) });
    rnote("history", historyLines());
    if (movedId) rnote("moved", { id: movedId, before: digestBefore, after: await meshDigest(movedId) });
    await page.screenshot({ path: join(outDir, `${key}-committed.png`), type: "png" });
    r.ok = after.meshes > before.meshes || (run.expectMeshes === "same" && after.meshes === before.meshes && (!movedId || JSON.stringify(r.moved.before) !== JSON.stringify(r.moved.after)));
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
