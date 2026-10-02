/** 🐕️ Ticket tool of work package J: walks the real quiz site as a first-time learner in headless Chromium and looks at
 * the pets on the way — introduction, identity, home, an opened quiz page, a run — with a screenshot and a measurement
 * of every pet at each stop. Everything the page logs as an error, every failed request and every violation of the
 * document's Content-Security-Policy is a finding.
 *
 * Usage (from the repository root):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_j_walk_site.mjs" --url http://127.0.0.1:6191 --out <dir> --name desktop-light
 *        [--width 1440] [--height 900] [--scheme light|dark] [--locale en|de] [--quiz heating] [--mobile 1] [--linger 4] [--watch 60]
 * `--watch <seconds>` stays on the home screen of the returning learner and samples twice a second whether a pet's
 * drawing overlaps a text block or a control.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6191");
const out = resolve(option("out", "pet-walk"));
const name = option("name", "walk");
const width = Number(option("width", "1440"));
const height = Number(option("height", "900"));
const scheme = option("scheme", "light");
const locale = option("locale", "en");
const quiz = option("quiz", "heating");
const mobile = option("mobile", "") === "1";
const linger = Number(option("linger", "4")) * 1000;
const watch = Number(option("watch", "0")) * 1000;
mkdirSync(out, { recursive: true });

const measure = () => {
  const layer = document.querySelector(".pet-layer");
  const boxOf = (rect) => ({ left: Math.round(rect.left), top: Math.round(rect.top), right: Math.round(rect.right), bottom: Math.round(rect.bottom) });
  const pets = [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
    let drawn = null;
    for (const child of pet.children) {
      const rect = child.getBoundingClientRect();
      if (rect.width === 0 && rect.height === 0) continue;
      drawn = drawn === null ? boxOf(rect) : { left: Math.min(drawn.left, Math.round(rect.left)), top: Math.min(drawn.top, Math.round(rect.top)), right: Math.max(drawn.right, Math.round(rect.right)), bottom: Math.max(drawn.bottom, Math.round(rect.bottom)) };
    }
    return { id: pet.getAttribute("data-pet"), transform: pet.style.transform, opacity: pet.style.opacity, drawn };
  });
  const visible = (element) => {
    if (element.closest("[inert], [hidden], .pet-layer")) return false;
    const rect = element.getBoundingClientRect();
    return rect.width > 0 && rect.height > 0 && rect.bottom > 0 && rect.right > 0 && rect.top < innerHeight && rect.left < innerWidth;
  };
  const controls = [...document.querySelectorAll("a[href], button, input, select, textarea, summary")].filter(visible);
  const texts = [...document.querySelectorAll("p, li, h1, h2, h3, h4, h5, h6, label, th, td")].filter(visible);
  const overlap = (a, b) => Math.max(0, Math.min(a.right, b.right) - Math.max(a.left, b.left)) * Math.max(0, Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top));
  const covered = [];
  for (const pet of pets) {
    if (pet.drawn === null) continue;
    for (const element of [...controls, ...texts]) {
      const box = boxOf(element.getBoundingClientRect());
      const area = overlap(pet.drawn, box);
      if (area > 0) covered.push({ pet: pet.id, tag: element.tagName.toLowerCase(), text: (element.textContent ?? "").trim().slice(0, 32), area, box });
    }
  }
  const stolen = controls.filter((element) => {
    const rect = element.getBoundingClientRect();
    const hit = document.elementFromPoint(Math.min(innerWidth - 1, Math.max(0, rect.left + rect.width / 2)), Math.min(innerHeight - 1, Math.max(0, rect.top + rect.height / 2)));
    return hit !== null && hit.closest(".pet-layer") !== null;
  }).length;
  const surfaces = [...document.querySelectorAll("#quiz-main [data-card]")].filter((element) => !element.closest("[inert], [hidden]")).map((element) => ({ card: element.getAttribute("data-card"), ...boxOf(element.getBoundingClientRect()) }));
  return {
    layer: layer === null ? null : { ariaHidden: layer.getAttribute("aria-hidden"), pointerEvents: getComputedStyle(layer).pointerEvents, zIndex: getComputedStyle(layer).zIndex, focusable: layer.querySelectorAll("a, button, input, [tabindex]").length },
    choice: document.querySelector(".quiz-app")?.getAttribute("data-pets") ?? null,
    pets,
    covered,
    stolen,
    controls: controls.length,
    surfaces,
    overflow: document.documentElement.scrollWidth - innerWidth,
    viewport: { width: innerWidth, height: innerHeight },
  };
};

const signature = () => [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => `${pet.getAttribute("data-pet")}|${pet.style.transform}|${pet.style.opacity}|${[...pet.querySelectorAll("[transform], [d], [cx]")].map((node) => `${node.getAttribute("transform") ?? ""}${node.getAttribute("d") ?? ""}${node.getAttribute("cx") ?? ""}${node.getAttribute("cy") ?? ""}`).join(";")}`).join("\n");

const browser = await chromium.launch();
const findings = { url, name, viewport: { width, height }, scheme, locale, console: [], errors: [], failed: [], violations: [], requests: [], stops: {}, notes: [] };
try {
  const context = await browser.newContext({ viewport: { width, height }, colorScheme: scheme, locale: locale === "de" ? "de-DE" : "en-GB", isMobile: mobile, hasTouch: mobile });
  await context.exposeBinding("petWalkViolation", (_source, violation) => void findings.violations.push(violation));
  await context.addInitScript(() => document.addEventListener("securitypolicyviolation", (event) => window.petWalkViolation(`${event.effectiveDirective} blocked ${event.blockedURI || "inline code"}`)));
  const page = await context.newPage();
  page.on("console", (message) => {
    if (message.type() === "error" || message.type() === "warning") findings.console.push(`${message.type()}: ${message.text()}`);
  });
  page.on("pageerror", (error) => findings.errors.push(error.message));
  page.on("requestfailed", (request) => {
    if (request.failure()?.errorText !== "net::ERR_ABORTED") findings.failed.push(`${request.method()} ${request.url()} ${request.failure()?.errorText}`);
  });
  page.on("response", (response) => {
    if (response.status() >= 400) findings.failed.push(`${response.status()} ${response.url()}`);
    if (/pets|%F0%9F%90%BE|🐾/u.test(decodeURIComponent(response.url()))) findings.requests.push(`${response.status()} ${decodeURIComponent(response.url()).replace(url, "")}`);
  });

  const stop = async (label, settle = linger) => {
    await page.waitForTimeout(settle);
    await page.screenshot({ path: resolve(out, `${name}-${label}.png`) });
    findings.stops[label] = await page.evaluate(measure);
  };
  const moving = async (label, milliseconds) => {
    const first = await page.evaluate(signature);
    let changes = 0;
    let last = first;
    const places = new Map();
    for (let waited = 0; waited < milliseconds; waited += 250) {
      await page.waitForTimeout(250);
      const next = await page.evaluate(signature);
      if (next !== last) changes += 1;
      last = next;
      for (const line of next.split("\n")) {
        const [id, transform] = line.split("|");
        if (!places.has(id)) places.set(id, new Set());
        places.get(id).add(transform);
      }
    }
    findings.stops[label] = { ...(findings.stops[label] ?? {}), changesIn: `${milliseconds} ms: ${changes} of ${milliseconds / 250} samples differ`, places: Object.fromEntries([...places].map(([id, seen]) => [id, seen.size])) };
  };
  const primary = (scope) => scope.locator('[data-overview-card-action="primary"]');
  const front = (card) => page.locator(`#quiz-main [data-card="${card}"]`).and(page.locator(":not([inert] *)"));

  const started = Date.now();
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await front("introduction").waitFor({ timeout: 150_000 });
  await page.locator(".pet-layer svg.pet").first().waitFor({ state: "attached", timeout: 30_000 }).then(
    () => findings.notes.push(`first pet on the introduction after ${Date.now() - started} ms`),
    () => findings.notes.push("no pet on the introduction within 30 s"),
  );
  await stop("1-introduction");
  await primary(front("introduction")).click();
  await front("identity").waitFor();
  await stop("2-identity", 2500);
  await front("identity").locator('input[type="radio"][value="anonymous"]').check();
  await primary(front("identity")).click();
  await page.locator("[data-layered-overview]").waitFor();
  const home = Date.now();
  await page.locator(".pet-layer svg.pet").first().waitFor({ state: "attached", timeout: 30_000 }).then(
    () => findings.notes.push(`first pet on home after ${Date.now() - home} ms`),
    () => findings.notes.push("no pet on home within 30 s"),
  );
  await stop("3-home");
  await page.mouse.move(width / 2, height / 2, { steps: 8 });
  await moving("3-home", 6000);
  await stop("3-home-later", 500);

  await page.reload({ waitUntil: "domcontentloaded" });
  await page.locator("[data-layered-overview]").waitFor({ timeout: 60_000 });
  const back = Date.now();
  await page.locator(".pet-layer svg.pet").first().waitFor({ state: "attached", timeout: 30_000 }).then(
    () => findings.notes.push(`first pet for a returning learner after ${Date.now() - back} ms`),
    () => findings.notes.push("no pet for a returning learner within 30 s"),
  );
  await stop("3b-home-returning");
  await moving("3b-home-returning", 6000);
  if (watch > 0) {
    const seen = { samples: 0, covering: 0, worst: [], walked: new Set(), shots: 0 };
    let before = new Map();
    for (let waited = 0; waited < watch; waited += 500) {
      await page.waitForTimeout(500);
      const now = await page.evaluate(measure);
      seen.samples += 1;
      for (const pet of now.pets) {
        if (before.has(pet.id) && before.get(pet.id) !== pet.transform.replace(/ scale.*/u, "")) seen.walked.add(pet.id);
        before.set(pet.id, pet.transform.replace(/ scale.*/u, ""));
      }
      if (now.covered.length === 0) continue;
      seen.covering += 1;
      if (seen.worst.length < 12) seen.worst.push(...now.covered.slice(0, 2).map((cover) => ({ at: waited, ...cover })));
      if (seen.shots < 3) await page.screenshot({ path: resolve(out, `${name}-3c-cover-${(seen.shots += 1)}.png`) });
    }
    findings.stops["3c-home-watch"] = { changesIn: `${watch} ms watched: ${seen.covering} of ${seen.samples} samples with a pet over text or a control; walked: ${[...seen.walked].join(" ") || "nobody"}`, worst: seen.worst };
    await page.screenshot({ path: resolve(out, `${name}-3c-home-watch.png`) });
  }

  await page.evaluate((id) => (window.location.hash = id), quiz);
  await page.locator(`[data-layered-pane="${quiz}"][data-opened]`).waitFor();
  await stop("4-quiz-page", Math.max(linger, 6000));
  await moving("4-quiz-page", 4000);
  await page.keyboard.press("Escape");
  await page.locator(`[data-layered-pane="${quiz}"]:not([data-opened])`).waitFor();

  const card = page.locator(`[data-layered-card="${quiz}"]`);
  await card.scrollIntoViewIfNeeded();
  await primary(card).click();
  await front("run").waitFor();
  await stop("5-run", 3000);
  await moving("5-run", 8000);
  await stop("5-run-later", 500);
  await context.close();
} catch (error) {
  findings.errors.push(`walk: ${error instanceof Error ? error.message : String(error)}`);
} finally {
  await browser.close();
}
writeFileSync(resolve(out, `${name}.json`), `${JSON.stringify(findings, null, 2)}\n`);
const brief = Object.fromEntries(Object.entries(findings.stops).map(([label, stop]) => [label, { choice: stop.choice, pets: stop.pets?.map((pet) => pet.id).join(" "), covered: stop.covered?.length, stolen: stop.stolen, overflow: stop.overflow, changesIn: stop.changesIn, places: stop.places }]));
process.stdout.write(`${JSON.stringify({ name, notes: findings.notes, console: findings.console, errors: findings.errors, failed: findings.failed, violations: findings.violations, requests: findings.requests.length, stops: brief }, null, 2)}\n`);
