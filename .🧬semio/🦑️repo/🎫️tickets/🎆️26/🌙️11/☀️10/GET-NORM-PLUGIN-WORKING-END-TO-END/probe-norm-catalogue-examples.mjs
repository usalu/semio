import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {createRequire} from "node:module";
import {basename, isAbsolute} from "node:path";

const require = createRequire(process.cwd() + "/package.json");
const {chromium} = require("playwright");
const base = process.env.PLAYWRIGHT_BASE_URL;
const preview = process.env.PLAY_TEST_GUEST;
const bridgeFile = process.env.PLAY_TEST_BRIDGE;
const bridgeOverride = bridgeFile && await readFile(bridgeFile, "utf8");
assert(!preview || isAbsolute(preview), "PLAY_TEST_GUEST must be an absolute path");
assert(base, "PLAYWRIGHT_BASE_URL is required");

const examples = {
  din16798: ["Compliant office", "Non-compliant office"],
  iso16757: ["Conforming catalogue", "Broken catalogue"],
  vdi3805: ["Demo", "Nonconforming Valve Dataset"],
};
const routes = process.argv.slice(2);
const browser = await chromium.launch({
  args: ["--in-process-gpu", "--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"],
});

try {
  for (const id of routes.length ? routes : Object.keys(examples)) {
    assert(examples[id], `Unsupported catalogue route: ${id}`);
    const [demo, nonconforming] = examples[id];
    const page = await browser.newPage({locale: "en-US"});
    const faults = new Set();
    let step = "boot";
    const shell = page.locator(`[data-shell-id="${id}"]`);
    let previewRequested = false;
    const coreResponses = [];
    page.on("response", response => {
      if (new URL(response.url()).pathname.endsWith("/semio_hub_norm_component.core.wasm")) {
        coreResponses.push({url: response.url(), status: response.status()});
      }
    });
    if (preview) {
      await page.route(url => url.pathname.endsWith("/" + basename(preview)), async route => {
        previewRequested = true;
        const url = new URL(route.request().url());
        url.pathname = "/@fs" + preview;
        await route.continue({url: url.href});
      });
    }
    let bridgeRequested = false;
    if (bridgeOverride || process.env.PLAY_TRACE_NORM_REFUSAL) {
      await page.route(url => decodeURIComponent(url.pathname).endsWith("/📕️norm/🌉️bridge.js"), async route => {
        const response = await route.fetch();
        try {
          bridgeRequested = true;
          const body = bridgeOverride || await response.text();
          if (!process.env.PLAY_TRACE_NORM_REFUSAL) {
            await route.fulfill({response, body});
            return;
          }
          const anchor = "const receipt = failure?.payload ?? failure;";
          assert(body.includes(anchor), "The bridge has no poll receipt handler");
          const observed = `console.warn("[DEBUG] Norm poll refusal", JSON.stringify({
            message: failure?.message, failure: receipt?.failure, epoch: receipt?.input?.epoch,
          }, (_, value) => typeof value === "bigint" ? value.toString() : value));`;
          await route.fulfill({response, body: body.replace(anchor, anchor + observed)});
        } finally {
          await response.dispose();
        }
      });
    }
    page.on("pageerror", error => faults.add(error.message));
    page.on("crash", () => faults.add("renderer crashed"));
    const failureCauses = [];
    const diagnostics = [];
    page.on("console", message => {
      const output = message.text();
      if (diagnostics.length < 30 && ["error", "warning"].includes(message.type())) {
        diagnostics.push({level: message.type(), text: output.slice(0, 1000)});
      }
      if (/input #\d+ .*refused|boot failed|dispatch-failed|authority failure|load-failed/.test(output)) {
        faults.add(output.split("\n", 1)[0]);
      }
      if (output.includes("documentTransfer.load-failed")) {
        const error = message.args()[1];
        if (error) failureCauses.push(error.evaluate(value => {
          const chain = [];
          for (let depth = 0; value != null && depth < 5; depth++, value = value.cause) {
            chain.push(typeof value === "object" ? {
              name: value.name, message: value.message, code: value.code, tag: value.tag,
              stack: value.stack?.split("\n").slice(1, 5),
            } : String(value));
          }
          return chain;
        }).catch(() => []));
      }
    });

    try {
      const picker = shell.locator('[id="playground.navbar.fixture"]');
      const reportOverview = value => value.split(/\nCHECKS\b/i, 1)[0].trim();
      const summaryText = () => shell.locator('[data-slot="window-body"]').filter({hasText: /^\s*summary/i}).innerText({timeout: 20_000}).then(reportOverview);
      const pick = async (label, value) => {
        await picker.click();
        const options = page.getByRole("option", {name: label, exact: true});
        if (id === "din16798") assert.equal(await options.count(), 1, "Each example label must identify one option");
        const initial = page.getByRole("option", {selected: true});
        const priorValue = await initial.getAttribute("data-value", {timeout: 5_000});
        assert(priorValue, "The selected example has no stable value");
        await (value ? options.and(page.locator(`[data-value="${value}"]`)) : options).click();
        return priorValue;
      };
      const waitForExample = async (label, reference, same) => page.waitForFunction(({app, label, prior, same}) => {
        const pane = document.querySelector('[data-shell-id="' + app + '"]');
        if (pane?.querySelector('[id="playground.navbar.fixture"]')?.innerText.trim() !== label) return false;
        const frames = pane.querySelectorAll('[data-slot="window-body"]');
        const text = Array.from(frames, frame => frame.innerText).find(value => /^\s*summary/i.test(value));
        const overview = text?.split(/\nCHECKS\b/i, 1)[0].trim();
        return overview && ((overview === prior) === same) ? overview : false;
      }, {app: id, label, prior: reference, same}, {timeout: 45_000}).then(result => result.jsonValue());

      await page.goto(new URL("#" + id, base).href, {waitUntil: "commit"});
      await shell.locator('xpath=self::*[@data-shell-ready or @data-shell-error or @data-shell-not-found]').waitFor({timeout: 120_000});
      assert.equal(await shell.getAttribute("data-shell-ready"), id, (await shell.getAttribute("data-shell-error")) ?? "Shell was not ready");
      const introduction = shell.locator('[id="ui.introduction.skip"]');
      if (await introduction.count()) await introduction.click();
      await picker.waitFor({timeout: 20_000});
      assert.equal((await picker.innerText()).trim(), demo);
      const initialSummary = await summaryText();

      step = nonconforming;
      const originalValue = await pick(nonconforming);
      const changedSummary = await waitForExample(nonconforming, initialSummary, false);
      assert.equal((await picker.innerText()).trim(), nonconforming);

      step = demo;
      await pick(demo, originalValue);
      await waitForExample(demo, initialSummary, true);
      assert.equal((await picker.innerText()).trim(), demo);
      assert.deepEqual([...faults], []);
      if (preview) assert(previewRequested, "The browser did not request the preview core");
      if (bridgeOverride) assert(bridgeRequested, "The browser did not request the alternate bridge");
      console.log(`[DEBUG] PASS ${id}`, JSON.stringify({
        demo: initialSummary.split("\n")[1], nonconforming: changedSummary.split("\n")[1], coreResponses,
      }));
    } catch (error) {
      process.exitCode = 1;
      const outcome = await shell.evaluate(el => ({...el.dataset}), undefined, {timeout: 250}).catch(() => ({}));
      console.error(`[DEBUG] FAIL ${id} during ${step}`, JSON.stringify({
        message: error.message, outcome, previewRequested, bridgeRequested, coreResponses, faults: [...faults],
        picker: await shell.locator('[id="playground.navbar.fixture"]').innerText({timeout: 500}).catch(() => ""),
        report: await shell.locator('[data-slot="window-body"]').filter({hasText: /^\s*summary/i}).innerText({timeout: 500}).then(text => text.split(/\nCHECKS\b/i, 1)[0]).catch(() => ""),
        causes: (await Promise.all(failureCauses)).flat(), diagnostics,
      }));
    } finally {
      await page.close();
    }
  }
} finally {
  await browser.close();
}
