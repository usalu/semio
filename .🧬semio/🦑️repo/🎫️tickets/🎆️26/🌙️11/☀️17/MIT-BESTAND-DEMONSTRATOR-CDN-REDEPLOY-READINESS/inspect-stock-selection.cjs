const {createRequire} = require("node:module");
const {resolve} = require("node:path");
const project = createRequire(resolve("♻️mit-bestand/🧺️demonstrator/package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= resolve(".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright");
const {chromium} = project("@playwright/test");

/** 🔍️ Records the actual pointer target and the resulting selection for each stock activation. */
async function inspect() {
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({locale: "en-US", viewport: {width: 1440, height: 900}});
    page.on("pageerror", error => console.log("[DEBUG] page-error", String(error)));
    page.on("console", message => {
      if (["error","warning"].includes(message.type()) || /command.failed|unsupported|interactionSelect|stock-|plugin-ui|patch[.-]|fault|refusal|reject|command stall|action.failed/i.test(message.text())) console.log("[DEBUG]", message.text().slice(0, 400));
    });
    await page.addInitScript(() => {
      const Native = Worker;
      const seen = new Set();
      const log = (key, detail) => {
        if (seen.has(key)) return;
        seen.add(key);
        console.info("[DEBUG] stock-wire", key, detail);
      };
      window.Worker = class extends Native {
        constructor(...args) {
          super(...args);
          this.addEventListener("message", ({data}) => {
            const result = data.value ?? data.result;
            const ingress = result?.commandIngress;
            if (window.stockObserve) {
              for (const field of ["effects", "uiPatches", "diagnostics", "operations"]) {
                for (const effect of result?.[field] ?? []) {
                  if (field === "uiPatches") {
                    const ops = effect.ops.map(op => [op.tag, new TextDecoder().decode(op.val?.node instanceof Uint8Array ? op.val.node : new Uint8Array()).match(/[A-Za-z0-9.-]{3,}/g)?.slice(0,22)]);
                    log("patch " + effect.surface.surface + " " + effect.revision, JSON.stringify(ops));
                    continue;
                  }
                  const bytes = effect.val?.payload ?? effect.val?.input ?? effect.payload ?? effect.bytes;
                  const detail = bytes instanceof Uint8Array ? new TextDecoder().decode(bytes).match(/[A-Za-z0-9.-]{4,}/g)?.filter(word => /select|rows|glulam|scope|unsupported|fault|denied|interaction|progress|operation|sourcing|terminal|failed|missing/i.test(word)).slice(0,28).join(" ") : JSON.stringify(effect, (_, v) => typeof v === "bigint" ? String(v) : v)?.slice(0,500);
                  log("effect " + field + " " + effect.tag + " " + (detail ?? "").slice(0,180), Object.keys(effect));
                }
              }
            }
            if (ingress) log("ingress " + ingress.tag + " " + ingress.val?.seq, JSON.stringify(ingress, (_,v) => typeof v === "bigint" ? String(v) : v).slice(0,180));
            if (data.ok === false) log("fault " + data.requestId, String(data.error ?? data.message));
          });
        }
        postMessage(data, ...args) {
          if (data.commandPage?.bytes) {
            const words = new TextDecoder().decode(Uint8Array.from(data.commandPage.bytes)).match(/[A-Za-z0-9.-]{4,}/g) ?? [];
            const detail = words.join(" ").slice(0,240);
            log("page " + data.commandPage.cursor?.seq + " " + detail, data.actorId);
          }
          return super.postMessage(data,...args);
        }
      };
    });
    await page.addInitScript(() => document.addEventListener("click", e => {
      if (e.target.closest("[data-row-id]")) console.info("[DEBUG] stock-click", e.target.tagName, e.target.textContent.slice(0, 70));
    }, true));
    await page.addInitScript(() => {
      const seen = new Set();
      new MutationObserver(() => {
        for (const element of document.querySelectorAll('[role="alert"],[aria-live="assertive"],[role="status"]')) {
          const text = element.textContent.trim();
          if (text && !seen.has(text)) {seen.add(text); console.info("[DEBUG] stock-status", text.slice(0,400));}
        }
      }).observe(document, {subtree:true, childList:true, characterData:true});
    });
    if (process.argv.includes("--trace-host")) {
      await page.route(url => decodeURIComponent(url.pathname).endsWith("/🌐️-BW-2g-aM.js"), async route => {
        const response = await route.fetch();
        let source = await response.text();
        const traces = [
          ['xs.record("patch.install",', 'console.info("[DEBUG] stock-install",B,String(O.view.revision)),xs.record("patch.install",'],
          ['catch(O){throw D&&await y(E,D)', 'catch(O){console.info("[DEBUG] stock-intake-error",String(O));throw D&&await y(E,D)'],
          ['return f(E,R),j.turns', 'return console.info("[DEBUG] stock-announce",R.join(",")),f(E,R),j.turns'],
        ];
        for (const [needle,replacement] of traces) {
          if (source.split(needle).length !== 2) throw new Error("Unexpected baseline host source " + needle);
          source = source.replace(needle,replacement);
        }
        await route.fulfill({response,body:source});
      });
    }
    await page.goto(process.argv[2], {waitUntil:"domcontentloaded"});
    const pane = page.locator('[data-shell-id="aussuchen"]');
    const row = pane.locator('[data-row-id="beam-glulam-gl24h"]').first();
    await row.waitFor({timeout:180000});
    const skip = pane.locator('[id="ui.introduction.skip"],[data-element-alias~="ui.introduction.skip"]').first();
    if (await skip.isVisible()) await page.keyboard.press("Escape");
    const preview = pane.locator('[id="framework.window.sourcingPreview"],[data-element-alias~="framework.window.sourcingPreview"]').first();
    await page.evaluate(() => {window.stockObserve = true;});
    for (const method of ["center","name","keyboard"]) {
      if (method === "center") await row.click();
      if (method === "name") await row.getByText("Glulam GL24h 200×400", {exact:true}).click();
      if (method === "keyboard") await row.press("Enter");
      const world = preview.locator(".semio-world-3d-host");
      await world.waitFor({timeout:12000}).catch(() => {});
      console.log("[DEBUG]", {method, selected:await row.getAttribute("data-active"), preview:await preview.innerText(), world:await world.count()});
      if (await world.count()) break;
    }
  } finally {await browser.close();}
}
inspect().catch(error => {console.error(error); process.exitCode=1;});
