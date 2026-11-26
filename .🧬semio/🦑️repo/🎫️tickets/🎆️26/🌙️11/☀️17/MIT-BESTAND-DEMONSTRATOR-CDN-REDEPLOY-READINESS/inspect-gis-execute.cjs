const {createRequire} = require("node:module");
const {resolve} = require("node:path");
const project = createRequire(resolve("♻️mit-bestand/🧺️demonstrator/package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= resolve(".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright");
const {chromium} = project("@playwright/test");

/** 🩺️ Reads the native fault before the shell projects its localizable code. */
async function capturePublicationFault(page) {
  const session = await page.context().newCDPSession(page);
  session.on("Debugger.paused", async ({ data }) => {
    try {
      if (!data?.objectId) return;
      const { result } = await session.send("Runtime.callFunctionOn", {
        objectId: data.objectId,
        functionDeclaration: `function () {
          const fault = this.fault;
          if (fault?.code !== "interactive-job.publication-stalled") return null;
          return { code: fault.code, message: fault.message, causes: fault.causes };
        }`,
        returnByValue: true,
      });
      if (result.value) console.log("[DEBUG] original GIS publication fault", JSON.stringify(result.value));
    } catch (error) {
      console.error("[DEBUG] GIS publication fault capture", String(error));
    } finally {
      await session.send("Debugger.resume").catch(() => {});
    }
  });
  await session.send("Debugger.enable");
  await session.send("Debugger.setPauseOnExceptions", { state: "all" });
  return async () => { await session.send("Debugger.disable"); await session.detach(); };
}

/** 🗺️ Inspects the live count rows and their authored panel scopes. */
async function inspect() {
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({locale:"en-US"});
    page.on("console", m=>console.log("[DEBUG]",m.type(),m.text().slice(0,1200)));
    page.on("pageerror", e=>console.log("[DEBUG]",String(e)));
    await page.goto(process.argv[2],{waitUntil:"domcontentloaded"});
    const pane = page.locator('[data-shell-id="verfolgen"]');
    await page.locator('[data-shell-id="verfolgen"][data-shell-ready]').waitFor({timeout:140000});
    const button = pane.locator('[data-tab-id="framework.panel.inspection"]:visible').first();
    await button.waitFor({timeout:150000});
    const skip = pane.locator('[id="ui.introduction.skip"],[data-element-alias~="ui.introduction.skip"]').first();
    if(await skip.isVisible()) await skip.click({timeout:5000}).catch(()=>page.keyboard.press("Escape"));
    if(await button.getAttribute("aria-pressed") !== "true") await button.click();
    await pane.locator('[id="framework.window.gis2dMain.engagement.toggle"],[data-element-alias~="framework.window.gis2dMain.engagement.toggle"]').first().click();
    const add = pane.getByRole("button",{name:"Add Feature…"});
    await add.waitFor();
    console.log("[DEBUG]", await add.evaluate(x=>[x.closest("[id]")?.id,x.closest("[data-slot=window-action-pane]")?.outerHTML.slice(0,300)]));
    await add.click();
    await pane.locator('[id="framework.window.gis2dMain.action.addFeature.arg.label.input"]').fill("Release test");
    const stop = process.argv.includes("--fault-detail") ? await capturePublicationFault(page) : null;
    try {
      await pane.locator('[id="framework.window.gis2dMain.action.addFeature.execute"]').click();
      await page.waitForTimeout(stop ? 15000 : 4000);
    } finally {
      await stop?.();
    }
    console.log("[DEBUG] result",await pane.getByRole("treeitem").allTextContents());
  } finally {await browser.close();}
}
inspect().catch(error=>{console.error(error);process.exitCode=1;});
