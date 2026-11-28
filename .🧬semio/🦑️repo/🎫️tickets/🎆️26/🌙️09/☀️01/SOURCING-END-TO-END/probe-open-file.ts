import { assert, awaitBeacon, dismissIntroduction, join, out, runJourneys, session, url, type Browser } from "./journey-kit.ts";
await runJourneys("probe", async (browser: Browser, language) => {
  const s = await session(browser, language);
  const { page, settle } = s;
  const path = process.env.FILE!;
  page.on("console", m => { if (/DEBUG|os-shell|natural|Retained|rror/i.test(m.text())) console.log("CONSOLE", m.type(), m.text().slice(0,900)); });
  await page.goto(url);
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  await settle(3000);
  if (!process.env.SKIPFIX) { await page.locator('[id="playground.navbar.fixture"]').click();
  await page.getByRole("option", { name: /^(no example|kein)/iu }).first().click(); }
  await settle(2500);
  try { await page.locator("[id='framework.window.sourcingPool.engagement.toggle']").click({ timeout: 8000 }); } catch { await s.shot("probe-notoggle"); }
  (globalThis as any).go = true; const chooser = page.waitForEvent("filechooser", { timeout: 20_000 });
  await page.locator("[id='action.openArtifactFile']").click();
  await (await chooser).setFiles(path);
  await settle(8000);
  console.log("BODY", (await s.body()).slice(-1500));
  console.log("FAULTS", s.faults);
  await s.shot("probe");
}, );
