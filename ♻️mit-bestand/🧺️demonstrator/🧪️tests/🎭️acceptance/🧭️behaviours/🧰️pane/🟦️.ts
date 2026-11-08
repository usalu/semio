// #region 🧲️Header
// 🧰️ Pane helpers of the behaviour acceptance: the baseline suite's selector and readiness helpers, restated here
// because importing `../../🟦️.ts` would register its 25 cases inside this file and change the baseline count.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { expect, test as browserTest, type Locator, type Page } from "@playwright/test";
import { significantConsoleErrors } from "../../🧬️console/🟦️.ts";
import { assertDemonstratorE2eArtifactPath } from "../../../../🔨️modules/🧪️e2e/📁️artifacts/🟦️.ts";
import artifactPolicy from "../../../../🔨️modules/🧪️e2e/📁️artifacts/🔣️.json" with { type: "json" };
// #endregion 🔌️Adapters

export const SHELL_READY_TIMEOUT_MS = 120_000;
export const TEST_TIMEOUT_MS = 240_000;
export const SURFACE_CONTENT_TIMEOUT_MS = 60_000;

const demonstratorRoot = join(import.meta.dirname, "..", "..", "..", "..");

/** 🩺️ The baseline error policy for every behaviour case: page errors and significant console errors fail it. */
export const test = browserTest.extend<{ browserHealth: void }>({
  browserHealth: [async ({ page }, use, testInfo) => {
    expect(testInfo.title.length, "bounded diagnostic title").toBeLessThanOrEqual(artifactPolicy.maxTitleChars);
    for (const name of ["trace.zip", "error-context.md", "p.png"]) {
      assertDemonstratorE2eArtifactPath(join(testInfo.outputDir, name));
      assertDemonstratorE2eArtifactPath(testInfo.outputPath(name));
    }
    const pageErrors: string[] = [];
    const consoleErrors: string[] = [];
    page.on("pageerror", (error) => pageErrors.push(error.message));
    page.on("console", (message) => {
      if (message.type() === "error") consoleErrors.push(message.text());
    });
    try {
      await use();
    } finally {
      expect.soft(pageErrors, "unexpected browser page errors").toEqual([]);
      expect.soft(significantConsoleErrors(consoleErrors), "unexpected browser console errors").toEqual([]);
    }
  }, { auto: true }],
});

//#region 🌐️Languages
export type Language = "en" | "de";
export type Wording = Readonly<Record<Language, string>>;
const catalog = JSON.parse(readFileSync(join(demonstratorRoot, "🔨️modules", "🌐️locale", "🔣️.json"), "utf8")) as { readonly texts: Readonly<Record<string, Wording>> };

/** 🗣️ The explicit project language; the browser locale of the project is stated by the shared config. */
export function projectLanguage(): Language {
  const name = test.info().project.name;
  if (name !== "en" && name !== "de") throw new Error("Behaviour acceptance requires an explicit en or de project");
  return name;
}

/** 🗣️ One wording of the page's own catalog in the project language. */
export function catalogText(key: string): string {
  const cells = catalog.texts[key];
  if (cells === undefined) throw new Error(`unknown demonstrator text ${key}`);
  return cells[projectLanguage()];
}

/** 🗣️ One wording of a case table in the project language. */
export const wording = (cells: Wording): string => cells[projectLanguage()];
//#endregion 🌐️Languages

//#region 🆔️ElementId
/** 🆔️ Mirror of `🆔️ElementId`'s `elementIdSegment`, as in the baseline suite. */
export function elementIdSegment(raw: string): string {
  let segment = "";
  let capitalizeNext = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") {
      capitalizeNext = true;
      continue;
    }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (segment.length === 0) segment += ch.toLowerCase();
    else if (capitalizeNext) {
      segment += ch.toUpperCase();
      capitalizeNext = false;
    } else segment += ch;
  }
  return segment;
}

/** 🪟️ `framework.window.<camelKindId>` of a window kind or instance id. */
export const windowElementId = (kindId: string): string => `framework.window.${elementIdSegment(kindId)}`;

/** 🎯️ An element by real id or `data-element-alias` token, scoped to one pane's shell. */
export function paneElementSelector(paneId: string, elementId: string): string {
  return `[data-shell-id="${paneId}"] [id="${elementId}"], [data-shell-id="${paneId}"] [data-element-alias~="${elementId}"]`;
}

/** 🪟️ A pane's window root by kind or instance id. */
export const paneWindow = (page: Page, paneId: string, windowId: string): Locator => page.locator(paneElementSelector(paneId, windowElementId(windowId)));
//#endregion 🆔️ElementId

//#region 🚦️ShellReadiness
/** 🚦️ Waits for one pane shell to report an outcome and asserts the language contract on its root. */
export async function waitForPaneShellOutcome(page: Page, paneId: string): Promise<"ready" | "error" | "notFound"> {
  await page.waitForFunction(
    (id) => {
      const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement | null;
      return !!el && (el.dataset.shellReady !== undefined || el.dataset.shellError !== undefined || el.dataset.shellNotFound !== undefined);
    },
    paneId,
    { timeout: SHELL_READY_TIMEOUT_MS },
  );
  await expect(page.locator("html")).toHaveAttribute("lang", projectLanguage());
  await expect(page.locator(`[data-shell-id="${paneId}"]`)).toHaveAttribute("lang", projectLanguage());
  return page.evaluate((id) => {
    const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement;
    if (el.dataset.shellReady !== undefined) return "ready";
    if (el.dataset.shellError !== undefined) return "error";
    return "notFound";
  }, paneId) as Promise<"ready" | "error" | "notFound">;
}

/** 👋️ Dismisses the pane's introduction overlay by click, else by its documented Escape route. Absence is not an error. */
export async function dismissIntroductionIfPresent(page: Page, paneId: string): Promise<void> {
  const skip = page.locator(paneElementSelector(paneId, "ui.introduction.skip"));
  if (!(await skip.first().isVisible({ timeout: 3_000 }).catch(() => false))) return;
  const clicked = await skip.first().click({ timeout: 20_000 }).then(() => true).catch(() => false);
  if (!clicked) await page.keyboard.press("Escape");
  await expect(skip, "the introduction overlay must be dismissable").toHaveCount(0, { timeout: 15_000 });
}

/** 🚪️ Enters a pane through its hash route, requires `ready` and dismisses the introduction. */
export async function enterPane(page: Page, paneId: string): Promise<void> {
  await page.goto(`/#${paneId}`, { waitUntil: "domcontentloaded" });
  expect(await waitForPaneShellOutcome(page, paneId), `pane "${paneId}": expected its shell to reach "ready"`).toBe("ready");
  await dismissIntroductionIfPresent(page, paneId);
}
//#endregion 🚦️ShellReadiness

//#region 🪟️Surfaces
/** 📸️ Retains a short PNG of one pane shell. */
export async function capturePaneArtifact(page: Page, paneId: string, name = "p.png"): Promise<void> {
  if (!/^[a-z][a-z0-9-]{0,15}\.png$/.test(name)) throw new Error("Use a short semantic PNG artifact name");
  assertDemonstratorE2eArtifactPath(join(test.info().outputDir, name));
  const body = await page.locator(`[data-shell-id="${paneId}"]`).screenshot({ path: test.info().outputPath(name), animations: "disabled" });
  await test.info().attach(`${paneId}-${name.slice(0, -4)}`, { contentType: "image/png", body });
}

const jsonArrayLength = (raw: string | null): number => {
  try {
    const parsed = raw ? (JSON.parse(raw) as unknown) : [];
    return Array.isArray(parsed) ? parsed.length : 0;
  } catch {
    return 0;
  }
};

/** 🌍️ The world host of a window, waited until it carries a non-empty scene. */
export async function worldHost(container: Locator): Promise<Locator> {
  const host = container.locator(".semio-world-3d-host");
  await expect(host.first()).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  await expect.poll(async () => jsonArrayLength(await host.first().getAttribute("data-meshes-json")) + jsonArrayLength(await host.first().getAttribute("data-instances-json")), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toBeGreaterThan(0);
  return host.first();
}

/** ⏱️ Waits for the production compute chrome of a world window to leave its busy and failed phases. */
export async function waitWorldSettled(container: Locator): Promise<void> {
  await worldHost(container);
  await expect(container.locator('[data-slot="world-compute-status"][aria-busy="true"]')).toHaveCount(0, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(container.locator('[data-slot="world-compute-status"]')).not.toHaveAttribute("data-compute-phase", /cancel|fault|fail|error/i);
}

/** 🧩️ The raw `data-instances-json` of a window, `[]` while it carries no world host. */
export async function instanceSnapshot(container: Locator): Promise<string> {
  const host = container.locator(".semio-world-3d-host");
  return (await host.count()) === 0 ? "[]" : (await host.first().getAttribute("data-instances-json")) ?? "[]";
}

/** 🆔️ The sorted distinct instance ids a window projects. */
export async function worldInstanceIds(container: Locator): Promise<readonly string[]> {
  return [...new Set((JSON.parse(await instanceSnapshot(container)) as { readonly id: string }[]).map((instance) => instance.id))].sort();
}

/** 🧱️ Geometry of a window without ephemeral interaction paint. */
export async function documentWorldProjection(container: Locator): Promise<string> {
  return container.locator(".semio-world-3d-host").first().evaluate((host) => JSON.stringify({
    meshes: JSON.parse(host.getAttribute("data-meshes-json") ?? "[]"),
    instances: (JSON.parse(host.getAttribute("data-instances-json") ?? "[]") as Record<string, unknown>[]).map(({ selected, hovered, highlighted, ...instance }) => instance),
  }));
}

/** 🌀️ The number of connection-point markers a window projects. */
export async function vortexCount(container: Locator): Promise<number> {
  const host = container.locator(".semio-world-3d-host");
  return (await host.count()) === 0 ? 0 : jsonArrayLength(await host.first().getAttribute("data-vortices-json"));
}

/** ↩️ Undo or redo through the pane's keyboard route after focusing its scene canvas. */
export async function historyGesture(page: Page, container: Locator, direction: "undo" | "redo"): Promise<void> {
  await container.locator("canvas").first().click({ position: { x: 8, y: 8 } });
  await page.keyboard.press(direction === "undo" ? "ControlOrMeta+z" : "ControlOrMeta+Shift+z");
}
//#endregion 🪟️Surfaces

//#region 🧭️PanelChrome
/** 🗂️ Opens a top-level document tab (`framework.panel.<id>`) and returns its panel body. */
export async function openDocumentTab(page: Page, paneId: string, tabId: string): Promise<Locator> {
  const tab = page.locator(`[data-shell-id="${paneId}"] [data-tab-id="framework.panel.${tabId}"]:visible`).first();
  if ((await tab.getAttribute("aria-pressed")) !== "true") await tab.click();
  return page.locator(paneElementSelector(paneId, `framework.panelTab.framework.panel.${tabId}`));
}

/** 🧭️ Opens a tool or display tab through its category chip, as a user would. */
export async function openPaneTab(page: Page, paneId: string, categoryId: string, tabId: string): Promise<void> {
  const shell = page.locator(`[data-shell-id="${paneId}"]`);
  const tab = shell.locator(`[data-tab-id="${tabId}"]:visible`).first();
  if (!(await tab.isVisible())) await shell.locator(`[data-tab-id="${categoryId}"]:visible`).first().click();
  if ((await tab.getAttribute("aria-pressed")) !== "true") await tab.click();
}

/** 🎚️ Unfolds the window options pane of one window when it is folded. */
export async function openWindowOptions(page: Page, paneId: string, windowId: string): Promise<void> {
  const unfold = page.locator(paneElementSelector(paneId, `${windowElementId(windowId)}.measures.unfold`));
  if (await unfold.first().isVisible().catch(() => false)) await unfold.first().click();
}

/** 🪪️ The newest framework tool-run group id rendered in a pane, `""` while none exists. */
export async function latestToolRunGroup(page: Page, paneId: string): Promise<string> {
  return page.locator(`[data-shell-id="${paneId}"] [id], [data-shell-id="${paneId}"] [data-element-alias]`).evaluateAll((elements) => {
    const runs = elements.flatMap((element) => [element.id, ...(element.getAttribute("data-element-alias") ?? "").split(/\s+/)])
      .flatMap((id) => /^framework\.toolRun\.(\d+)(?:\.|$)/.exec(id)?.[1] ?? []);
    const latest = runs.reduce((highest, run) => BigInt(run) > BigInt(highest) ? run : highest, "0");
    return runs.length ? `framework.toolRun.${latest}` : "";
  });
}

/** 🔽️ Picks one option of a window-measure or staged-argument select by its `data-value`. */
export async function pickSelectValue(page: Page, trigger: Locator, value: string): Promise<void> {
  await trigger.click();
  await page.locator(`[data-slot="select-item"][data-value="${value}"]`).last().click();
}
//#endregion 🧭️PanelChrome
