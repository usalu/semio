/** 🔬️ Ticket tool of work package P1: how many elements one kind of write into a pet's drawing makes the browser restyle, and what that write costs in style, paint, pre-paint, layerization and on the GPU — on the home overview with still pets (which write nothing by themselves), one write per animation frame for two seconds, traced; optionally under a stylesheet that overrides the pets' own (`--css`), to compare ways of compositing them.
 *
 * Kinds: `none` (frames only), `group` (the `transform` attribute of one bone's group of one pet), `groups` (every
 * group of one pet), `all` (every group of every pet), `allcss` (the same through each group's CSSOM `transform`),
 * `alldom` (the same through the SVG DOM's transform list), `pupil` (`cx` of one pupil), `place` (the CSSOM `transform` of
 * one pet's root), `places` (of every pet's root), `opacity` (the CSSOM `opacity` of one pet's root), `cursor` (the hand's
 * `data-pet-cursor` on the document element, set and removed by turns), `mark` (the same with `data-p1`, which only the
 * stylesheets of `--css` name, to compare ways of showing the hand), `eye` and `part` (the group of a bone that carries
 * an eye, or a single shape).
 *
 * Usage (from the repository root): node ".../p1_style_probe.mjs" [--url http://127.0.0.1:6245/] [--browser gpu|shell]
 *        [--kinds none,group,...] [--css "<rules>"]...
 */
import { chromium } from "playwright";

const option = (name, fallback) => {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : process.argv[at + 1];
};
const sheets = process.argv.flatMap((value, index) => (process.argv[index - 1] === "--css" ? [value] : []));
const url = option("url", "http://127.0.0.1:6245/");
const kinds = option("kinds", "none,group,groups,all,pupil,place,places,opacity").split(",");
const browser = await chromium.launch(option("browser", "gpu") === "gpu" ? { channel: "chromium" } : {});
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1, locale: "en-US", bypassCSP: true });
const page = await context.newPage();
await page.goto(url);
const intro = page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]');
await intro.waitFor({ timeout: 60_000 });
await intro.click();
await page.locator('#quiz-main [data-card="identity"] input[type="radio"][value="anonymous"]').check();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
await page.locator('[data-layered-overview][data-mode="strip"]').waitFor({ timeout: 60_000 });
await page.evaluate(() => {
  const key = Object.keys(localStorage)
    .find((name) => name.startsWith("semio.quiz.") && name.endsWith(".learner"))
    .replace(/learner$/u, "preferences");
  localStorage.setItem(key, JSON.stringify({ ...JSON.parse(localStorage.getItem(key) ?? "{}"), pets: "still", petsChosen: true }));
});
await page.mouse.move(720, 20);
await page.reload();
await page.locator(".pet-layer svg.pet").first().waitFor({ timeout: 90_000 });
await page.waitForTimeout(5000);
const pets = await page.evaluate(() => [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => `${pet.getAttribute("data-pet")} ${pet.querySelectorAll("*").length + 1}`));
process.stdout.write(`pets on stage (elements): ${pets.join(", ")}\n`);
const cdp = await browser.newBrowserCDPSession();
const NAMES = ["UpdateLayoutTree", "Paint", "PrePaint", "Layerize", "Commit", "UpdateLayer"];
for (const css of ["", ...sheets]) {
  await page.evaluate((css) => {
    document.querySelector("style[data-p1]")?.remove();
    if (css === "") return;
    const sheet = document.createElement("style");
    sheet.setAttribute("data-p1", "");
    sheet.textContent = css;
    document.head.append(sheet);
  }, css);
  await page.waitForTimeout(500);
  process.stdout.write(`css: ${css || "(the pets' own)"}\n`);
  for (const kind of kinds) {
    const complete = new Promise((done) => cdp.once("Tracing.tracingComplete", done));
    await cdp.send("Tracing.start", { transferMode: "ReturnAsStream", traceConfig: { recordMode: "recordAsMuchAsPossible", includedCategories: ["devtools.timeline", "disabled-by-default-devtools.timeline", "blink.user_timing"] } });
    await page.evaluate(
      (kind) =>
        new Promise((done) => {
          const all = [...document.querySelectorAll(".pet-layer svg.pet")];
          const pet = all[0];
          const groups = [...pet.querySelectorAll(":scope > g")];
          const pupil = pet.querySelector(".pet-pupil");
          const places = all.map((each) => each.style.transform);
          const kept = new Map([...document.querySelectorAll(".pet-layer svg.pet > g")].map((group) => [group, group.getAttribute("transform")]));
          const begun = performance.now();
          performance.mark("p1-sweep-start");
          let step = 0;
          const frame = () => {
            step += 1;
            const wobble = (step % 2) * 0.01;
            if (kind === "group") groups[0].setAttribute("transform", `matrix(1 0 0 1 ${wobble} 0)`);
            if (kind === "groups") for (const group of groups) group.setAttribute("transform", `matrix(1 0 0 1 ${wobble} 0)`);
            if (kind === "all") for (const group of kept.keys()) group.setAttribute("transform", `matrix(1 0 0 1 ${wobble} 0)`);
            if (kind === "allcss") for (const group of kept.keys()) group.style.transform = `matrix(1, 0, 0, 1, ${wobble}, 0)`;
            if (kind === "alldom")
              for (const group of kept.keys()) {
                const list = group.transform.baseVal;
                if (list.numberOfItems === 0) list.appendItem(group.ownerSVGElement.createSVGTransform());
                list.getItem(0).setTranslate(wobble, 0);
              }
            if (kind === "pupil") pupil.setAttribute("cx", String(wobble));
            if (kind === "eye") pupil.parentElement.parentElement.setAttribute("transform", `matrix(1 0 0 1 ${wobble} 0)`);
            if (kind === "part") groups.find((group) => group.childElementCount === 1 && !group.firstElementChild.matches("g")).setAttribute("transform", `matrix(1 0 0 1 ${wobble} 0)`);
            if (kind === "place") pet.style.transform = `${places[0]} translate(${wobble}px, 0px)`;
            if (kind === "places") all.forEach((each, index) => (each.style.transform = `${places[index]} translate(${wobble}px, 0px)`));
            if (kind === "opacity") pet.style.opacity = step % 2 === 1 ? "0.99" : "1";
            if (kind === "cursor" && step % 2 === 1) document.documentElement.setAttribute("data-pet-cursor", "grab");
            if (kind === "cursor" && step % 2 === 0) document.documentElement.removeAttribute("data-pet-cursor");
            if (kind === "mark" && step % 2 === 1) document.documentElement.setAttribute("data-p1", "grab");
            if (kind === "mark" && step % 2 === 0) document.documentElement.removeAttribute("data-p1");
            if (performance.now() - begun < 2000) requestAnimationFrame(frame);
            else {
              performance.mark("p1-sweep-end");
              all.forEach((each, index) => (each.style.transform = places[index]));
              pet.style.opacity = "1";
              for (const [group, value] of kept) {
                group.style.removeProperty("transform");
                if (group.getAttribute("style") === "") group.removeAttribute("style");
                if (value === null) group.removeAttribute("transform");
                else group.setAttribute("transform", value);
              }
              done(step);
            }
          };
          requestAnimationFrame(frame);
        }),
      kind,
    );
    await cdp.send("Tracing.end");
    const { stream } = await complete;
    const chunks = [];
    for (;;) {
      const read = await cdp.send("IO.read", { handle: stream, size: 16 * 1024 * 1024 });
      chunks.push(read.base64Encoded ? Buffer.from(read.data, "base64") : Buffer.from(read.data, "utf8"));
      if (read.eof) break;
    }
    await cdp.send("IO.close", { handle: stream });
    const events = JSON.parse(Buffer.concat(chunks).toString("utf8")).traceEvents;
    const start = events.find((event) => event.name === "p1-sweep-start");
    const end = events.find((event) => event.name === "p1-sweep-end");
    const inside = (event) => event.ph === "X" && event.ts >= start.ts && event.ts <= end.ts;
    const main = events.filter((event) => inside(event) && event.pid === start.pid && event.tid === start.tid);
    const frames = main.filter((event) => event.name === "UpdateLayoutTree").length || 1;
    const totals = Object.fromEntries(NAMES.map((name) => [name, main.filter((event) => event.name === name).reduce((sum, event) => sum + event.dur / 1000, 0)]));
    const elements = main.filter((event) => event.name === "UpdateLayoutTree").map((event) => event.args?.elementCount ?? 0);
    const gpu = events.filter((event) => inside(event) && event.name === "GPUTask").reduce((sum, event) => sum + event.dur / 1000, 0);
    const raster = events.filter((event) => inside(event) && event.name === "RasterTask").reduce((sum, event) => sum + event.dur / 1000, 0);
    process.stdout.write(`  ${kind}: frames ${frames}, restyled per frame ${elements.sort((a, b) => a - b)[Math.floor(elements.length / 2)] ?? 0}; ms per frame: ${NAMES.map((name) => `${name} ${(totals[name] / frames).toFixed(3)}`).join(", ")}, GPUTask ${(gpu / frames).toFixed(3)}, RasterTask ${(raster / frames).toFixed(3)}\n`);
  }
}
await browser.close();
