// #region 🧲️Header
// 💻️ .storybook/os-plugins.spec.ts
// Specs: End-to-end readiness checks for the `framework/os` scope's per-plugin boot matrix (`.storybook/stories/framework/os/plugins.stories.tsx`).
// Summary: For every `PLUGIN_BUILD_TARGETS` entry, navigates `iframe.html?id=<plugins-story-id>&viewMode=story` and waits for the shell's readiness beacon (`semioOsReady`/`semioOsError` dataset keys — see `#region 🔖️ReadinessBeacon` in `framework/os/renderer/js/react/index.tsx`) for plugins with a prebuilt artifact, or the `OsBootHost` artifact-missing panel for the ones deliberately without one (`framework/os/dev/plugin-modules/`) — that panel never mounts `FrameworkOsShell`, so no beacon fires for it. Asserts zero unexpected `console.error` per story (model: `.storybook/puzzle-2d.spec.ts`).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Page } from "@playwright/test";

import { PLUGIN_BUILD_TARGETS, pluginModuleUrl, type PluginBuildTarget } from "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts";

const PLUGINS_STORY_TITLE_ID = "🛠️framework🖥️os-plugins";
const READY_TIMEOUT_MS = 60_000;

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

// #region 🔖️ArtifactAvailability
/** @emoji 🔍️ Same HEAD-probe `OsBootHost` (`.storybook/framework/os/index.tsx`) itself does — mirrored here
 * so the spec knows, per target, whether to expect the readiness beacon or the artifact-missing panel. */
async function pluginArtifactAvailable(page: Page, target: PluginBuildTarget): Promise<boolean> {
  const moduleUrl = pluginModuleUrl(target.pluginId, target.wasmOut);
  const res = await page.request.head(moduleUrl).catch(() => undefined);
  return !!res?.ok();
}
// #endregion 🔖️ArtifactAvailability

type OsBeaconOutcome = "ready" | "error";

async function waitForOsBeaconOrArtifactMissing(page: Page, pluginId: string, expectArtifact: boolean): Promise<OsBeaconOutcome | "artifact-missing"> {
  if (!expectArtifact) {
    await expect(page.getByText("plugin artifact missing", { exact: false })).toBeVisible({ timeout: READY_TIMEOUT_MS });
    return "artifact-missing";
  }
  await page.waitForFunction(
    (id) => {
      const root = document.documentElement;
      return root.dataset.semioOsReady === id || root.dataset.semioOsError === id;
    },
    pluginId,
    { timeout: READY_TIMEOUT_MS },
  );
  const outcome = await page.evaluate((id) => (document.documentElement.dataset.semioOsReady === id ? "ready" : "error"), pluginId);
  return outcome as OsBeaconOutcome;
}

for (const target of PLUGIN_BUILD_TARGETS) {
  test(`framework/os program matrix: ${target.pluginId} reaches a deterministic boot outcome`, async ({ page }) => {
    const pageErrors: Error[] = [];
    const consoleErrors: string[] = [];
    page.on("pageerror", (error) => pageErrors.push(error));
    page.on("console", (message) => {
      if (message.type() === "error") consoleErrors.push(message.text());
    });

    const expectArtifact = await pluginArtifactAvailable(page, target);
    const storyId = `${PLUGINS_STORY_TITLE_ID}--${target.pluginId}`;

    await page.goto(`iframe.html?id=${encodeURIComponent(storyId)}&viewMode=story`, { waitUntil: "domcontentloaded" });
    await expect(page.locator("body")).not.toContainText("Couldn't find story matching");

    const outcome = await waitForOsBeaconOrArtifactMissing(page, target.pluginId, expectArtifact);
    if (expectArtifact) {
      expect(outcome, `${target.pluginId}: expected the shell to reach "ready" (had a prebuilt artifact at ${pluginModuleUrl(target.pluginId, target.wasmOut)})`).toBe("ready");
    } else {
      expect(outcome).toBe("artifact-missing");
    }

    expect(pageErrors.map((error) => error.message)).toEqual([]);
    expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  });
}

//#region 🧪️WgpuStoryAdapter
test("wgpu glass-content story receives its plugin argument and reaches a declared host outcome", async ({ page }) => {
  const pageErrors: Error[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  await page.goto(`iframe.html?id=${encodeURIComponent("🛠️framework🖥️os-wgpu--glass-content")}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.getByTestId("wgpu-silhouette-visual-floor")).toBeVisible();
  await expect(page.locator("body")).not.toContainText("unknown program");
  expect(pageErrors.map((error) => error.message)).toEqual([]);
});
//#endregion 🧪️WgpuStoryAdapter
