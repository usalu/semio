/** ♿️ S18 item 7 audit probe: Chromium's own accessibility tree (CDP `Accessibility.getFullAXTree`, the third-party
 * oracle) of the `s` shell in en and de — interactive nodes without an accessible name, the document language, and
 * names that stay identical in both locales (untranslated candidates). Surfaces: signed-out Home, the hub workspace,
 * signed-in Home, Home's Create Space dialog.
 * usage: bun s18-a11y-audit.mjs <url> <out.json> */
import { writeFileSync } from "node:fs";
import { boot, dialog, openSessions, signIn, activate } from "../wp-c11/c11-lib.mjs";
const [url = "http://127.0.0.1:6540/", out = "generated/s18-14b-a11y.json"] = process.argv.slice(2);
const INTERACTIVE = new Set(["button", "link", "textbox", "searchbox", "combobox", "checkbox", "radio", "switch", "slider", "spinbutton", "tab", "menuitem", "menuitemcheckbox", "menuitemradio", "option", "treeitem", "gridcell", "row", "listbox", "tree", "grid", "tablist", "menu", "dialog", "tabpanel"]);

async function axSurface(page, label) {
  const cdp = await page.context().newCDPSession(page);
  const { nodes } = await cdp.send("Accessibility.getFullAXTree");
  await cdp.detach();
  const live = nodes.filter((node) => !node.ignored);
  const nameOf = (node) => (node.name?.value ?? "").trim();
  const roleOf = (node) => node.role?.value ?? "";
  const focusable = (node) => (node.properties ?? []).some((property) => property.name === "focusable" && property.value?.value === true);
  const unnamed = live.filter((node) => (INTERACTIVE.has(roleOf(node)) || focusable(node)) && !nameOf(node) && !["row", "gridcell", "tabpanel", "generic", "RootWebArea", "StaticText", "InlineTextBox"].includes(roleOf(node)));
  const lang = await page.evaluate(() => document.documentElement.lang);
  const names = [...new Set(live.filter((node) => INTERACTIVE.has(roleOf(node)) || ["heading", "status", "alert", "region", "navigation", "toolbar", "columnheader"].includes(roleOf(node))).map((node) => `${roleOf(node)}|${nameOf(node)}`).filter((entry) => !entry.endsWith("|")))];
  const unnamedDetail = [];
  for (const node of unnamed.slice(0, 40)) {
    let html = "";
    if (node.backendDOMNodeId) {
      const cdp2 = await page.context().newCDPSession(page);
      try {
        const { outerHTML } = await cdp2.send("DOM.getOuterHTML", { backendNodeId: node.backendDOMNodeId });
        html = outerHTML.slice(0, 220);
      } catch { html = "?"; }
      await cdp2.detach();
    }
    unnamedDetail.push({ role: roleOf(node), html });
  }
  return { label, lang, nodes: live.length, unnamedCount: unnamed.length, unnamed: unnamedDetail, names };
}

const report = { url, at: new Date().toISOString(), locales: {} };
for (const locale of ["en-US", "de-DE"]) {
  const { browser, sessions } = await openSessions([url], { locale });
  const [A] = sessions;
  const surfaces = [];
  try {
    await boot(A);
    await A.page.waitForTimeout(3_000);
    surfaces.push(await axSurface(A.page, "home-signed-out"));
    await A.page.locator('[data-semio-hub-sign-in=""]').first().click();
    await A.page.locator("[data-semio-hub-workspace]").waitFor({ state: "visible", timeout: 30_000 });
    await A.page.waitForTimeout(1_000);
    surfaces.push(await axSurface(A.page, "hub-workspace-sign-in"));
    await A.page.keyboard.press("Escape");
    await A.page.waitForTimeout(1_000);
    await signIn(A);
    await A.page.waitForTimeout(6_000);
    surfaces.push(await axSurface(A.page, "home-signed-in"));
    await activate(A.page, "s-home-create-space");
    await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
    await A.page.waitForTimeout(1_000);
    surfaces.push(await axSurface(A.page, "create-space-dialog"));
  } catch (error) {
    surfaces.push({ label: "error", error: String(error).slice(0, 400) });
  } finally {
    report.locales[locale] = { surfaces, pageerrors: A.lines.filter((line) => line.includes("pageerror")).length };
    await browser.close();
  }
}
const en = report.locales["en-US"]?.surfaces ?? [];
const de = report.locales["de-DE"]?.surfaces ?? [];
report.sameInBoth = en.map((surface) => {
  const other = de.find((candidate) => candidate.label === surface.label);
  if (!other?.names || !surface.names) return { label: surface.label, same: [] };
  const deNames = new Set(other.names);
  return { label: surface.label, same: surface.names.filter((name) => deNames.has(name) && /[A-Za-z]{3,}/u.test(name.split("|")[1] ?? "")) };
});
writeFileSync(out, JSON.stringify(report, null, 2));
for (const [locale, entry] of Object.entries(report.locales)) for (const surface of entry.surfaces) console.log(locale, surface.label, surface.error ?? `lang=${surface.lang} nodes=${surface.nodes} unnamed=${surface.unnamedCount}`);
for (const row of report.sameInBoth) console.log("SAME", row.label, row.same.length, JSON.stringify(row.same.slice(0, 25)));
