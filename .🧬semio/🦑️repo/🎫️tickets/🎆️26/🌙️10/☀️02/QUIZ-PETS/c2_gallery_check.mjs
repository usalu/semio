/** 🎠️ Ticket tool of work package C2: drives the stories gallery of `@semio-tech/pets-react` in headless Chromium and records what it sees — every species page (each section on its light and dark ground; screenshots), the sandbox (light and dark), three clicks on a pet, a pet dragged and thrown high, the pointer circling round sunny, and the inspector forcing a state, a trick, a mood, an activity and a drop from high — with the readout (poofs, overlaps, particles, rate) and the `data-pet-*` attributes of the pets. Others edit the core while it runs, so the reloads and module updates the dev server pushes are held back from the page (counted in `reloads`): the page keeps the code it loaded. Every console message and page error is collected; what the dev tooling itself prints — the Vite client's `[vite]` lines and react-dom's development hint about its DevTools — is listed apart (`vite`). Prints one JSON report and writes it to `<out>/report.json`; exit code 1 when an expectation fails.
 *
 * Usage (from the repository root, gallery served on 6253 for the architecture menagerie):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c2_gallery_check.mjs" [--url http://127.0.0.1:6253/] [--out <dir>] [--part species|close|sandbox|all] [--zoom 2] [--close sunny,housy]
 *
 * `species` sweeps every page at `--zoom` (one full-page shot each), `close` shoots the states, tricks and getting-around
 * sections of the `--close` species at size 4 on both grounds, `sandbox` drives the sandbox. `--roster <n>` is how many
 * species the menagerie has (20 for the architecture menagerie).
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const here = dirname(fileURLToPath(import.meta.url));
const args = process.argv.slice(2);
const option = (name, fallback) => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : fallback);
const base = option("url", "http://127.0.0.1:6253/");
const out = resolve(option("out", join(here, "🗑️generated", "c2", "browser")));
const part = option("part", "all");
const zoom = Number(option("zoom", "2"));
const close = option("close", "sunny,housy,thermy,solary").split(",").filter((id) => id !== "");
mkdirSync(out, { recursive: true });

const report = { console: [], vite: [], errors: [], species: {}, sandbox: {} };
const failed = [];
const expect = (name, holds, detail) => {
  if (!holds) failed.push(`${name}: ${JSON.stringify(detail)}`);
};
const pause = (milliseconds) => new Promise((done) => setTimeout(done, milliseconds));

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-US", colorScheme: "light", deviceScaleFactor: 1 });
report.reloads = 0;
await context.routeWebSocket(/./, (socket) => {
  const server = socket.connectToServer();
  server.onMessage((message) => {
    if (typeof message === "string" && /"type":"(full-reload|update)"/u.test(message)) {
      report.reloads += 1;
      return;
    }
    socket.send(message);
  });
});
const page = await context.newPage();
page.on("console", (message) => (message.text().startsWith("[vite]") || message.text().includes("Download the React DevTools") ? report.vite : report.console).push(`${message.type()}: ${message.text()}`));
page.on("pageerror", (error) => report.errors.push(String(error)));

/** 🎚️ Sets a range input of the controls by its label to `value` the way React hears it. */
const slide = async (label, value) => {
  await page.evaluate(
    ([name, wanted]) => {
      const control = [...document.querySelectorAll("label.control")].find((candidate) => candidate.querySelector("span")?.textContent === name);
      const input = control?.querySelector("input[type=range]");
      if (!input) throw new Error(`no slider ${name}`);
      Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value").set.call(input, String(wanted));
      input.dispatchEvent(new Event("input", { bubbles: true }));
    },
    [label, value],
  );
};

/** 🧾️ The readout of the sandbox: its numbers and its actors with their boxes in viewport pixels. */
const readout = () =>
  page.evaluate(() => {
    const numbers = Object.fromEntries([...document.querySelectorAll("[data-reading]")].map((entry) => [entry.getAttribute("data-reading"), entry.getAttribute("data-value")]));
    const actors = [...document.querySelectorAll(".readout tr[data-species]")].map((row) => {
      const [x, y, width, height] = row.getAttribute("data-body").split(" ").map(Number);
      return { species: row.dataset.species, footing: row.dataset.footing, activity: row.dataset.activity, state: row.dataset.state, mood: row.dataset.mood, x, y, width, height };
    });
    return { numbers, actors };
  });

/** 🐾️ What the drawing of a species in the layer says about it now (`null` when it is not drawn). */
const drawn = (species) =>
  page.evaluate((id) => {
    const svg = document.querySelector(`.pet-layer svg.pet[data-pet="${id}"]`);
    if (!svg) return null;
    return { activity: svg.getAttribute("data-pet-activity"), footing: svg.getAttribute("data-pet-footing"), state: svg.getAttribute("data-pet-state"), mood: svg.getAttribute("data-pet-mood"), opacity: svg.style.opacity };
  }, species);

/** ⏱️ Samples `drawn(species)` every `every` ms for `span` ms and keeps every change. */
const watch = async (species, span, every = 50) => {
  const seen = [];
  const until = Date.now() + span;
  while (Date.now() < until) {
    const now = await drawn(species);
    const text = now === null ? "gone" : `${now.footing}/${now.activity}/${now.state}/${now.mood}`;
    if (seen.at(-1) !== text) seen.push(text);
    await pause(every);
  }
  return seen;
};

const centreOf = (actor) => ({ x: actor.x + actor.width / 2, y: actor.y + actor.height / 2 });

await page.goto(base, { waitUntil: "networkidle" });
await page.waitForSelector(".roster a");
const roster = await page.$$eval(".roster a", (links) => links.map((link) => link.getAttribute("href").slice(1)));
report.roster = roster;
expect("the roster lists every species", roster.length === Number(option("roster", "20")), roster.length);

if (part === "species" || part === "all") {
  await slide("Size", zoom);
  for (const id of roster) {
    await page.evaluate((species) => (window.location.hash = species), id);
    await page.waitForSelector(`.story[data-species="${id}"]`);
    const height = await page.evaluate(() => document.documentElement.scrollHeight);
    await page.setViewportSize({ width: 1440, height: Math.min(height, 7000) });
    await pause(900);
    const facts = await page.evaluate((species) => {
      const story = document.querySelector(`.story[data-species="${species}"]`);
      const sections = Object.fromEntries([...story.querySelectorAll(".story-section")].map((section) => [section.dataset.section, section.querySelector(".ground-light").querySelectorAll(".specimen").length]));
      const painted = [...story.querySelectorAll(".specimen-stage svg.pet")].filter((svg) => svg.style.transform !== "").length;
      const tools = [...story.querySelectorAll('.pet-gear [visibility="visible"]')].map((tool) => tool.getAttribute("data-pet-tool"));
      const particles = story.querySelectorAll('.pet-effects [data-pet-emitter][visibility="visible"]').length;
      const ladders = story.querySelectorAll(".pet-ladders g").length;
      const notes = [...story.querySelectorAll(".story-note")].map((note) => note.textContent);
      return { sections, specimens: story.querySelectorAll(".specimen").length, painted, tools: [...new Set(tools)].sort(), particles, ladders, notes };
    }, id);
    report.species[id] = facts;
    expect(`${id}: every specimen is painted`, facts.painted === facts.specimens, facts);
    await page.screenshot({ path: join(out, `species-${id}.png`), fullPage: true });
    await page.setViewportSize({ width: 1440, height: 900 });
  }
}

if (part === "close" || part === "all") {
  await slide("Size", 4);
  for (const id of close) {
    await page.evaluate((species) => (window.location.hash = species), id);
    await page.waitForSelector(`.story[data-species="${id}"]`);
    for (const section of ["states", "tricks", "around"]) {
      for (const ground of ["light", "dark"]) {
        const locator = page.locator(`.story[data-species="${id}"] .story-section[data-section="${section}"] .ground-${ground}`);
        const box = await locator.boundingBox();
        await page.setViewportSize({ width: 1440, height: Math.max(900, Math.min(Math.ceil(box?.height ?? 900) + 160, 5000)) });
        await locator.scrollIntoViewIfNeeded();
        await pause(700);
        await locator.screenshot({ path: join(out, `close-${id}-${section}-${ground}.png`) });
      }
    }
    const poof = page.locator(`.story[data-species="${id}"] .ground-light .specimen[data-scene="poof"]`);
    await poof.evaluate((element) => element.scrollIntoView({ block: "center" }));
    const dusty = (species) => document.querySelector(`.story[data-species="${species}"] .ground-light .specimen[data-scene="poof"] [data-pet-puff][visibility="visible"]`) !== null;
    let appeared = true;
    for (const [index, wait] of [0, 240, 480].entries()) {
      await page.waitForFunction((species) => !document.querySelector(`.story[data-species="${species}"] .ground-light .specimen[data-scene="poof"] [data-pet-puff][visibility="visible"]`), id, { timeout: 6000 }).catch(() => null);
      appeared = appeared && (await page.waitForFunction(dusty, id, { timeout: 6000 }).then(() => true, () => false));
      await pause(wait);
      await slide("Speed", 0);
      await pause(120);
      await poof.screenshot({ path: join(out, `poof-${id}-${index}.png`) });
      await slide("Speed", 1);
    }
    report.species[id] = { ...(report.species[id] ?? {}), dust: appeared };
    expect(`${id}: the poof leaves dust`, appeared, appeared);
    await page.setViewportSize({ width: 1440, height: 900 });
  }
  await page.evaluate(() => window.scrollTo(0, 0));
}

if (part === "sandbox" || part === "all") {
  await page.getByRole("button", { name: "Sandbox" }).click();
  await page.waitForSelector(".pet-layer");
  await page.waitForFunction(() => document.querySelectorAll(".readout tr[data-species]").length >= 3, null, { timeout: 20000 });
  await pause(6000);
  let reading = await readout();
  report.sandbox.start = reading;
  await page.screenshot({ path: join(out, "sandbox-light.png") });
  const fixtures = await page.$$eval("[data-pet-prop]", (rows) => rows.map((row) => row.getAttribute("data-pet-prop")));
  report.sandbox.props = fixtures;
  expect("the task card has rows with topic keys", fixtures.length > 0, fixtures);

  const perched = (except = []) => reading.actors.filter((actor) => actor.footing === "perch" && !except.includes(actor.species) && actor.activity !== "walk");
  const clickTarget = perched(["sunny"])[0] ?? perched()[0];
  report.sandbox.clicks = { species: clickTarget?.species, seen: [] };
  if (clickTarget) {
    for (let click = 0; click < 3; click++) {
      reading = await readout();
      const now = reading.actors.find((actor) => actor.species === clickTarget.species) ?? clickTarget;
      const point = centreOf(now);
      await page.mouse.move(point.x, point.y);
      await page.mouse.down();
      await page.mouse.up();
      report.sandbox.clicks.seen.push(await watch(clickTarget.species, 900));
    }
    await page.screenshot({ path: join(out, "sandbox-after-clicks.png") });
  }

  reading = await readout();
  const thrown = perched(["sunny", clickTarget?.species])[0] ?? perched([clickTarget?.species])[0];
  report.sandbox.throw = { species: thrown?.species };
  if (thrown) {
    const from = centreOf(thrown);
    await page.mouse.move(from.x, from.y);
    await page.mouse.down();
    for (let step = 1; step <= 12; step++) {
      await page.mouse.move(from.x + step * 4, from.y - step * 10);
      await pause(16);
    }
    await page.screenshot({ path: join(out, "sandbox-held.png") });
    report.sandbox.throw.held = await drawn(thrown.species);
    for (let step = 1; step <= 6; step++) {
      await page.mouse.move(from.x + 48 + step * 10, from.y - 120 - step * 60);
      await pause(16);
    }
    await page.mouse.up();
    const flight = watch(thrown.species, 7000, 40);
    await pause(450);
    await page.screenshot({ path: join(out, "sandbox-thrown.png") });
    report.sandbox.throw.seen = await flight;
    await page.screenshot({ path: join(out, "sandbox-landed.png") });
  }

  await page.locator("label.control", { hasText: "Scene" }).locator("select").selectOption("physics");
  await page.waitForFunction(() => document.querySelector('.readout tr[data-species="sunny"][data-footing="perch"]') !== null, null, { timeout: 30000 }).catch(() => null);
  await pause(4000);
  reading = await readout();
  report.sandbox.physics = reading.actors.map((actor) => `${actor.species}:${actor.footing}/${actor.activity}`);
  const sunny = reading.actors.find((actor) => actor.species === "sunny" && actor.footing === "perch");
  report.sandbox.circle = { found: sunny !== undefined };
  if (sunny) {
    await page.locator("label.control", { hasText: "Mode" }).locator("select").selectOption("calm");
    const radius = Math.max(sunny.width, sunny.height) * 0.75 + 12;
    const before = await drawn("sunny");
    report.sandbox.circle = { before, radius, rounds: [] };
    for (let round = 0; round < 3; round++) {
      await page.waitForFunction(() => document.querySelector('.readout tr[data-species="sunny"][data-activity="idle"][data-footing="perch"]') !== null, null, { timeout: 15000 }).catch(() => null);
      let middle = centreOf((await readout()).actors.find((actor) => actor.species === "sunny") ?? sunny);
      const circling = watch("sunny", 5000, 50);
      for (let step = 0; step <= 3 * 48; step++) {
        if (step % 8 === 0) middle = centreOf((await readout()).actors.find((actor) => actor.species === "sunny") ?? sunny);
        const angle = (step / 48) * 2 * Math.PI;
        await page.mouse.move(middle.x + radius * Math.cos(angle), middle.y + radius * Math.sin(angle));
        await pause(18);
      }
      await page.mouse.move(middle.x + radius * 3, middle.y + radius * 2);
      const seen = await circling;
      report.sandbox.circle.rounds.push({ middle, seen });
      if (round === 0) await page.screenshot({ path: join(out, "sandbox-circled.png") });
      if (seen.some((entry) => entry.includes("/trick/")) || seen.some((entry) => entry.split("/")[2] !== before?.state)) break;
    }
    await page.locator("label.control", { hasText: "Mode" }).locator("select").selectOption("lively");
  }

  const inspect = async (actor) => {
    await page.locator(".inspect label.control", { hasText: "Pet" }).locator("select").selectOption(actor);
    await pause(300);
  };
  reading = await readout();
  const subject = reading.actors.find((actor) => actor.species === "sunny" && actor.footing === "perch") ?? perched()[0];
  report.sandbox.forced = { species: subject?.species };
  if (subject) {
    await inspect(subject.species);
    const states = await page.locator(".inspect label.control", { hasText: "State" }).locator("option").allTextContents();
    const wanted = states.find((state) => state !== subject.state) ?? states[0];
    await page.locator(".inspect label.control", { hasText: "State" }).locator("select").selectOption(wanted);
    await page.getByRole("button", { name: "Enter" }).click();
    await pause(700);
    report.sandbox.forced.state = { wanted, drawn: await drawn(subject.species) };
    expect("a forced state is drawn", report.sandbox.forced.state.drawn?.state === wanted, report.sandbox.forced.state);
    await page.locator(".inspect label.control", { hasText: "Mood" }).locator("select").selectOption("grumpy");
    await page.getByRole("button", { name: "Feel" }).click();
    await pause(500);
    report.sandbox.forced.mood = await drawn(subject.species);
    expect("a forced mood is drawn", report.sandbox.forced.mood?.mood === "grumpy", report.sandbox.forced.mood);
    await page.screenshot({ path: join(out, "sandbox-forced-state.png") });
    const forcedTrick = watch(subject.species, 1500, 40);
    await page.getByRole("button", { name: "Perform" }).click();
    report.sandbox.forced.trick = await forcedTrick;
    expect("a forced trick is performed", report.sandbox.forced.trick.some((entry) => entry.includes("/trick/")), report.sandbox.forced.trick);
    await pause(2500);
    await page.locator(".inspect label.control", { hasText: "Activity" }).locator("select").selectOption("dizzy");
    const forcedActivity = watch(subject.species, 1200, 40);
    await page.getByRole("button", { name: "Do it" }).click();
    report.sandbox.forced.activity = await forcedActivity;
    expect("a forced activity is played", report.sandbox.forced.activity.some((entry) => entry.includes("/dizzy/")), report.sandbox.forced.activity);
    await pause(2500);
    const dropped = watch(subject.species, 7000, 40);
    await page.getByRole("button", { name: "Drop from high (chute)" }).click();
    await pause(900);
    await page.screenshot({ path: join(out, "sandbox-dropped-high.png") });
    report.sandbox.forced.drop = await dropped;
    report.sandbox.forced.news = await page.locator(".inspect-news").textContent().catch(() => null);
    await pause(2500);
    const picked = watch(subject.species, 3500, 40);
    await page.getByRole("button", { name: "Pick up (hand)" }).click();
    await pause(700);
    await page.screenshot({ path: join(out, "sandbox-picked-up.png") });
    report.sandbox.forced.pickUp = await picked;
    await pause(1500);
    const headed = watch(subject.species, 5000, 40);
    await page.getByRole("button", { name: "Drop on a neighbour (head)" }).click();
    report.sandbox.forced.onHead = await headed;
    await pause(2000);
    const tossed = watch(subject.species, 7000, 40);
    await page.getByRole("button", { name: "Toss" }).click();
    await pause(1200);
    await page.screenshot({ path: join(out, "sandbox-tossed.png") });
    report.sandbox.forced.toss = await tossed;
  }

  await page.getByRole("checkbox", { name: "Dark page" }).check();
  await pause(1500);
  await page.screenshot({ path: join(out, "sandbox-dark.png") });
  await pause(4000);
  reading = await readout();
  report.sandbox.end = reading;
  expect("no two bodies overlapped in any frame", reading.numbers.overlapped === "0" && reading.numbers.overlaps === "0", reading.numbers);
}

expect("the console stays empty", report.console.length === 0, report.console);
expect("no page error", report.errors.length === 0, report.errors);
report.failed = failed;
writeFileSync(join(out, "report.json"), JSON.stringify(report, null, 2));
process.stdout.write(`${JSON.stringify({ failed, console: report.console, vite: report.vite, errors: report.errors, sandbox: { start: report.sandbox.start?.numbers, end: report.sandbox.end?.numbers, clicks: report.sandbox.clicks, throw: report.sandbox.throw, circle: report.sandbox.circle, forced: report.sandbox.forced } }, null, 2)}\n`);
await browser.close();
process.exitCode = failed.length === 0 ? 0 : 1;
