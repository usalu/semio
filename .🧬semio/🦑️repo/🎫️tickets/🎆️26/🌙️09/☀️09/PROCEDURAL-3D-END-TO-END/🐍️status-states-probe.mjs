/** 🚦 Every node-status state a user can be shown, DRIVEN and READ — in English and in German.
 *
 * `Generation3dLabels` declares six status words (`status_ok`/`stale`/`queued`/`computing`/`error`/
 * `blocked`) and the Flow outline paints one of them beside every node row (`node_status_label`,
 * `🕸️flow/🦀️.rs`). Until now only `Evaluated` was ever asserted by anything: the other five were
 * unreachable to the battery, and their German forms were never read at all
 * (`📓️window-coverage-audit-2026-09-14.md` §5 items 5 and 7).
 *
 * Reading them is a SAMPLING problem, not a click problem — `stale`, `queued` and `computing` exist
 * only while an evaluation is in flight — so this probe polls the app's own published evidence four
 * times a second across a whole convergence and records, for every status tag it ever sees, the ROW
 * TEXT the outline painted for that node at that same instant. A tag observed in
 * `data-status-json` whose word never reached a row is the finding, and so is the reverse.
 *
 * Two of the six have to be provoked rather than waited for:
 *
 *   `blocked` — a node whose input port has no wire. The packaged column example already ships one
 *               (`extrude` blocked on `wire`), and a wire cut would only re-prove the same producer.
 *   `error`   — an operator that FAILS. Driven the way a user would: select an input widget in the
 *               Document panel and type a degenerate value into the Inspection panel's own field
 *               (radius 0 → a polygon with no area → the extrude downstream of it fails).
 *
 * The whole drive runs twice, once per locale, because a status word is only proven localized when
 * the SAME state paints the German word (`Veraltet`, `In Warteschlange`, `Blockiert`, …).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-gaps/status-states bun 🐍️status-states-probe.mjs
 * @see 🐍️react-battery.mjs, 🗣️terminology/🦀️.rs, 🧰️framework/…/🌊️flow/🖥️host/🦀️.rs `build_flow_status_json`
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "status-states");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 200);
mkdirSync(outDir, { recursive: true });

const MAIN = "window:procedural-main";
/** 🗣️ The six `Generation3dLabels` status words, per locale — the exact strings `🗣️terminology/🦀️.rs` declares. */
const STATUS_WORDS = {
  en: { ok: "Evaluated", stale: "Stale", queued: "Queued", computing: "Computing", error: "Error", blocked: "Blocked" },
  de: { ok: "Ausgewertet", stale: "Veraltet", queued: "In Warteschlange", computing: "Berechnet", error: "Fehler", blocked: "Blockiert" },
};
const TAGS = ["ok", "stale", "queued", "computing", "error", "blocked"];

const lines = [];
const t0 = Date.now();
const results = { url, steps: [], passes: {} };
let shot = 0;

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));

const note = async (step, ok, detail) => {
  shot += 1;
  results.steps.push({ step, ok, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ok=${ok} ${JSON.stringify(detail).slice(0, 600)}`);
  await page.screenshot({ path: join(outDir, `${String(shot).padStart(2, "0")}-${step.replace(/[^a-z0-9]+/giu, "-")}.png`) }).catch(() => {});
};

/** 📡️ One instant: the guest's own per-node status map and the outline rows that mirror it. */
const sample = () =>
  page.evaluate(
    (main) => {
      const host = document.querySelector(`[data-surface-id="${main}"]`);
      let status = null;
      try {
        status = JSON.parse(host?.getAttribute("data-status-json") ?? "null");
      } catch {
        status = null;
      }
      const rows = [...document.querySelectorAll('[data-slot="window"][id="procedural-main"] [role="treeitem"]')].map((el) => ({ id: (el.id ?? "").split("/").pop(), text: (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120) }));
      return { status, rows, lang: document.documentElement.lang };
    },
    MAIN,
  );

/** 🚦 Walks one instant: for every node the guest tagged, the row text the outline painted for it. */
const fold = (seen, instant) => {
  if (!instant.status || typeof instant.status !== "object") return seen;
  for (const [nodeId, entry] of Object.entries(instant.status)) {
    const tag = entry?.status;
    if (typeof tag !== "string") continue;
    const row = instant.rows.find((r) => r.id === nodeId);
    const known = seen[tag];
    if (known && known.rowText) continue;
    seen[tag] = { nodeId, rowText: row?.text ?? null, at: Date.now() - t0, ports: entry?.ports ?? null };
  }
  return seen;
};

const poll = async (seen, seconds) => {
  const deadline = Date.now() + seconds * 1000;
  while (Date.now() < deadline) {
    fold(seen, await sample());
    if (TAGS.every((tag) => seen[tag]?.rowText)) break;
    await page.waitForTimeout(250);
  }
  return seen;
};

const clickId = (id) => page.locator(`[id="${id}"]`).first().click({ timeout: 8000 });

/** 🔁️ Two hops, never one: re-picking the option already shown dispatches nothing, so a fresh
 * convergence needs the example to actually change and change back. */
const pickExample = async (match) => {
  const opened = await clickId("playground.navbar.fixture")
    .then(() => true)
    .catch((e) => {
      lines.push(`example picker ${String(e).replace(/\s+/gu, " ").slice(0, 160)}`);
      return false;
    });
  if (!opened) return false;
  await page.waitForTimeout(1200);
  const option = page.locator('[role="option"]').filter({ hasText: match }).first();
  if (!(await option.count())) {
    await page.keyboard.press("Escape");
    return false;
  }
  await option.click({ timeout: 8000 }).catch(() => {});
  await page.waitForTimeout(1000);
  return true;
};

/** ❌️ Drives a real evaluation FAILURE the way a user would: pick an input widget in the Document
 * panel, then type a degenerate value into the Inspection panel's own number field. */
const driveError = async (seen) => {
  const attempts = [];
  await clickId("framework.panel.artifact").catch((e) => lines.push(`artifact tab ${String(e).replace(/\s+/gu, " ").slice(0, 140)}`));
  await page.waitForTimeout(2000);
  const rows = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')].map((el) => ({ id: el.id, text: (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40) })));
  const wanted = ["radius", "sides", "height"].map((name) => rows.find((r) => r.id.endsWith(`/${name}`))).filter(Boolean);
  for (const row of wanted) {
    await page.locator(`[data-slot="panel"] [id="${row.id}"]`).first().click({ timeout: 8000 }).catch(() => {});
    await page.waitForTimeout(1800);
    await clickId("framework.panel.inspection").catch(() => {});
    await page.waitForTimeout(2000);
    const field = page.locator('[data-slot="panel"] [id$="procedural-play-inspector.value.input"]').first();
    if ((await field.count()) === 0) {
      attempts.push({ row: row.id, field: false });
      continue;
    }
    for (const value of ["0", "-1"]) {
      await field.fill(value, { timeout: 8000 }).catch(() => {});
      await page.keyboard.press("Enter");
      await field.blur().catch(() => {});
      await poll(seen, 40);
      attempts.push({ row: row.id, field: true, typed: value, error: seen.error ?? null });
      if (seen.error?.rowText) return attempts;
    }
  }
  return attempts;
};

const setLocale = async (label) => {
  await clickId("framework.settings").catch(() => {});
  await page.waitForTimeout(1800);
  await clickId("framework.settings.language").catch(() => {});
  await page.waitForTimeout(1200);
  const option = page.locator('[role="option"]').filter({ hasText: label }).first();
  const picked = (await option.count()) > 0 && (await option.click({ timeout: 8000 }).then(() => true).catch(() => false));
  await page.keyboard.press("Escape");
  await page.waitForTimeout(4000);
  return picked;
};

/** 🎬️ One locale's whole drive: a fresh convergence to sample the transient states, then the two
 * provoked ones. */
const runPass = async (locale) => {
  const seen = {};
  await poll(seen, 25);
  await pickExample(/Rectangle Extrude|Rechteck/u);
  await poll(seen, 60);
  await pickExample(/Hexagonal|Sechseckige/u);
  await poll(seen, 90);
  const attempts = seen.error?.rowText ? [] : await driveError(seen);
  results.passes[locale] = { seen, attempts, rows: (await sample()).rows };
  for (const tag of TAGS) {
    const word = STATUS_WORDS[locale][tag];
    const hit = seen[tag];
    await note(`${locale}:${tag}`, Boolean(hit?.rowText && hit.rowText.includes(word)), { word, ...(hit ?? { missing: true }) });
  }
};

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < bootWait; i += 1) {
  await page.waitForTimeout(1000);
  if ((await page.locator(`[data-surface-id="${MAIN}"]`).count()) > 0) break;
}

await runPass("en");
const germanPicked = await setLocale(/Deutsch/u);
await note("locale-de", germanPicked && (await sample()).lang === "de", { germanPicked, lang: (await sample()).lang });
await runPass("de");

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] STATUS-STATES DONE ${results.steps.filter((s) => s.ok).length}/${results.steps.length} -> ${outDir}`);
await browser.close();
