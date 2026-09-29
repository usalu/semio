/** 🌱️ S18 §14c p33: Home + directory for a user on a clean device profile, per locale — sign in through the shell, the
 * directory bootstrap timeline, every Home space row, a first space created from Home, the space index it opens (its
 * directory rows and the kind picker's labels), Home again after a reload, and the fault lines of the whole journey.
 * usage: bun s18-probe-fresh-home.mjs <url> <locale> <userIndex> <captureDir> */
import { USERS, activate, boot, dialog, openSessions, signIn, submitDialog } from "../wp-c11/c11-lib.mjs";
import { join } from "node:path";
const [url = "http://127.0.0.1:6540/", locale = "en-US", userIndex = "1", captures = "."] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url], { locale, users: [USERS[Number(userIndex)]] });
const [A] = sessions;
const tag = `fresh-${locale}`;
const bootstrap = () => A.page.evaluate(() => [...document.querySelectorAll("[data-directory-bootstrap]")].map((e) => `${e.getAttribute("data-directory-bootstrap")}:${e.getAttribute("data-directory-bootstrap-code") ?? ""}:${(e.textContent ?? "").trim().slice(0, 60)}`));
const alerts = () => A.page.evaluate(() => [...document.querySelectorAll('[role="alert"], [role="status"]')].map((e) => (e.textContent ?? "").replace(/\s+/gu, " ").trim()).filter(Boolean).slice(0, 8));
const homeRows = () => A.page.evaluate(async () => {
  const out = new Map();
  for (const scroller of document.querySelectorAll('[data-slot="table-window-scroll"]')) {
    for (let top = 0; top < 20000; top += 200) {
      scroller.scrollTop = top;
      await new Promise((resolve) => setTimeout(resolve, 150));
      for (const row of document.querySelectorAll('[data-ui-node-key^="space:"]')) out.set(row.getAttribute("data-ui-node-key"), (row.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60));
      if (scroller.scrollTop + scroller.clientHeight >= scroller.scrollHeight) break;
    }
    scroller.scrollTop = 0;
  }
  return [...out];
});
const timeline = [];
const sample = async (label, seconds) => {
  for (let i = 0; i < seconds; i += 1) {
    const now = JSON.stringify(await bootstrap());
    if (timeline.at(-1)?.now !== now) timeline.push({ at: `${label}+${i}s`, now });
    await A.page.waitForTimeout(1_000);
  }
};
const verdict = {};
try {
  await boot(A);
  verdict.bootLang = await A.page.evaluate(() => document.documentElement.lang);
  verdict.createSpaceLabel = await A.page.locator('[data-ui-node-key="s-home-create-space"]').first().evaluate((e) => e.getAttribute("aria-label") ?? (e.textContent ?? "").trim());
  await sample("boot", 3);
  await signIn(A);
  await sample("signed-in", 20);
  const before = await homeRows();
  verdict.homeRowsAtSignIn = before.length;
  verdict.homeStatus = await alerts();
  await A.page.screenshot({ path: join(captures, `s18-14c-p33-${tag}-home.png`) });
  const name = `S18 Fresh ${locale} ${Date.now().toString(36)}`;
  await activate(A.page, "s-home-create-space");
  await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
  verdict.dialogTitle = await dialog(A.page).evaluate((e) => (e.querySelector("h2, [data-slot='dialog-title']")?.textContent ?? "").trim());
  await A.page.locator("#name").fill(name);
  await submitDialog(A.page);
  let created = null;
  for (let i = 0; i < 30 && !created; i += 1) {
    await A.page.waitForTimeout(2_000);
    created = (await homeRows()).find(([, text]) => text.includes(name)) ?? null;
  }
  verdict.spaceCreated = created !== null;
  if (created) {
    await activate(A.page, created[0]);
    const spaceId = created[0].slice("space:".length);
    await A.page.waitForFunction((id) => location.pathname.includes(`/spaces/${id}`) && [...document.querySelectorAll("[data-window-id]")].some((e) => e.getAttribute("data-window-id") !== "s-home-main"), spaceId, { timeout: 180_000 }).catch(() => undefined);
    verdict.spaceRoute = await A.page.evaluate(() => location.pathname);
    await sample("space", 20);
    verdict.spaceWindows = await A.page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((e) => e.getAttribute("data-window-id")));
    verdict.indexRows = await A.page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"] [data-ui-node-key^="artifact:"]')].length);
    verdict.indexStatus = await alerts();
    await A.page.evaluate(() => { for (const element of document.querySelectorAll('[data-slot="window-action-pane"] [aria-expanded="false"]')) if (/create artifact|artefakt anlegen/iu.test(element.textContent ?? "")) element.click(); });
    await A.page.locator('[data-slot="window-action-pane"] [id="action.createArtifact"]').first().click({ force: true }).catch(() => undefined);
    await A.page.waitForTimeout(1_500);
    const trigger = A.page.locator('[data-slot="window-action-pane"] [id$=".arg.kindChoice"] [role="combobox"], [data-slot="window-action-pane"] button#kindChoice').first();
    if ((await trigger.count()) > 0) {
      await trigger.click({ force: true }).catch(() => undefined);
      await A.page.locator('[role="option"]').first().waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
      const labels = await A.page.getByRole("option").allInnerTexts();
      verdict.kindOptions = labels.length;
      verdict.kindLabelsSample = labels.slice(0, 12);
      verdict.kindLabelsDuplicated = labels.length - new Set(labels).size;
      await A.page.keyboard.press("Escape");
    }
    await A.page.screenshot({ path: join(captures, `s18-14c-p33-${tag}-space.png`) });
    await A.page.goto(url, { waitUntil: "domcontentloaded" });
    await A.page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 });
    await A.page.waitForTimeout(8_000);
    verdict.homeAfterReloadListsSpace = (await homeRows()).some(([, text]) => text.includes(name));
    verdict.homeRowsAfterReload = (await homeRows()).length;
  }
  verdict.timeline = timeline;
  verdict.pageerrors = A.lines.filter((line) => line.includes("pageerror")).length;
  verdict.faultLines = A.lines.filter((line) => /fault|refus|UiText|capacity|exhausted|recovery|reopen/iu.test(line) && !/status of 404|DevTools/u.test(line)).slice(-12).map((line) => line.slice(0, 300));
  console.log("VERDICT", JSON.stringify(verdict));
} catch (error) {
  console.log("ERROR", String(error?.stack ?? error).slice(0, 800), JSON.stringify(verdict));
} finally {
  await browser.close();
}
