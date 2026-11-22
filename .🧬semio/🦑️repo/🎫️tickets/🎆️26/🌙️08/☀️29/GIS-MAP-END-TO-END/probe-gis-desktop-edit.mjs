import { chromium, devices } from "playwright";

const fullscreen = devices["Desktop Chrome"];
const browser = await chromium.launch({ channel: "chrome", headless: true });
try {
  const page = await browser.newPage({ ...fullscreen, locale: "en-US" });
  const notices = [];
  page.on("pageerror", error => notices.push(error.message));
  page.on("console", message => { if (["warning", "error"].includes(message.type())) notices.push(message.text()); });
  await page.goto(process.argv[2] ?? "http://127.0.0.1:6041/");
  await page.locator('[data-shell-ready="gis2d"]').first().waitFor({ timeout: 60000 });
  const report = async phase => console.log(JSON.stringify(await page.evaluate(({ phase, notices }) => {
    const visible = node => node.getClientRects().length > 0;
    const entry = node => ({
      tag: node.tagName, id: node.id, alias: node.getAttribute("data-element-alias"),
      slot: node.getAttribute("data-slot"), role: node.getAttribute("role"),
      pressed: node.getAttribute("aria-pressed"), expanded: node.getAttribute("aria-expanded"),
      label: node.getAttribute("aria-label"), text: node.innerText?.slice(0, 120),
      slots: [...node.querySelectorAll("[data-slot], input")].slice(0, 12).map(child => child.getAttribute("data-slot") ?? child.tagName),
    });
    const collect = selector => [...document.querySelectorAll(selector)].filter(visible).slice(0, 24).map(entry);
    return {
      phase, notices, relevant: collect('[data-tab-id="framework.panel.inspection"], [id*="positions-count"], [id*="engagement.toggle"], [id*="addFeature"]'),
      panes: collect('[data-slot*="action-pane"]'),
      text: document.body.innerText.slice(0, 1200),
    };
  }, { phase, notices: notices.slice(-8) })));
  await report("initial");
  const tab = page.locator('[data-tab-id="framework.panel.inspection"]:visible').first();
  console.log(JSON.stringify({ tab: await tab.getAttribute("aria-pressed"), html: await tab.evaluate(node => node.outerHTML.slice(0, 650)) }));
  if (await tab.getAttribute("aria-pressed") !== "true") await tab.click();
  await new Promise(resolve => setTimeout(resolve, 2500));
  await report("inspection");
  const engagement = page.locator('[id*="engagement.toggle"]:visible, [data-element-alias*="engagement.toggle"]:visible').first();
  if (await engagement.count()) {
    await engagement.click({ timeout: 4000 });
    const addFeature = page.locator('[id="action.addFeature"] [data-slot="tree-label"]');
    await addFeature.waitFor({ timeout: 5000 });
    await report("actions");
    await addFeature.click();
    await new Promise(resolve => setTimeout(resolve, 1200));
    await report("add feature fields");
    if (process.argv.includes("--execute")) {
      await page.locator('[id="action.addFeature.arg.label"] input').fill("GIS isolated edit probe");
      await page.locator('[id="framework.window.gis2dMain.action.addFeature.execute"]').click();
      await new Promise(resolve => setTimeout(resolve, 8000));
      await report("add feature result");
    }
  }
} finally {
  await browser.close();
}
