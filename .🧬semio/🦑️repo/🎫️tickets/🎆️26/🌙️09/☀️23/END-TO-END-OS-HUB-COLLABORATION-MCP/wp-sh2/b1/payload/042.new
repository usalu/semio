/** 🧩️ Semantic studio verification owner. */

import { mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { withDevServe } from "../🧮️program-matrix/🟦️.ts";
import { activate, boot, clickRowAction, dialog, faultsSince, openSessions, pageWindowedTables, settleHome, signIn, submitDialog, waitNamedRow, type Human, type Session } from "../👥️two-human/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";



//#region 🔖️SpaceE2eVerify
/** 🎭️ Playwright end-to-end workflow verification for the `s` studio shell (folded in from the former `.🧬semio/🦑️repo/🎫️tickets/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs`). */
const STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS = ["NoCompatibleDevice"];

function spaceE2eAssert(condition: boolean, message: string): void {
  if (!condition) throw new Error(message);
}

function isIgnorableStudioE2ePageError(message: string): boolean {
  return STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS.some((fragment) => message.includes(fragment));
}

async function waitForStudioE2eCondition(page: import("playwright").Page, predicate: (state: { text: string; children: number }) => boolean, label: string, deadline: number): Promise<{ text: string; children: number }> {
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const children = await page.locator("#root *").count().catch(() => 0);
    if (predicate({ text, children })) return { text, children };
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for ${label}`);
}

async function openStudioE2e(page: import("playwright").Page, deadline: number): Promise<{ text: string; children: number }> {
  await page.keyboard.press("Meta+n");
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const path = new URL(page.url()).pathname;
    const children = await page.locator("#root *").count().catch(() => 0);
    if (/Catalogue/i.test(text) && /Parameters/i.test(text) && path.startsWith("/spaces/")) {
      return { text, children };
    }
    await page.waitForTimeout(500);
  }
  throw new Error("timeout waiting for studio workspace");
}

async function activateStudioE2eWorkflowWindow(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(200);
}

async function expandStudioE2eWorkflowEngagement(page: import("playwright").Page): Promise<void> {
  await activateStudioE2eWorkflowWindow(page);
  await page.evaluate(() => document.getElementById("framework.window.sWorkflow.search.toggle")?.click());
  await page.waitForSelector("#s-media-catalogue-hint", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromEngagement(page: import("playwright").Page): Promise<string> {
  await expandStudioE2eWorkflowEngagement(page);
  const engagementInput = page.locator("#s-media-catalogue-hint");
  await engagementInput.fill("draw draw");
  await engagementInput.press("Enter");
  await page.waitForTimeout(1500);
  return "engagement";
}

async function openStudioE2eCommandPalette(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(100);
  await page.keyboard.press("Meta+p");
  await page.waitForSelector("[role='dialog'] [data-slot='command-input']", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromPalette(page: import("playwright").Page): Promise<string | null> {
  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("draw");
  await page.waitForTimeout(400);
  const drawSpawn = page
    .locator('[data-slot="command-item"]')
    .filter({ hasText: /Spawn Draw/i })
    .first();
  if (await drawSpawn.count()) {
    await drawSpawn.click();
    return "palette";
  }
  await page.keyboard.press("Escape");
  return null;
}

async function runStudioE2eVerify(baseUrl: string, timeoutMs: number): Promise<void> {
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const pageErrors: string[] = [];
  page.on("pageerror", (err) => pageErrors.push(String(err)));

  console.log(`navigating to ${baseUrl}`);
  await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.waitForFunction(() => /home/i.test(document.body.innerText) && /Demo Studio|New Studio/i.test(document.body.innerText) && document.querySelectorAll("#root *").length > 150, { timeout: 120_000 });

  const deadline = Date.now() + timeoutMs;
  const booted = await waitForStudioE2eCondition(page, ({ text }) => /Home/i.test(text) && /Studios|Search/i.test(text) && /Demo Studio|New Studio/i.test(text), "home shell with studios", deadline);
  console.log(`home loaded (${booted.children} nodes)`);
  spaceE2eAssert(/Demo Studio|Studios/i.test(booted.text), "home studios vfs should list seeded studio");

  await openStudioE2e(page, deadline);
  const pathAfterCreate = await page.evaluate(() => location.pathname);
  console.log(`studio loaded at ${pathAfterCreate}`);
  spaceE2eAssert(pathAfterCreate.startsWith("/spaces/"), "studio uri should be under /spaces/");

  await page.waitForFunction(() => document.querySelector(".semio-node-graph-host") != null, { timeout: 30_000 });

  const bodyText = await page.locator("body").innerText();
  spaceE2eAssert(!/Missing window:/i.test(bodyText), "all studio windows should render");
  spaceE2eAssert((await page.locator(".semio-node-graph-host").count()) > 0, "node graph host should render");
  spaceE2eAssert((await page.locator(".semio-text-editor-host").count()) > 0, "compiled dag editor should render");
  console.log("three studio windows rendered");

  let spawnMode: string | null = null;
  try {
    spawnMode = await spawnStudioE2eDrawFromEngagement(page);
    console.log(`spawn via ${spawnMode}`);
  } catch {
    spawnMode = await spawnStudioE2eDrawFromPalette(page);
    spaceE2eAssert(spawnMode === "palette", "draw spawn should work via engagement rail or command palette");
    console.log(`spawn via ${spawnMode}`);
  }

  await page.keyboard.press("Meta+z");
  await page.waitForTimeout(1500);
  console.log("undo issued");

  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("undo");
  await page.waitForTimeout(300);
  spaceE2eAssert((await page.locator('[data-slot="command-item"]').filter({ hasText: "Undo" }).count()) > 0, "undo should be in command palette");
  await paletteInput.fill("checkpoint");
  await page.waitForTimeout(300);
  spaceE2eAssert(
    (await page
      .locator('[data-slot="command-item"]')
      .filter({ hasText: /checkpoint/i })
      .count()) > 0,
    "checkpoint command should be in command palette",
  );
  console.log("studio commands in palette");
  await page.keyboard.press("Escape");

  await page.keyboard.press("Meta+f");
  await page.waitForTimeout(500);
  spaceE2eAssert((await page.locator("[role='dialog'] [data-slot='command-input']").count()) > 0, "find palette should open");
  console.log("find palette available");
  await page.keyboard.press("Escape");

  await page.getByRole("button", { name: "← Home" }).click({ force: true });
  await waitForStudioE2eCondition(page, ({ text }) => text.includes("Demo Studio") || text.includes("New Studio"), "home via studio bar", deadline);
  console.log("studio home bar navigation works");

  const demoStudioRow = page.locator('[data-row-id="studio:default"]');
  if (await demoStudioRow.count()) {
    await demoStudioRow.dblclick({ force: true });
    await page.waitForFunction(() => location.pathname.startsWith("/spaces/"), { timeout: 15_000 });
    await waitForStudioE2eCondition(page, ({ text }) => /Catalogue/i.test(text), "opened studio from home vfs", deadline);
    console.log("home vfs open studio works");
  }

  const criticalErrors = pageErrors.filter((message) => !isIgnorableStudioE2ePageError(message));
  if (criticalErrors.length !== pageErrors.length) {
    console.log(`ignored headless gpu errors: ${pageErrors.filter(isIgnorableStudioE2ePageError).join(" | ")}`);
  }
  spaceE2eAssert(criticalErrors.length === 0, `page errors: ${criticalErrors.join(" | ")}`);

  await browser.close();
  console.log("PASS: S studio end-to-end workflows verified");
}

export { STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS, activateStudioE2eWorkflowWindow, expandStudioE2eWorkflowEngagement, isIgnorableStudioE2ePageError, openStudioE2e, openStudioE2eCommandPalette, runStudioE2eVerify, spaceE2eAssert, spawnStudioE2eDrawFromEngagement, spawnStudioE2eDrawFromPalette, waitForStudioE2eCondition };

//#region 🔖️HomeE2e
/** 🏠️ The Home landing's studio catalog as a person drives it in the served React `s` shell (goal outcome 1, Home row of
 * the reachability census): the toolbar's Import Studio control opens the host file picker and the retained import job
 * lists the picked `.os` studio under its own name; the row's "Remove from Home" retires it (a Home config tombstone,
 * undoable, and unlisted from the host's local document catalog); a second imported studio is kept on this device, so a
 * reload reopens Home with the removal kept AND the kept studio listed again (the host re-hydrates it from its own folder
 * lane — a serve this harness starts gets a scratch `S_DATA_DIR`). With `--hub` the person also signs in, creates a hub space from
 * the toolbar's Create Space dialog, finds it again after the reload and deletes it. Every step is judged on the rendered
 * rows (windowed tables paged as a person scrolls) and the run fails on any fault line in the console.
 *
 * Promoted from the ticket probe `wp-sh2/sh2-home-e2e.mjs` (ticket 26/09/23, session 14). Binding a studio to a file
 * needs the host-owned local studio catalog (a wasm32 guest has no filesystem) and joins this journey with it.
 * @see ../👥️two-human/🟦️.ts — sessions, sign-in, dialogs and windowed-table rows
 * @see ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs — `HomeCatalogWork` */

/** 🎛️ One Home run; `human` is the hub credential (`null` = local-only, no hub steps). */
export type HomeE2eOptions = Readonly<{ baseUrl: string; hubUrl: string | null; human: Human | null; locale: "en" | "de"; tag: string; outDir: string; signal: AbortSignal }>;

/** 🧾️ One judged step of the journey. */
export type HomeE2eStep = { step: string; pass: boolean; detail: unknown };

/** 📊️ The run's report, rewritten after every step. */
export type HomeE2eReport = { tag: string; baseUrl: string; hubUrl: string | null; locale: string; startedAt: string; finishedAt?: string; steps: HomeE2eStep[]; faults: string[]; fatal?: string; cancelled?: boolean };

/** 🌐️ The Home labels a person reads, per locale. */
const HOME_E2E_LABELS = {
  en: { importStudio: /Import Studio/u, remove: /Remove from Home/u, removeLabel: "Remove from Home", delete: /^delete\b/iu },
  de: { importStudio: /Studio importieren/u, remove: /Aus Home entfernen/u, removeLabel: "Aus Home entfernen", delete: /^(löschen|delete)\b/iu },
} as const;

/** 📄️ A minimal authored `.os` studio manifest named `name` (the text an `export-studio-dsl` writes). */
export const homeE2eStudioText = (name: string): string =>
  `schema=s.space name="${name}" kind=atelier visibility=private programs=[ ] extensions=[ ]\nusers [id:TEXT name:TEXT avatar:TEXT role:ENUM] {\n  u1 "User u1" _ author\n}\ncollections [id:TEXT name:TEXT document-id:TEXT] {\n  c1 Main doc-c1\n}\n`;

/** 🔎️ Whether no Home row shows `name` any more (every windowed table paged), polled until `deadlineMs`. */
async function homeRowGone(page: Page, name: string, deadlineMs: number): Promise<boolean> {
  const deadline = Date.now() + deadlineMs;
  do {
    const present = await pageWindowedTables(page, () => page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements, wanted) => (elements.some((element) => (element.textContent ?? "").includes(wanted)) ? true : null), name));
    if (present === null) return true;
    await page.waitForTimeout(400);
  } while (Date.now() < deadline);
  return false;
}

/** 🏠️ Runs the Home journey in one headless browser, closed at the end; the report is flushed after every step and the
 * signal ends the run before its next step. */
export async function runHomeE2e(options: HomeE2eOptions): Promise<HomeE2eReport> {
  const outDir = join(options.outDir, options.tag);
  mkdirSync(outDir, { recursive: true });
  const labels = HOME_E2E_LABELS[options.locale];
  const report: HomeE2eReport = { tag: options.tag, baseUrl: options.baseUrl, hubUrl: options.hubUrl, locale: options.locale, startedAt: new Date().toISOString(), steps: [], faults: [] };
  const stamp = Date.now().toString(36);
  const spaceName = `Home E2E hub ${stamp}`;
  const studioName = `Home E2E import ${stamp}`;
  const studioFile = join(outDir, "studio.os");
  writeFileSync(studioFile, homeE2eStudioText(studioName));
  const keptName = `Home E2E kept ${stamp}`;
  const keptFile = join(outDir, "kept.os");
  writeFileSync(keptFile, homeE2eStudioText(keptName));
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
  const [session] = (await openSessions(browser, [options.baseUrl], [options.human ?? { label: "local", email: "", password: "" }], options.locale === "de" ? "de-DE" : "en-US")) as [Session];
  const page = session.page;
  await session.context.addInitScript(() => {
    const seen: string[] = [];
    const counted = new WeakSet<Element>();
    (globalThis as unknown as { __homeE2eNotices: string[] }).__homeE2eNotices = seen;
    const scan = (): void => {
      for (const element of document.querySelectorAll("[data-notice-code]")) {
        if (counted.has(element)) continue;
        counted.add(element);
        seen.push(element.getAttribute("data-notice-code") ?? "");
      }
    };
    const start = (): void => new MutationObserver(scan).observe(document.body, { subtree: true, childList: true, attributes: true });
    if (document.body) start();
    else document.addEventListener("DOMContentLoaded", start);
  });
  const noticesSeen = (): Promise<string[]> => page.evaluate(() => (globalThis as unknown as { __homeE2eNotices?: string[] }).__homeE2eNotices ?? []);
  const flush = (): void => {
    writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
    writeFileSync(join(outDir, "console.txt"), session.lines.join("\n"));
  };
  const step = async (name: string, run: () => Promise<{ pass: boolean; detail: unknown }>): Promise<boolean> => {
    if (options.signal.aborted) {
      report.cancelled = true;
      return false;
    }
    const cursor = session.lines.length;
    const judged = await run().catch((error: unknown) => ({ pass: false, detail: String(error instanceof Error ? error.message : error).slice(0, 400) }));
    const faults = faultsSince(session, cursor);
    const pass = judged.pass && faults.length === 0;
    report.steps.push({ step: name, pass, detail: faults.length === 0 ? judged.detail : { ...(judged.detail as object), faults } });
    console.log(`[home-e2e] ${name}: ${pass ? "PASS" : "FAIL"} ${JSON.stringify(report.steps.at(-1)!.detail)}`.slice(0, 1600));
    if (!pass) await page.screenshot({ path: join(outDir, `${name}.png`) }).catch(() => undefined);
    flush();
    return pass;
  };
  try {
    if (!(await step("boot", async () => (await boot(session), { pass: true, detail: page.url() })))) throw new Error("Home never mounted");
    await step("import-control", async () => {
      const text = await page.locator('[data-ui-node-key="s-home-import-studio"]').first().evaluate((element) => `${element.getAttribute("aria-label") ?? ""} ${element.textContent ?? ""}`).catch(() => "");
      return { pass: labels.importStudio.test(text), detail: { text: text.trim() } };
    });
    let spaceId: string | null = null;
    if (options.human !== null) {
      await step("sign-in", async () => (await signIn(session), { pass: await settleHome(session), detail: "signed in, Home settled" }));
      await step("create-hub-space", async () => {
        await activate(page, "s-home-create-space");
        await dialog(page).waitFor({ state: "visible", timeout: 20_000 });
        await page.locator("#name").fill(spaceName);
        await submitDialog(page);
        spaceId = await waitNamedRow(page, "space", spaceName, 120_000);
        return { pass: true, detail: { spaceId } };
      });
    }
    let studioId: string | null = null;
    if (await step("import-studio", async () => {
      const chooser = page.waitForEvent("filechooser", { timeout: 30_000 }).catch((error: unknown) => (error instanceof Error ? error : new Error(String(error))));
      await activate(page, "s-home-import-studio");
      const picker = await chooser;
      if (picker instanceof Error) throw picker;
      await picker.setFiles(studioFile);
      studioId = await waitNamedRow(page, "space", studioName, 60_000);
      const actions = await page.locator(`[data-ui-node-key="space:${studioId}"] button`).evaluateAll((elements) => elements.map((element) => `${element.getAttribute("aria-label") ?? ""} ${element.textContent ?? ""}`.trim()));
      return { pass: actions.some((action) => labels.remove.test(action)), detail: { studioId, actions, expected: labels.removeLabel } };
    })) {
      await step("remove-from-home", async () => {
        await clickRowAction(page, "space", studioId!, labels.remove);
        return { pass: await homeRowGone(page, studioName, 30_000), detail: { studioId } };
      });
    }
    let keptId: string | null = null;
    await step("import-kept-studio", async () => {
      const chooser = page.waitForEvent("filechooser", { timeout: 30_000 }).catch((error: unknown) => (error instanceof Error ? error : new Error(String(error))));
      await activate(page, "s-home-import-studio");
      const picker = await chooser;
      if (picker instanceof Error) throw picker;
      const before = (await noticesSeen()).length;
      await picker.setFiles(keptFile);
      keptId = await waitNamedRow(page, "space", keptName, 60_000);
      const settled = await page.waitForFunction((from) => ((globalThis as unknown as { __homeE2eNotices?: string[] }).__homeE2eNotices ?? []).slice(from).some((code) => code.startsWith("shell.localCatalog.") && code !== "shell.localCatalog.keeping"), before, { timeout: 45_000 }).then(() => true, () => false);
      const notices = (await noticesSeen()).slice(before);
      return { pass: settled && notices.includes("shell.localCatalog.kept"), detail: { keptId, notices } };
    });
    await step("reopen", async () => {
      await page.reload({ waitUntil: "domcontentloaded" });
      await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 });
      if (options.human !== null) await settleHome(session);
      const hubRow = spaceId === null ? null : await waitNamedRow(page, "space", spaceName, 60_000).then(() => true, () => false);
      const removedStaysRemoved = studioId === null ? null : await homeRowGone(page, studioName, 5_000);
      const keptAfterReload = keptId === null ? null : await waitNamedRow(page, "space", keptName, 60_000).then(() => true, () => false);
      return { pass: hubRow !== false && removedStaysRemoved !== false && keptAfterReload !== false, detail: { hubRow, removedStaysRemoved, keptAfterReload } };
    });
    if (spaceId !== null) {
      await step("delete-hub-space", async () => {
        await clickRowAction(page, "space", spaceId!, labels.delete);
        await dialog(page).waitFor({ state: "visible", timeout: 20_000 });
        const submittedAt = Date.now();
        await submitDialog(page);
        const live = await homeRowGone(page, spaceName, 60_000);
        const liveMs = Date.now() - submittedAt;
        if (live) return { pass: true, detail: { spaceId, liveMs } };
        await page.reload({ waitUntil: "domcontentloaded" });
        await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 });
        await settleHome(session);
        return { pass: false, detail: { spaceId, liveRemovedWithinMs: null, budgetMs: 60_000, goneAfterReload: await homeRowGone(page, spaceName, 5_000) } };
      });
    }
  } catch (error) {
    report.fatal = String(error instanceof Error ? error.stack ?? error.message : error).slice(0, 1200);
  } finally {
    report.faults = faultsSince(session, 0);
    report.finishedAt = new Date().toISOString();
    flush();
    await browser.close();
  }
  return report;
}

/** 🏠️ `verify home --serve <url> [--hub <url>] [--locale en|de] [--tag <t>] [--out <dir>]` — runs {@link runHomeE2e} against
 * the serve `--serve` names (reused, or started and stopped by {@link withDevServe}); with `--hub` the hub credential comes
 * from `OS_HUB_PROBE_EMAIL`/`OS_HUB_PROBE_PASSWORD` only (never argv, never logged) and a missing one reads `blocked`.
 * Writes `report.json` + `console.txt` under `<out>/<tag>/`, publishes the `home-e2e` acceptance record and exits
 * non-zero unless every step passes. */
export async function runHomeE2eCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const flag = (name: string): string | undefined => {
    const value = segments.indexOf(name) >= 0 ? segments[segments.indexOf(name) + 1] : undefined;
    return value === undefined || value.startsWith("--") ? undefined : value;
  };
  const serveUrl = flag("--serve");
  if (!serveUrl) throw new Error("usage: verify home --serve <url> [--hub <url>] [--locale en|de] [--tag <t>] [--out <dir>]");
  const hubUrl = flag("--hub") ?? null;
  const locale = flag("--locale") === "de" ? "de" : "en";
  const tag = flag("--tag") ?? `home-${locale}${hubUrl === null ? "-local" : "-hub"}`;
  const startedAt = new Date();
  const email = process.env.OS_HUB_PROBE_EMAIL;
  const password = process.env.OS_HUB_PROBE_PASSWORD;
  if (hubUrl !== null && (!email || !password)) {
    publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({ check: "home-e2e", status: "blocked", startedAt, measured: { hub: hubUrl, credential: false }, summary: { en: "a hub run needs OS_HUB_PROBE_EMAIL and OS_HUB_PROBE_PASSWORD", de: "ein Lauf mit Hub braucht OS_HUB_PROBE_EMAIL und OS_HUB_PROBE_PASSWORD" } }));
    process.exitCode = 1;
    return;
  }
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const outDir = resolve(flag("--out") ?? defaultOutDir);
  if (!process.env.S_DATA_DIR) {
    process.env.S_DATA_DIR = join(outDir, tag, "data");
    mkdirSync(process.env.S_DATA_DIR, { recursive: true });
  }
  await withAcceptanceRecord(repoRoot, "home-e2e", () => withDevServe(repoRoot, "home-e2e", { serveUrl, ...(hubUrl === null ? {} : { hubUrl }), locale, signal: controller.signal, startedAt }, async (baseUrl) => {
    const report = await runHomeE2e({ baseUrl, hubUrl, human: hubUrl === null ? null : { label: "user", email: email!, password: password! }, locale, tag, outDir, signal: controller.signal });
    const passed = report.steps.filter((row) => row.pass).length;
    const total = report.steps.length;
    const failing = report.steps.filter((row) => !row.pass).map((row) => row.step);
    const unreachable = /ERR_CONNECTION_REFUSED|ECONNREFUSED/u.test(report.fatal ?? "");
    const status = unreachable ? "blocked" : report.fatal || report.cancelled || failing.length > 0 || report.faults.length > 0 ? "fail" : "pass";
    publishAcceptanceCheckResult(
      repoRoot,
      acceptanceCheckResult({
        check: "home-e2e",
        status,
        startedAt,
        measured: { locale, hub: hubUrl !== null, steps: total, passed, failed: total - passed, faults: report.faults.length, cancelled: Boolean(report.cancelled), fatal: Boolean(report.fatal) },
        summary: {
          en: `${passed}/${total} Home steps pass in ${locale}${hubUrl === null ? " (local only)" : " (hub)"}${failing.length ? `; failing: ${failing.join(", ")}` : ""}${report.faults.length ? `; ${report.faults.length} fault line(s)` : ""}${report.fatal ? `; fatal: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
          de: `${passed}/${total} Home-Schritte bestehen in ${locale}${hubUrl === null ? " (nur lokal)" : " (Hub)"}${failing.length ? `; fehlgeschlagen: ${failing.join(", ")}` : ""}${report.faults.length ? `; ${report.faults.length} Fehlerzeile(n)` : ""}${report.fatal ? `; Abbruch: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
        },
        evidence: [join(outDir, tag, "report.json"), join(outDir, tag, "console.txt")],
      }),
    );
    console.log(`[home-e2e] === ${tag}: PASS ${passed}/${total} → ${join(outDir, tag)} ===`);
    if (status !== "pass") process.exitCode = 1;
  }));
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
}
//#endregion 🔖️HomeE2e
