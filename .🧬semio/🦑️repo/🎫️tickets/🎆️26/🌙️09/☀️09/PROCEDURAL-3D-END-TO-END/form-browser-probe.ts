import assert from "node:assert/strict";
import { createServer } from "vite";
import tailwind from "@tailwindcss/vite";
import { chromium } from "playwright";
import { dirname, resolve } from "node:path";
import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";

const checkout = process.cwd();
const retained = process.argv.includes("--retained");
const generated = resolve(import.meta.dirname, "🗑️generated", retained ? "form-browser-retained" : process.argv.includes("--short-check") ? "form-browser-short-text" : "form-browser");
const textsFile = resolve(import.meta.dirname,"long-generate-text.json");
const texts = JSON.parse(await readFile(textsFile,"utf8"));
const renderer = resolve(checkout, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const fixtureFile = resolve(checkout, "✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🧫️fixtures/form-panel.json");
const fixture = JSON.parse(await readFile(fixtureFile, "utf8"));
const imageFixtureFile = resolve(dirname(fixtureFile), "form-image.json");
const imageCases = JSON.parse(await readFile(imageFixtureFile, "utf8"));
await mkdir(generated, { recursive: true });
await Promise.all([
  copyFile(resolve(import.meta.dirname, "form-browser-mount.tsx"), resolve(generated, "mount.tsx")),
  writeFile(resolve(generated, "index.html"), '<!doctype html><html><head><meta charset="utf-8"></head><body><div id="root"></div><script type="module" src="/mount.tsx"></script></body></html>'),
]);
const server = await createServer({
  configFile: false,
  root: generated,
  cacheDir: resolve(generated, "vite-cache"),
  esbuild: { jsx: "automatic" },
  plugins: [tailwind()],
  server: { host: "127.0.0.1", fs: { allow: [checkout, dirname(fixtureFile)] } },
  optimizeDeps: { entries: [resolve(generated, "index.html")], include: ["react", "react-dom", "react-dom/client", "react/jsx-runtime", "react/jsx-dev-runtime", "scheduler"] },
  resolve: { alias: {
    "probe:interpreter": resolve(renderer, "🧱️elements/🗣️Interpreter/🟦️.tsx"),
    "probe:document-store": resolve(renderer, "🧱️elements/📃️UiDocumentStore/🟦️.tsx"),
    "probe:action-bridge": resolve(renderer, "🧱️elements/🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx"),
    "probe:window-chrome": resolve(checkout, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧱️window-chrome/🟦️.tsx"),
    "probe:styles": resolve(renderer, "🎨️.css"),
    "probe:fixture": fixtureFile,
    "probe:long-text-fixture": textsFile,
    "probe:image-fixture": imageFixtureFile,
  } },
});
let browser;
try {
  await server.listen();
  browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 500, height: 400 } });
  const errors = [];
  page.on("pageerror", (error) => { errors.push(error.message); console.log("[DEBUG] browser error", error.message); });
  page.on("response", (response) => { if (response.status() >= 400) console.log("[DEBUG] response", response.status(), response.url().slice(-300)); });
  if (retained) {
    const origin = server.resolvedUrls.local[0] + "?retained";
    const compose = ({prefix,unit,repeat,suffix}) => prefix + unit.repeat(repeat) + suffix;
    await page.goto(origin,{waitUntil:"domcontentloaded"});
    const caption = fixture.fields.find(field=>field.id === fixture.finalEdit.questionId);
    const box = page.getByRole("textbox",{name:caption.label,exact:true});
    assert.equal(await box.inputValue(),compose(texts.cases[2]));
    const next = compose(texts.edit);
    await box.fill(next);
    await box.dispatchEvent("keydown",{key:"Enter",isComposing:true,bubbles:true});
    assert.equal(await page.evaluate(()=>window.formIntents.length),0);
    await box.press("Enter");
    await page.waitForFunction(()=>window.formIntents.length===1);
    const posted = await page.evaluate(()=>window.formIntents[0]);
    assert.equal(posted.windowId,fixture.surface);
    assert.equal(posted.action.args.questionId,caption.id);
    assert.equal(posted.action.args.generationId,fixture.generation);
    assert.equal(posted.action.args.value.flat().join(""),next);
    assert.ok(posted.action.args.value.flat().every(chunk=>Buffer.byteLength(chunk)<=512));
    await page.reload({waitUntil:"domcontentloaded"});
    assert.equal(await box.inputValue(),next);
    await box.fill("discard this");
    await box.press("Escape");
    assert.equal(await box.inputValue(),next);
    const alternative = "Ö" + next.slice(1);
    assert.equal(Buffer.byteLength(alternative),Buffer.byteLength(next));
    await box.fill("pending draft");
    await page.evaluate(({id,text})=>window.formExternal(id,text),{id:caption.id,text:alternative});
    await page.waitForFunction(()=>document.querySelector('[aria-invalid="true"]') !== null);
    assert.equal(await box.inputValue(),"pending draft");
    await box.press("Escape");
    assert.equal(await box.inputValue(),alternative);
    const notes = page.getByRole("textbox",{name:"Long notes",exact:true});
    const saved = compose(texts.cases[3]);
    assert.equal(await notes.inputValue(),saved);
    await notes.press("End");
    await notes.press("Enter");
    assert.equal(await page.evaluate(()=>window.formIntents.length),0);
    await notes.press("Control+Enter");
    await page.waitForFunction(()=>window.formIntents.length===1);
    const notesIntent = await page.evaluate(()=>window.formIntents[0]);
    assert.equal(notesIntent.action.args.questionId,"notes");
    assert.equal(notesIntent.action.args.value.flat().join(""),await notes.inputValue());
    await box.fill("Short again");
    await box.press("Enter");
    await page.waitForFunction(()=>window.formIntents.length===2);
    assert.equal(await page.evaluate(()=>window.formIntents[1].intent.input),"Short again");
    await page.reload({waitUntil:"domcontentloaded"});
    assert.equal(await box.inputValue(),"Short again");
    const crossing = compose(texts.cases[1]);
    await box.fill(crossing);
    await box.press("Enter");
    await page.waitForFunction(()=>window.formIntents.length===1);
    assert.equal(await page.evaluate(()=>window.formIntents[0].action.args.value.flat().join("")),crossing);
    console.log("[DEBUG] One-line and multiline retained answers mounted, committed and reopened");
  } else for (const locale of fixture.locales) {
    await page.goto(server.resolvedUrls.local[0] + "?locale=" + locale.id, { waitUntil: "domcontentloaded" });
    const form = page.getByRole("form", { name: locale.label });
    const imageCase = imageCases[locale.id];
    const picture = form.getByRole("img", { name: imageCase.alt });
    await picture.waitFor({ state: "attached" });
    await page.waitForFunction(label => {
      const image = [...document.images].find(element => element.alt === label);
      return image?.complete && image.naturalWidth === 1 && image.naturalHeight === 1;
    }, imageCase.alt);
    assert.equal(await picture.getAttribute("src"), imageCase.steps[0].blocks[0].src);
    for (const [kind, role] of [["slider", "slider"], ["text", "textbox"], ["number", "spinbutton"], ["boolean", "button"]]) {
      const field = fixture.fields.find((item) => item.kind === kind);
      await form.getByRole(role, { name: field.label, exact: true }).waitFor({ timeout: 30000 });
    }
    const before = await form.evaluate((element) => ({ height: element.clientHeight, content: element.scrollHeight, y: element.scrollTop, overflow: getComputedStyle(element).overflowY }));
    assert.equal(before.overflow, "auto");
    assert.ok(before.height <= fixture.viewport.height && before.content > before.height, JSON.stringify(before));
    await page.mouse.move(40, 80);
    await page.mouse.wheel(0, 160);
    await page.waitForFunction(() => document.querySelector('[role="form"]').scrollTop > 0);
    const number = fixture.fields.find((item) => item.kind === "number");
    const numberControl = form.getByRole("spinbutton", { name: number.label, exact: true });
    const numberLabel = form.locator('[data-slot="field-label"]').filter({ hasText: number.label });
    await numberLabel.click();
    assert.ok(await numberControl.evaluate((element) => document.activeElement === element));
    await form.evaluate((element) => { element.scrollTop = 0; });
    const caption = fixture.fields.find((item) => item.id === fixture.finalEdit.questionId);
    const input = form.getByRole("textbox", { name: caption.label, exact: true });
    const visibleLabel = form.locator('[data-slot="field-label"]').filter({ hasText: caption.label });
    await visibleLabel.click();
    assert.ok(await input.evaluate((element) => document.activeElement === element));
    await input.fill(fixture.finalEdit.value);
    await input.press("Tab");
    await page.waitForFunction(() => window.formIntents.length === 1);
    const [{ intent, windowId, action }] = await page.evaluate(() => window.formIntents);
    assert.equal(windowId, fixture.surface);
    assert.deepEqual(intent.args, { generationId: fixture.generation, questionId: fixture.finalEdit.questionId });
    assert.equal(intent.input, fixture.finalEdit.value);
    assert.deepEqual(action.args, { ...intent.args, value: fixture.finalEdit.value });
    assert.ok(await form.evaluate((element) => element.scrollTop > 0));
    const size = fixture.fields.find((item) => item.kind === "slider");
    const slider = form.getByRole("slider", { name: size.label, exact: true });
    await slider.press("ArrowRight");
    await page.waitForFunction(() => window.formIntents.some(({ intent }) => intent.input?.commit === true));
    const enabled = fixture.fields.find((item) => item.kind === "boolean");
    await form.getByRole("button", { name: enabled.label, exact: true }).click();
    await page.waitForFunction(() => window.formIntents.length === 4);
    const [offered, committed, toggled] = await page.evaluate(() => window.formIntents.slice(1));
    const sliderArgs = { generationId: fixture.generation, questionId: size.id, value: size.value + size.step };
    assert.equal(typeof offered.action.args.gesture, "string");
    assert.deepEqual(offered.action.args, { ...sliderArgs, gesture: offered.action.args.gesture, commit: false });
    assert.deepEqual(committed.action.args, { ...sliderArgs, gesture: offered.action.args.gesture, commit: true });
    assert.deepEqual(toggled.action.args, { generationId: fixture.generation, questionId: enabled.id, value: !enabled.value });
    assert.ok([offered, committed, toggled].every((edit) => edit.windowId === fixture.surface && edit.action.action === "updateGenerationValues"));
    console.log(locale.id + ": named form, decoded image, four named controls, wheel scroll, label click, keyboard focus, three owned edits");
  }
  assert.deepEqual(errors, []);
} finally {
  await browser?.close();
  await server.close();
}
