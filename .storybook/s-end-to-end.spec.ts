// #region 🧲️Header
// 💻️ .storybook/s-end-to-end.spec.ts
// Specs: End-to-end acceptance for `s` — semio's OS host shell (plugin id `s`, backed by `semio-s-plugin-space`).
// Summary: `os-plugins.spec.ts` proves every plugin reaches *a deterministic boot outcome*, which a failed boot also satisfies. This spec makes the stronger claim `s` alone has to meet: it boots to READY (never `semioOsError`), it renders the shell's structural landmarks, and it is INTERACTIVE — the command palette opens on Ctrl/Cmd+K and closes on Escape, and the shell answers a context-menu gesture. Assertions are structural (`data-*`/`role`), never text, because the UI is multi-language with no default language.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Locator, type Page } from "@playwright/test";

const S_STORY_ID = "🛠️framework🖥️os-plugins--s";
const S_PLUGIN_ID = "s";
const READY_TIMEOUT_MS = 60_000;

/** @emoji 🔇️ Same filter `os-plugins.spec.ts` and `puzzle-2d.spec.ts` use: a 404 for an optional asset is not a shell defect. */
function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

type BootOutcome = "ready" | "error" | "not-found" | "artifact-missing";

/** @emoji 🚦️ Reads the shell's readiness beacon (`#region 🔖️ReadinessBeacon` in `ShellHost/🟦️.tsx`), which stamps the pluginId onto exactly one of three dataset keys on `<html>`. */
async function bootOutcome(page: Page, pluginId: string): Promise<BootOutcome> {
  await page.waitForFunction(
    (id) => {
      const root = document.documentElement;
      return root.dataset.semioOsReady === id || root.dataset.semioOsError === id || root.dataset.semioOsNotFound === id;
    },
    pluginId,
    { timeout: READY_TIMEOUT_MS },
  );
  return page.evaluate((id) => {
    const root = document.documentElement;
    if (root.dataset.semioOsReady === id) return "ready" as const;
    if (root.dataset.semioOsError === id) return "error" as const;
    return "not-found" as const;
  }, pluginId);
}

async function openSStory(page: Page): Promise<{ readonly scope: Locator; readonly consoleErrors: string[]; readonly pageErrors: Error[] }> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto(`iframe.html?id=${encodeURIComponent(S_STORY_ID)}&viewMode=story`, { waitUntil: "domcontentloaded" });

  // 🧭️ `s` is a host variant with a prebuilt artifact, so the artifact-missing panel must NOT appear —
  // if it does, the plugin fleet did not materialize and the rest of this spec would assert nothing.
  await expect(page.getByText("plugin artifact missing", { exact: false })).toHaveCount(0);

  const outcome = await bootOutcome(page, S_PLUGIN_ID);
  expect(outcome, `s must boot READY, got "${outcome}"`).toBe("ready");

  const scope = page.locator(".semio-scope[data-shell-id]").first();
  await expect(scope).toBeVisible();
  return { scope, consoleErrors, pageErrors };
}

test.describe("s boots and is interactive", () => {
  test("s reaches the ready beacon and renders the shell's structural landmarks", async ({ page }) => {
    const { scope, consoleErrors, pageErrors } = await openSStory(page);

    // 🏗️ The three landmarks every booted shell owns, asserted structurally rather than by text.
    await expect(scope.locator("[data-level='base']").first()).toBeVisible();
    await expect(page.locator("[data-semio-portal-layer]").first()).toBeAttached();

    // 🏷️ The navbar's app identity slot must render and be non-empty — its *content* is
    // language-dependent, so only its presence and non-emptiness are asserted.
    const appName = scope.locator("[data-slot='app-name']").first();
    await expect(appName).toBeVisible();
    expect((await appName.innerText()).trim().length).toBeGreaterThan(0);

    // 🪞️ The beacon is mirrored onto the shell's own root, so a nested shell can be identified.
    await expect(scope).toHaveAttribute("data-shell-ready", S_PLUGIN_ID);

    expect(pageErrors, `uncaught page errors: ${pageErrors.map((e) => e.message).join(" | ")}`).toEqual([]);
    expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  });

  test("s answers keyboard input: the command palette opens on Ctrl/Cmd+K and closes on Escape", async ({ page }) => {
    const { scope, consoleErrors, pageErrors } = await openSStory(page);

    await scope.click({ position: { x: 8, y: 8 } });

    const palette = page.locator("[role='dialog']").first();
    await expect(palette).toHaveCount(0);

    await page.keyboard.press("ControlOrMeta+KeyK");
    await expect(palette).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(palette).toBeHidden();

    expect(pageErrors, `uncaught page errors: ${pageErrors.map((e) => e.message).join(" | ")}`).toEqual([]);
    expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  });

  test("s answers pointer input: a context-menu gesture opens the shell's own menu", async ({ page }) => {
    const { scope, consoleErrors, pageErrors } = await openSStory(page);

    await scope.click({ button: "right", position: { x: 24, y: 120 } });

    await expect(page.locator("[role='menu']").first()).toBeVisible();

    await page.keyboard.press("Escape");

    expect(pageErrors, `uncaught page errors: ${pageErrors.map((e) => e.message).join(" | ")}`).toEqual([]);
    expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  });
});
