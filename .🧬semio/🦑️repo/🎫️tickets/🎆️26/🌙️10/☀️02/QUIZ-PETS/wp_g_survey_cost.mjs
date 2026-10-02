/** ⏱️ Ticket tool of work package G: measures on the harness page what a survey of a large page costs in headless Chromium and where the time goes — the selector queries, the walk past inert pages, the box reads — on a page of ten sections (nine inert) with 12 cards each.
 *
 * Usage (from the repository root, harness running on 6199): node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_g_survey_cost.mjs"
 */
import { chromium } from "playwright";

const browser = await chromium.launch();
try {
  const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
  await page.goto("http://127.0.0.1:6199/?mode=still&seed=4", { waitUntil: "load" });
  await page.waitForTimeout(800);
  const result = await page.evaluate(() => {
    const KEEP = 'a[href], button, input, select, textarea, summary, [role="button"], [role="link"], [role="checkbox"], [role="radio"], [role="tab"], [role="menuitem"], [role="option"], [role="slider"], [role="switch"], [tabindex]:not([tabindex="-1"]), [contenteditable]:not([contenteditable="false"]), p, li, h1, h2, h3, h4, h5, h6, label, figcaption, td, th, legend, dt, dd, blockquote, pre, [data-pet-keepout]';
    const bulk = document.createElement("div");
    bulk.style.cssText = "position:absolute;left:0;top:60px;width:1200px;height:600px;overflow:hidden";
    for (let section = 0; section < 10; section++) {
      const pane = document.createElement("section");
      if (section > 0) pane.setAttribute("inert", "");
      for (let index = 0; index < 12; index++) {
        const card = document.createElement("article");
        card.setAttribute("data-pet-surface", "");
        card.style.cssText = "display:inline-block;width:180px;margin:4px";
        card.innerHTML = "<h3>Card</h3>" + "<p>Some words to keep free.</p><button>Go</button><a href='#x'>link</a><ul><li>one</li><li>two</li></ul>".repeat(2);
        pane.append(card);
      }
      bulk.append(pane);
    }
    document.querySelector("main").append(bulk);
    const time = (run, rounds = 200) => {
      run();
      const start = performance.now();
      for (let round = 0; round < rounds; round++) run();
      return Number(((performance.now() - start) / rounds).toFixed(4));
    };
    const matched = [...document.querySelectorAll(KEEP)];
    const live = matched.filter((element) => element.closest("[inert], [hidden]") === null);
    const measured = {
      elements: document.querySelectorAll("*").length,
      matched: matched.length,
      live: live.length,
      survey: time(() => window.petSurvey(document)),
      queryKeepouts: time(() => document.querySelectorAll(KEEP).length),
      querySurfaces: time(() => document.querySelectorAll("[data-pet-surface]").length),
      closestEach: time(() => {
        for (const element of matched) element.closest("[inert], [hidden]");
      }),
      rectsOfLive: time(() => {
        for (const element of live) element.getBoundingClientRect();
      }),
      walkerPruned: time(() => {
        const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_ELEMENT, (node) => (node.hasAttribute("inert") || node.hasAttribute("hidden") ? NodeFilter.FILTER_REJECT : NodeFilter.FILTER_ACCEPT));
        let count = 0;
        for (let node = walker.nextNode(); node !== null; node = walker.nextNode()) if (node.matches(KEEP)) count += 1;
        return count;
      }),
      queryUnseen: time(() => document.querySelectorAll("[inert], [hidden]").length),
    };
    for (const pane of bulk.querySelectorAll("[inert]")) pane.removeAttribute("inert");
    measured.liveWhenNothingIsInert = [...document.querySelectorAll(KEEP)].length;
    measured.surveyWhenNothingIsInert = time(() => window.petSurvey(document));
    bulk.remove();
    measured.surveyOfTheHarnessPageAlone = time(() => window.petSurvey(document));
    return measured;
  });
  console.log(JSON.stringify(result, null, 1));
} finally {
  await browser.close();
}
