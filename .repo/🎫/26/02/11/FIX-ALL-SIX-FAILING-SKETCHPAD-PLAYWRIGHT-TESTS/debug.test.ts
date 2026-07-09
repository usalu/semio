import { test, expect } from "@playwright/test";
test("debug", async ({ page }) => {
  await page.goto("/");
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(5000);
  const bodyText = await page.evaluate(() => document.body.innerText);
  console.log("=== BODY TEXT ===");
  console.log(bodyText.slice(0, 2000));
  console.log("=== IDs ON PAGE ===");
  const ids = await page.evaluate(() => {
    return Array.from(document.querySelectorAll("[id]"))
      .map((el) => el.id)
      .filter((id) => id.includes("compose"))
      .join("\n");
  });
  console.log(ids);
  console.log("=== NAVBAR ===");
  const navbar = await page
    .locator('[id="compose.sketchpad.navbar"]')
    .isVisible()
    .catch(() => false);
  console.log("Navbar visible:", navbar);
  console.log("=== FOOTER ===");
  const footer = await page
    .locator("footer")
    .isVisible()
    .catch(() => false);
  console.log("Footer visible:", footer);
  console.log("=== TOOLBAR ===");
  const toolbar = await page
    .locator('[id="compose.sketchpad.toolbar"]')
    .isVisible()
    .catch(() => false);
  console.log("Toolbar visible:", toolbar);
  console.log("=== IMPORT KIT ===");
  const importKit = await page.locator('[id="compose.sketchpad.app.home.importKit"]').count();
  console.log("Import kit count:", importKit);
  console.log("=== PANEL TOGGLES ===");
  const toggles = await page.evaluate(() => {
    return Array.from(document.querySelectorAll('[id*="panelToggle"]'))
      .map((el) => `${el.id} visible:${(el as HTMLElement).offsetParent !== null}`)
      .join("\n");
  });
  console.log(toggles || "(none)");
  console.log("=== ERROR OVERLAYS / CONSOLE ERRORS ===");
  const viteError = await page.locator("vite-error-overlay").count();
  console.log("Vite error overlays:", viteError);
  const shadowError = await page.evaluate(() => {
    const overlay = document.querySelector("vite-error-overlay");
    if (overlay && overlay.shadowRoot) {
      return overlay.shadowRoot.textContent?.slice(0, 500) || "";
    }
    return "";
  });
  console.log("Shadow error:", shadowError);
  console.log("=== ALL ELEMENTS WITH COMPOSE IDs ===");
  const allComposeIds = await page.evaluate(() => {
    return Array.from(document.querySelectorAll('[id*="compose"]'))
      .map((el) => `${el.tagName}#${el.id}`)
      .join("\n");
  });
  console.log(allComposeIds || "(none)");
  console.log("=== ROOT INNER HTML (first 3000 chars) ===");
  const rootHtml = await page.evaluate(() => {
    const root = document.getElementById("root");
    return root ? root.innerHTML.slice(0, 3000) : "(root not found)";
  });
  console.log(rootHtml);
});
