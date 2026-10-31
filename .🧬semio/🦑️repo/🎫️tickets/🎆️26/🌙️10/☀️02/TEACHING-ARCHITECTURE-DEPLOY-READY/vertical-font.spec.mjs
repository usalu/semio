import { writeFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { test, expect, enter, primary, card, pane, goHome, layoutQuiet } from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts";

const run = process.env.VERTICAL_PROBE_RUN;
if (!/^[a-z0-9-]{1,12}$/.test(run ?? "")) throw Error("Explicit bounded vertical probe run required");
const root = dirname(fileURLToPath(import.meta.url));
const words = { en: ["Text size", "Largest", "Normal"], de: ["Textgröße", "Maximal", "Normal"] };

for (const initial of ["en", "de"]) test("native vertical font geometry " + initial, async ({ device }, info) => {
  const output = join(root, "🗑️generated", run + "-" + initial + ".json");
  if (existsSync(output)) throw Error("Fresh vertical receipt required");
  const learner = await device(initial), page = learner.page, stages = [];
  const runtime = []; let entered = false;
  page.on("console", message => { const text = message.text(); if (/^\[vite\] (server connection lost|connecting|connected)/.test(text) && runtime.length < 32) runtime.push({ at: Date.now(), entered, text }); });
  const tiny = initial === "en" ? { width: 1440, height: 100 } : { width: 375, height: 200 };
  const link = () => card(page, "learner").getByRole("link");
  let failed = false;
  const inspect = async (stage, locale) => {
    await layoutQuiet(page);
    const collect = () => page.evaluate(async () => {
      const saved = window.verticalNativeProbe, app = document.querySelector(".course-app"), main = document.querySelector("#course-main");
      const port = main.querySelector("[data-course-scrollport]"), focused = document.activeElement;
      const value = () => [port.style.minBlockSize, main.style.minBlockSize, port.clientWidth, port.clientHeight, main.clientHeight];
      let writes = 0, frameNumber = 0; const mutations = [];
      const observer = new MutationObserver(records => { const watched = records.filter(record => record.target === port || record.target === main || record.target === saved.focused); writes += watched.length; for (const record of watched) if (mutations.length < 64) mutations.push({ frame: frameNumber, at: performance.now(), tag: record.target.tagName, old: record.oldValue?.slice(0, 512), current: record.target.getAttribute("style")?.slice(0, 512) }); });
      observer.observe(main, { attributes: true, subtree: true, attributeFilter: ["style"], attributeOldValue: true });
      const samples = [];
      try {
        for (let index = 0; index < 12; index++) {
          await new Promise(resolve => requestAnimationFrame(resolve));
          frameNumber++; samples.push(value());
        }
      } finally { observer.disconnect(); }
      const controls = [...port.querySelectorAll("button,input:not([type=hidden]),select,textarea,a[href],[role=button],[tabindex]:not([tabindex='-1'])")]
        .filter(node => node.closest("[inert],[hidden],[aria-hidden=true]") === null && node.getClientRects().length > 0);
      const rect = node => { const box = node.getBoundingClientRect(); return { x: box.x, y: box.y, width: box.width, height: box.height }; };
      return {
        lang: app.lang, viewport: { width: innerWidth, height: innerHeight }, rootFont: getComputedStyle(document.documentElement).fontSize,
        font: getComputedStyle(focused).fontFamily, portMinimum: parseFloat(port.style.minBlockSize), mainMinimum: parseFloat(main.style.minBlockSize),
        port: rect(port), main: rect(main), app: { ...rect(app), client: app.clientHeight, scroll: app.scrollHeight, top: app.scrollTop },
        controls: controls.length, tallestControl: Math.max(0, ...controls.map(node => node.getBoundingClientRect().height)),
        active: { tag: focused.tagName, id: focused.id, text: focused.textContent?.slice(0, 120) }, focusRect: rect(saved.focused),
        focusAncestors: (() => { const list = []; for(let n = saved.focused; n && list.length < 12; n = n.parentElement) { const s = getComputedStyle(n); list.push({ tag: n.tagName, id: n.id, classes: String(n.className).slice(0, 120), rect: rect(n), x: s.overflowX, y: s.overflowY, top: n.scrollTop, scroll: n.scrollHeight, client: n.clientHeight }); } return list; })(),
        sameDOM: saved.app === app && saved.main === main && saved.port === port && saved.focused === focused && focused.isConnected,
        savedStyle: saved.focused.getAttribute("style"), writes, samples, mutations,
        documentOverflow: document.documentElement.scrollWidth - document.documentElement.clientWidth
      };
    });
    let observed; const windows = [];
    try {
      await expect.poll(async () => {
        observed = await collect();
        const stable = observed.samples.every(sample => JSON.stringify(sample) === JSON.stringify(observed.samples[0]));
        if (windows.length < 32) windows.push({ writes: observed.writes, stable, mutations: observed.mutations });
        return observed.writes === 0 && stable;
      }, { message: "native geometry converges for twelve unmodified animation frames" }).toBe(true);
    } finally { if (observed) stages.push({ stage, ...observed, convergenceWindows: windows }); }
    await expect(link()).toBeFocused();
    await expect(link()).toBeInViewport({ ratio: 1 });
    expect(observed.lang).toBe(locale);
    expect(observed.sameDOM).toBe(true);
    expect(observed.controls).toBeGreaterThan(0);
    expect(Number.isFinite(observed.portMinimum)).toBe(true);
    expect(observed.portMinimum).toBeGreaterThan(0);
    expect(observed.mainMinimum).toBeGreaterThanOrEqual(observed.portMinimum);
    expect(observed.port.height + 1).toBeGreaterThanOrEqual(observed.tallestControl);
    expect(observed.documentOverflow).toBeLessThanOrEqual(1);
    expect(observed.writes).toBe(0);
    expect(observed.samples.every(sample => JSON.stringify(sample) === JSON.stringify(observed.samples[0]))).toBe(true);
    return observed;
  };
  const remember = async () => {
    await link().evaluate(node => {
      const main = document.querySelector("#course-main"), app = document.querySelector(".course-app");
      window.verticalNativeProbe = { app, main, port: main.querySelector("[data-course-scrollport]"), focused: node, font: app.style.fontFamily };
    });
  };
  const textSize = async (locale, name) => {
    await page.setViewportSize({ width: 1440, height: 1000 });
    await primary(card(page, "prefs")).click();
    await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
    await pane(page, "prefs").getByRole("group", { name: words[locale][0], exact: true }).getByRole("button", { name, exact: true }).click();
    await goHome(learner);
    await link().focus();
    await remember();
  };
  try {
    await page.setViewportSize(tiny);
    await enter(learner, { kind: "anonymous" }, true);
    entered = true;
    await page.evaluate(() => document.fonts.ready);
    await link().focus();
    await remember();
    await inspect("tiny-entry", initial);
    await page.setViewportSize({ width: 375, height: 200 });
    const normal = await inspect("narrow-normal", initial);
    await page.locator(".course-app").evaluate(node => { node.style.fontFamily = "monospace"; });
    const font = await inspect("custom-font", initial);
    expect(font.font).toContain("monospace");
    await page.locator(".course-app").evaluate(node => { node.style.fontFamily = window.verticalNativeProbe.font; });
    const restored = await inspect("restored-font", initial);
    expect(Math.abs(restored.portMinimum - normal.portMinimum)).toBeLessThanOrEqual(1);
    await page.setViewportSize({ width: 1440, height: 1000 });
    const locale = initial === "en" ? "de" : "en";
    await page.locator('header button[lang="' + locale + '"]').click();
    await link().focus();
    await page.setViewportSize({ width: 375, height: 200 });
    const translated = await inspect("translated-normal", locale);
    await textSize(locale, words[locale][1]);
    await page.setViewportSize({ width: 375, height: 200 });
    const large = await inspect("largest-narrow", locale);
    expect(parseFloat(large.rootFont)).toBeGreaterThan(parseFloat(translated.rootFont));
    expect(large.portMinimum).toBeGreaterThan(translated.portMinimum);
    await page.setViewportSize({ width: 1440, height: 1000 });
    const wide = await inspect("largest-desktop", locale);
    expect(wide.app.scroll).toBeLessThanOrEqual(wide.app.client + 1);
    await textSize(locale, words[locale][2]);
    await page.setViewportSize({ width: 375, height: 200 });
    const final = await inspect("restored-text", locale);
    expect(final.rootFont).toBe(translated.rootFont);
    expect(Math.abs(final.portMinimum - translated.portMinimum)).toBeLessThanOrEqual(1);
    await page.setViewportSize(tiny);
    await inspect("restored-tiny", locale);
  } catch (error) { failed = true; throw error; }
  finally {
    try { writeFileSync(output, JSON.stringify({ initial, stages, runtime }, null, 2), { flag: "wx" }); }
    catch (error) { if (!failed) throw error; info.annotations.push({ type: "debug-observer-failure", description: String(error).slice(0, 1024) }); }
    if (runtime.some(event => event.entered && event.text.includes("server connection lost"))) {
      if (!failed) throw Error("Development runtime restarted during native layout proof");
      info.annotations.push({ type: "runtime-restarted-during-proof", description: "Native sample overlapped a Vite disconnect; preserve the original failure and inspect runtime events." });
    }
  }
});
