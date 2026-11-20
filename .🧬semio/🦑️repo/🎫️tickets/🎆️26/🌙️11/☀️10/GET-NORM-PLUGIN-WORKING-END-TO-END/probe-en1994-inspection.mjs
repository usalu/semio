import { strict as assert } from "node:assert";
import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = fileURLToPath(new URL(".", import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const require = createRequire(join(root, "🏢️semio-tech", "🎡️play", "package.json"));
const { chromium } = require("playwright");
const log = await readFile(process.argv[2], "utf8");
const marker = "[DEBUG] explanation ";
const expected = JSON.parse(log.split("\n").find(line => line.startsWith(marker)).slice(marker.length)).text;
const timeout = 140_000;
const browser = await chromium.launch({args:["--in-process-gpu","--enable-unsafe-swiftshader","--disable-renderer-backgrounding"]});
const page = await browser.newPage({locale:"en-US", viewport:{width:1440,height:1000}});
const observed = { meshes: 0, instances: 0, explanation: false, viewer: false, faults: [], assets: [] };
try {
  page.on("console", message => {
    const text = message.text();
    if (/semantic failure|ui\.surface-render|input #\d+ .*refused|load-failed|role switch to viewer failed/.test(text)) observed.faults.push(text.split("\n")[0]);
  });
  page.on("pageerror", error => observed.faults.push(error.message));
  page.on("response", response => {
    const url = decodeURIComponent(new URL(response.url()).pathname);
    if (url.includes("norm-eurocode-structural") && /(?:bridge\.js|\.core\.wasm|component\.js)$/.test(url)) observed.assets.push([url, response.status()]);
  });
  await page.goto(new URL("#en1994", process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6033/").href, {waitUntil:"commit"});
  const shell = page.locator('[data-shell-id="en1994"]');
  await shell.locator('xpath=self::*[@data-shell-ready or @data-shell-error or @data-shell-not-found]').waitFor({state:"attached",timeout});
  assert(await shell.getAttribute("data-shell-ready") !== null, await shell.innerText());
  const skip = shell.locator('[id="ui.introduction.skip"]');
  if (await skip.count()) await skip.click({timeout});
  const model = shell.locator('[data-ui-node-key="norm.en1994.play.viewport"] .semio-world-3d-host');
  const modelTab = shell.getByRole("tab",{name:"Model",exact:true});
  if (!(await model.isVisible()) && await modelTab.count()) await modelTab.first().click({timeout});
  await model.locator("canvas[data-world-geometry-drawn]").waitFor({state:"visible",timeout});
  Object.assign(observed, await model.evaluate(host => {
    const canvas = host.querySelector("canvas[data-world-geometry-drawn]");
    return {meshes: Object.keys(JSON.parse(host.dataset.meshesJson)).length, instances: Object.keys(JSON.parse(host.dataset.instancesJson)).length, revision: host.dataset.worldGeometryRevision, drawn: canvas.dataset.worldGeometryRevision};
  }));
  assert(observed.meshes && observed.instances && observed.revision === observed.drawn);
  const inspection = shell.locator('[data-ui-node-key="norm-inspection"]');
  const tab = shell.getByRole("tab",{name:"Inspection",exact:true});
  if (!(await inspection.isVisible()) && await tab.count()) await tab.first().click({timeout});
  const chunk = inspection.locator('[data-slot="tree-item-content"] [data-ui-node-key^="norm-text-chunk-"]');
  const row = shell.getByText(/Fire bending resistance at the steel temperature/).first();
  if (!(await chunk.count())) {
    const checks = shell.getByRole("tab",{name:"Checks",exact:true});
    if (await checks.count()) await checks.first().click({timeout});
    for (let i = 0; i < 15 && !(await row.count()); i++) {
      const list = shell.locator('[data-tree-window-key="norm-checks.list"]').first();
      if (!(await list.count())) break;
      await list.evaluate(element => {
        const scroller = element.closest('[data-slot="scroll-area"]') ?? element.parentElement;
        scroller.scrollBy(0,Math.max(80,scroller.clientHeight * 0.7));
      });
      await page.waitForTimeout(170);
    }
    await row.click({timeout:15000});
    if (!(await inspection.isVisible()) && await tab.count()) await tab.first().click({timeout});
  }
  await chunk.first().waitFor({state:"visible",timeout:20000});
  const text = await chunk.evaluateAll(parts => parts.map(part => part.textContent).join(""));
  assert.match(text,/M_fi,Ed/);
  assert.match(text,/unverified basis/i);
  observed.explanation = {bytes:Buffer.byteLength(text),chunks:await chunk.count(),matchesFixture:text===expected};
  const disclosure = inspection.getByText("Explanation",{exact:true}).locator("xpath=ancestor::*[@aria-expanded][1]");
  await disclosure.click({timeout:20000});
  await chunk.first().waitFor({state:"hidden",timeout:20000});
  await disclosure.press("Enter");
  await chunk.first().waitFor({state:"visible",timeout:20000});
  assert.equal(await chunk.evaluateAll(parts => parts.map(part => part.textContent).join("")),text);
  await shell.locator('[id="playground.navbar.roles.viewer"]').click({timeout:20000});
  await shell.locator('[data-slot="surface-role-chip"][data-role="viewer"]').waitFor({state:"visible",timeout:60000});
  await shell.locator('[data-ui-node-key="norm.en1994.view.viewport"] canvas[data-world-geometry-drawn]').waitFor({state:"visible",timeout:60000});
  observed.viewer = true;
  assert.deepEqual(observed.faults,[]);
} catch (error) {
  observed.error = String(error);
  observed.shell = (await page.locator('[data-shell-id="en1994"]').innerText({timeout:3000}).catch(()=>"")).slice(0,2800);
  await page.screenshot({path:join(ticket,"🗑️generated","en1994-inspection-canonical.png"),timeout:5000}).catch(()=>{});
  process.exitCode = 1;
} finally {
  console.log(JSON.stringify(observed));
  await browser.close();
}
