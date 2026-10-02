/** 🔭️ Ticket tool of work package N: drives the real quiz site through a list of screens in headless Chromium and looks at
 * the pets on each of them over time — a screenshot series, a measurement per sample (where every pet stands, whether
 * its feet are on an edge the layer surveyed, whether two pets stand in each other, whether a pet covers text or a
 * control), bursts of close-ups of whoever moves, and the layer's own survey with the perches the core cuts from it.
 *
 * Usage (from the repository root, with the private stack of `wp_j_private_stack.ts` up):
 *   node ".../wp_n_drive.mjs" --out <dir> --name <label> --stops "home@20,page:heating@10,run:physics:0@8,results:physics@8"
 *        [--url http://127.0.0.1:6191] [--width 1440] [--height 900] [--mobile 1] [--scheme light|dark] [--locale en|de]
 *        [--mode calm|lively|still|off] [--every 1000] [--bursts 2] [--probe 1] [--dom <card>] [--pointer pet|x,y] [--shots all|ends|none]
 *
 * Stops: `intro`, `identity`, `home`, `home-fresh` (the overview right after the identity step, without a reload),
 * `page:<id>` (a quiz id, `intro`, `board`, `badges`, `prefs`, `learner`), `run:<quiz>[:<task index>]`,
 * `results:<quiz>`, `scroll:<pixels>` (scrolls the scrolling pane in front), each followed by `@<seconds>` to watch.
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
const out = resolve(option("out", "pet-drive"));
const name = option("name", "drive");
const width = Number(option("width", "1440"));
const height = Number(option("height", "900"));
const mobile = option("mobile", "") === "1";
const scheme = option("scheme", "light");
const locale = option("locale", "en");
const mode = option("mode", "calm");
const every = Number(option("every", "1000"));
const bursts = Number(option("bursts", "0"));
const probing = option("probe", "") === "1";
const dom = option("dom", "");
const pointer = option("pointer", "");
const shots = option("shots", "all");
const stops = option("stops", "home@10").split(",").map((entry) => {
  const [screen, seconds] = entry.split("@");
  return { screen, seconds: Number(seconds ?? "5") };
});
const repo = resolve(".").replaceAll("\\", "/");
const PETS_LABEL = { en: "Pets", de: "Tierchen" };
const CHOICES = ["off", "still", "calm", "lively"];
mkdirSync(out, { recursive: true });

const measure = async (root) => {
  const quiz = await import(`/@fs/${root}/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx`);
  const target = await import(`/@fs/${root}/🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`);
  const layer = document.querySelector(".pet-layer");
  const seen = layer === null ? null : target.survey(document, { surfaces: quiz.QUIZ_PET_SURFACES, keepouts: `${target.PET_KEEPOUTS}, ${quiz.QUIZ_PET_KEEPOUTS}`, frame: layer });
  const round = (value) => Math.round(value * 10) / 10;
  const boxOf = (rect) => ({ left: round(rect.left), top: round(rect.top), right: round(rect.right), bottom: round(rect.bottom) });
  const pets = [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
    let drawn = null;
    for (const child of pet.children) {
      const rect = child.getBoundingClientRect();
      if (rect.width === 0 && rect.height === 0) continue;
      drawn = drawn === null ? boxOf(rect) : { left: Math.min(drawn.left, round(rect.left)), top: Math.min(drawn.top, round(rect.top)), right: Math.max(drawn.right, round(rect.right)), bottom: Math.max(drawn.bottom, round(rect.bottom)) };
    }
    const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec(pet.style.transform);
    const x = place === null ? null : Number(place[1]);
    const y = place === null ? null : Number(place[2]);
    let edge = null;
    if (seen !== null && x !== null && y !== null) {
      for (const surface of seen.surfaces) {
        if (x < surface.x0 || x > surface.x1) continue;
        const gap = round(surface.y - y);
        if (edge === null || Math.abs(gap) < Math.abs(edge)) edge = gap;
      }
    }
    return { id: pet.getAttribute("data-pet"), x, y, facing: place === null ? null : Math.sign(Number(place[3])), size: place === null ? null : Number(place[4]), opacity: Number(pet.style.opacity || "1"), edge, drawn };
  });
  const visible = (element) => {
    if (element.closest("[inert], [hidden], .pet-layer")) return false;
    const rect = element.getBoundingClientRect();
    if (!(rect.width > 0 && rect.height > 0 && rect.bottom > 0 && rect.right > 0 && rect.top < innerHeight && rect.left < innerWidth)) return false;
    let [left, top, right, bottom] = [rect.left, rect.top, rect.right, rect.bottom];
    for (let node = element.parentElement; node !== null && node !== document.body; node = node.parentElement) {
      const style = getComputedStyle(node);
      if (style.overflowX === "visible" && style.overflowY === "visible") continue;
      const box = node.getBoundingClientRect();
      [left, top, right, bottom] = [Math.max(left, box.left), Math.max(top, box.top), Math.min(right, box.right), Math.min(bottom, box.bottom)];
    }
    return right > left && bottom > top;
  };
  const controls = [...document.querySelectorAll("a[href], button, input, select, textarea, summary")].filter(visible);
  const texts = [...document.querySelectorAll("p, li, h1, h2, h3, h4, h5, h6, label, th, td")].filter(visible);
  const overlap = (a, b) => Math.max(0, Math.min(a.right, b.right) - Math.max(a.left, b.left)) * Math.max(0, Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top));
  const covered = [];
  for (const pet of pets) {
    if (pet.drawn === null || pet.opacity < 0.2) continue;
    for (const element of [...controls, ...texts]) {
      const box = boxOf(element.getBoundingClientRect());
      const area = overlap(pet.drawn, box);
      if (area > 4) covered.push({ pet: pet.id, tag: element.tagName.toLowerCase(), text: (element.textContent ?? "").trim().slice(0, 28), area: Math.round(area) });
    }
  }
  const stacked = [];
  for (let first = 0; first < pets.length; first++) {
    for (let second = first + 1; second < pets.length; second++) {
      const [one, two] = [pets[first], pets[second]];
      if (one.drawn === null || two.drawn === null || one.opacity < 0.2 || two.opacity < 0.2) continue;
      const area = overlap(one.drawn, two.drawn);
      const least = Math.min((one.drawn.right - one.drawn.left) * (one.drawn.bottom - one.drawn.top), (two.drawn.right - two.drawn.left) * (two.drawn.bottom - two.drawn.top));
      if (least > 0 && area / least > 0.1) stacked.push(`${one.id}+${two.id} ${Math.round((100 * area) / least)}%`);
    }
  }
  return { pets, covered, stacked, surfaces: seen?.surfaces.length ?? 0, keepouts: seen?.keepouts.length ?? 0, overflow: document.documentElement.scrollWidth - innerWidth };
};

const probe = async (root) => {
  const quiz = await import(`/@fs/${root}/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx`);
  const target = await import(`/@fs/${root}/🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`);
  const core = await import(`/@fs/${root}/🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts`);
  const menagerie = (await import(`/@fs/${root}/🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts`)).ARCHITECTURE_MENAGERIE;
  const layer = document.querySelector(".pet-layer");
  const seen = target.survey(document, { surfaces: quiz.QUIZ_PET_SURFACES, keepouts: `${target.PET_KEEPOUTS}, ${quiz.QUIZ_PET_KEEPOUTS}`, frame: layer });
  const onStage = [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => pet.getAttribute("data-pet"));
  const size = target.petScale(seen.width);
  const wanted = onStage.length > 0 ? onStage : menagerie.casts[0].core.slice(0, target.petCapacity(seen.width));
  const kinds = menagerie.species.filter((species) => wanted.includes(species.id));
  const tallest = Math.max(0, ...kinds.map((species) => species.size.height + (species.locomotion.hover ?? 0)));
  const widest = Math.max(0, ...kinds.map((species) => species.size.width));
  const round = (value) => Math.round(value * 10) / 10;
  const scaled = { surfaces: seen.surfaces.map((surface) => ({ ...surface, x0: surface.x0 / size, x1: surface.x1 / size, y: surface.y / size })), keepouts: seen.keepouts.map((box) => ({ x: box.x / size, y: box.y / size, width: box.width / size, height: box.height / size })) };
  const perches = core.perchesOf(scaled.surfaces, scaled.keepouts, seen.width / size, seen.height / size, tallest, widest);
  return {
    stage: { width: seen.width, height: seen.height, scale: size, tallest, widest, onStage, keepouts: seen.keepouts.length },
    surfaces: seen.surfaces.map((surface) => {
      const blocking = seen.keepouts.filter((box) => box.y < surface.y && box.y + box.height > surface.y - tallest * size && box.x < surface.x1 && box.x + box.width > surface.x0);
      return {
        id: surface.id,
        x0: round(surface.x0),
        x1: round(surface.x1),
        y: round(surface.y),
        blockers: blocking.length,
        blocked: blocking.slice(0, 5).map((box) => `${round(box.x)}..${round(box.x + box.width)} @${round(box.y)}..${round(box.y + box.height)}`),
        perches: perches.filter((perch) => perch.surface === surface.id).map((perch) => `${round(perch.x0 * size)}..${round(perch.x1 * size)}`),
      };
    }),
  };
};

const describeCard = (card) => {
  const section = [...document.querySelectorAll(`#quiz-main [data-card="${card}"]`)].find((element) => element.closest("[inert]") === null);
  const describe = (element, depth) => {
    const box = element.getBoundingClientRect();
    const hooks = [...element.attributes].filter((attribute) => attribute.name.startsWith("data-")).map((attribute) => `${attribute.name}=${attribute.value}`).join(" ");
    const lines = [`${"  ".repeat(depth)}<${element.tagName.toLowerCase()} ${hooks}> ${Math.round(box.left)},${Math.round(box.top)} ${Math.round(box.width)}x${Math.round(box.height)}`];
    if (depth < 5) for (const child of element.children) lines.push(...describe(child, depth + 1));
    return lines;
  };
  return section === undefined ? `no card ${card}` : describe(section, 0).join("\n");
};

const listCards = () =>
  [...document.querySelectorAll("#quiz-main [data-card]")]
    .filter((element) => element.closest("[inert], [hidden]") === null)
    .map((element) => {
      const text = (box) => `${Math.round(box.left)},${Math.round(box.top)} ${Math.round(box.width)}x${Math.round(box.height)}`;
      const chips = [...element.querySelectorAll('[data-window-silhouette-chip][data-dock="top"]')].map((chip) => text(chip.getBoundingClientRect()));
      const bodies = [...element.querySelectorAll('[data-slot="window-chrome-body-surface"]')].map((body) => text(body.getBoundingClientRect()));
      const nested = element.querySelectorAll("[data-card]").length;
      return `${element.getAttribute("data-card")} ${text(element.getBoundingClientRect())} chips[${chips.join(" | ")}] bodies[${bodies.join(" | ")}]${nested > 0 ? ` nested=${nested}` : ""}`;
    });

const browser = await chromium.launch();
const findings = { url, name, viewport: { width, height }, mobile, scheme, locale, mode, console: [], errors: [], failed: [], stops: {} };
try {
  const context = await browser.newContext({ viewport: { width, height }, colorScheme: scheme, locale: locale === "de" ? "de-DE" : "en-GB", isMobile: mobile, hasTouch: mobile, deviceScaleFactor: Number(option("dpr", "1")) });
  const page = await context.newPage();
  page.on("console", (message) => {
    if (message.type() === "error" || message.type() === "warning") findings.console.push(`${message.type()}: ${message.text()}`);
  });
  page.on("pageerror", (error) => findings.errors.push(error.message));
  page.on("requestfailed", (request) => {
    if (request.failure()?.errorText !== "net::ERR_ABORTED") findings.failed.push(`${request.method()} ${request.url()} ${request.failure()?.errorText}`);
  });
  const sheet = await context.newPage();
  await sheet.setContent("<canvas></canvas>");

  const primary = (scope) => scope.locator('[data-overview-card-action="primary"]');
  const secondary = (scope) => scope.locator('[data-overview-card-action="secondary"]');
  const front = (card) => page.locator(`#quiz-main [data-card="${card}"]`).and(page.locator(":not([inert] *)"));
  const overview = page.locator("[data-layered-overview]");
  const state = { entered: false, moded: false };

  const strip = async (frames, columns, path) => {
    if (frames.length === 0) return;
    const data = await sheet.evaluate(
      async ([images, perRow]) => {
        const loaded = await Promise.all(images.map((source) => new Promise((done) => {
          const image = new Image();
          image.onload = () => done(image);
          image.src = source;
        })));
        const canvas = document.querySelector("canvas");
        const [w, h] = [loaded[0].width, loaded[0].height];
        canvas.width = w * Math.min(perRow, loaded.length);
        canvas.height = h * Math.ceil(loaded.length / perRow);
        const pen = canvas.getContext("2d");
        loaded.forEach((image, index) => {
          pen.drawImage(image, (index % perRow) * w, Math.floor(index / perRow) * h);
          pen.strokeStyle = "#ff00ff";
          pen.strokeRect((index % perRow) * w + 0.5, Math.floor(index / perRow) * h + 0.5, w - 1, h - 1);
        });
        return canvas.toDataURL("image/png");
      },
      [frames.map((frame) => `data:image/png;base64,${frame.toString("base64")}`), columns],
    );
    writeFileSync(path, Buffer.from(data.split(",")[1], "base64"));
  };

  const goHome = async () => {
    if ((await front("results").count()) > 0) await primary(front("results")).click();
    else if ((await front("run").count()) > 0) await secondary(front("run")).first().click();
    else if ((await page.locator("[data-layered-pane][data-opened]").count()) > 0) {
      const button = page.locator("[data-layered-overview-button]");
      if ((await button.count()) > 0) await button.click();
      else await page.keyboard.press("Escape");
    }
    await overview.waitFor();
    await page.locator("[data-layered-pane][data-opened]").waitFor({ state: "detached", timeout: 5000 }).catch(() => {});
  };

  const setMode = async () => {
    if (state.moded || mode === "calm") return;
    await page.evaluate(() => (window.location.hash = "prefs"));
    const preferences = page.locator('[data-layered-pane="prefs"]');
    await page.locator('[data-layered-pane="prefs"][data-opened]').waitFor();
    await preferences.getByRole("group", { name: PETS_LABEL[locale], exact: true }).getByRole("button").nth(CHOICES.indexOf(mode)).click();
    await page.locator(`.quiz-app[data-pets="${mode}"]`).waitFor();
    await goHome();
    state.moded = true;
  };

  const enter = async (until) => {
    if (!state.arrived) {
      await page.goto(url, { waitUntil: "domcontentloaded" });
      await front("introduction").waitFor({ timeout: 150_000 });
      state.arrived = true;
    }
    if (until === "intro") return;
    if ((await front("introduction").count()) > 0) {
      await primary(front("introduction")).click();
      await front("identity").waitFor();
    }
    if (until === "identity") return;
    await front("identity").locator('input[type="radio"][value="anonymous"]').check();
    await primary(front("identity")).click();
    await overview.waitFor();
    state.entered = true;
  };

  const answer = async () => {
    const task = front("task");
    const selects = task.locator("[data-quiz-item] select, [data-quiz-drop] select");
    for (let guard = 0; guard < 80; guard++) {
      const open = await selects.evaluateAll((elements) => elements.findIndex((element) => element.value === "" && [...element.options].some((entry) => entry.value !== "" && !entry.disabled)));
      if (open < 0) break;
      const select = selects.nth(open);
      const value = await select.evaluate((element) => [...element.options].find((entry) => entry.value !== "" && !entry.disabled)?.value ?? "");
      await select.selectOption(value);
      await page.waitForTimeout(60);
    }
    const movers = task.locator("ol > [data-quiz-item] button");
    if ((await movers.count()) > 1) {
      await movers.nth(1).click();
      await page.waitForTimeout(150);
    }
  };

  const reach = async (screen) => {
    const [kind, first, second] = screen.split(":");
    if (kind === "intro" || kind === "identity") return enter(kind);
    if (kind === "home-fresh") {
      await enter("home");
      state.fresh = true;
      return;
    }
    if (!state.entered || state.fresh) {
      if (!state.entered) await enter("home");
      state.fresh = false;
      await page.locator(".pet-layer svg.pet").first().waitFor({ state: "attached", timeout: 10_000 }).catch(() => {});
      await setMode();
      await page.reload({ waitUntil: "domcontentloaded" });
      await overview.waitFor({ timeout: 60_000 });
    }
    if (kind === "scroll") {
      await page.evaluate((pixels) => {
        const panes = [...document.querySelectorAll(".quiz-page-frame, [data-layered-list], #quiz-main *")].filter((element) => element.closest("[inert]") === null && element.scrollHeight > element.clientHeight + 4 && getComputedStyle(element).overflowY !== "visible" && getComputedStyle(element).overflowY !== "hidden");
        panes[0]?.scrollBy(0, pixels);
      }, Number(first));
      return;
    }
    await goHome();
    if (kind === "home") return;
    if (kind === "page") {
      await page.evaluate((id) => (window.location.hash = id), first);
      await page.locator(`[data-layered-pane="${first}"][data-opened]`).waitFor();
      return;
    }
    const card = page.locator(`[data-layered-card="${first}"]`);
    await card.scrollIntoViewIfNeeded();
    await primary(card).click();
    await front("run").waitFor();
    const steps = front("run").locator("nav button");
    if (kind === "run") {
      if (second !== undefined) await steps.nth(Number(second)).click();
      return;
    }
    for (let index = 0; index < (await steps.count()); index++) {
      await steps.nth(index).click();
      await page.waitForTimeout(200);
      await answer();
    }
    await primary(front("run")).click();
    await primary(page.getByRole("alertdialog")).click();
    await front("results").waitFor();
  };

  const signatureOf = (sample) => sample.pets.map((pet) => `${pet.id} ${pet.x},${pet.y} ${pet.facing}`).join("|");

  for (const [order, stop] of stops.entries()) {
    const label = `${String(order + 1).padStart(2, "0")}-${stop.screen.replaceAll(":", "-")}`;
    const record = { samples: [], notes: [] };
    findings.stops[label] = record;
    try {
      await reach(stop.screen);
    } catch (error) {
      record.notes.push(`could not reach: ${error instanceof Error ? error.message.split("\n")[0] : String(error)}`);
      await page.screenshot({ path: resolve(out, `${name}-${label}-unreached.png`) });
      continue;
    }
    if (pointer !== "" && pointer !== "pet") {
      const [x, y] = pointer.split(",").map(Number);
      await page.mouse.move(x, y, { steps: 4 });
    }
    const frames = [];
    let burstsLeft = bursts;
    let before = null;
    const count = Math.max(1, Math.round((stop.seconds * 1000) / every));
    for (let index = 0; index < count; index++) {
      await page.waitForTimeout(every);
      const sample = await page.evaluate(measure, repo);
      sample.at = Math.round((index + 1) * every);
      record.samples.push(sample);
      if (pointer === "pet" && index === Math.floor(count / 2) && sample.pets.length > 0) {
        const pet = sample.pets.find((entry) => entry.drawn !== null && entry.opacity === 1) ?? sample.pets[0];
        await page.mouse.move((pet.drawn.left + pet.drawn.right) / 2, (pet.drawn.top + pet.drawn.bottom) / 2, { steps: 6 });
        record.notes.push(`pointer rests on ${pet.id} from sample ${index + 1}`);
        const close = { x: Math.max(0, Math.min(width - 200, pet.x - 100)), y: Math.max(0, Math.min(height - 150, pet.y - 110)), width: 200, height: 150 };
        const closeups = [];
        for (let frame = 0; frame < 8; frame++) {
          closeups.push(await page.screenshot({ clip: close }));
          await page.waitForTimeout(60);
        }
        await strip(closeups, 8, resolve(out, `${name}-${label}-pointer.png`));
      }
      const shoot = shots === "all" || (shots === "ends" && (index === 0 || index === count - 1));
      if (shoot) {
        const path = resolve(out, `${name}-${label}-${String(index + 1).padStart(2, "0")}.png`);
        const shot = await page.screenshot({ path });
        frames.push(shot);
      }
      if (burstsLeft > 0 && before !== null && signatureOf(sample) !== signatureOf(before)) {
        const mover = sample.pets.find((pet) => {
          const earlier = before.pets.find((entry) => entry.id === pet.id);
          return earlier !== undefined && (earlier.x !== pet.x || earlier.y !== pet.y || earlier.facing !== pet.facing);
        });
        if (mover !== undefined && mover.x !== null) {
          const clip = { x: Math.max(0, Math.min(width - 280, mover.x - 140)), y: Math.max(0, Math.min(height - 170, mover.y - 120)), width: Math.min(280, width), height: 170 };
          const burst = [];
          for (let frame = 0; frame < 8; frame++) {
            burst.push(await page.screenshot({ clip }));
            await page.waitForTimeout(100);
          }
          await strip(burst, 4, resolve(out, `${name}-${label}-burst-${bursts - burstsLeft + 1}-${mover.id}.png`));
          record.notes.push(`burst ${bursts - burstsLeft + 1} follows ${mover.id} from sample ${index + 1}`);
          burstsLeft -= 1;
        }
      }
      before = sample;
    }
    if (option("zoom", "") === "1" && record.samples.length > 0) {
      const closeups = [];
      for (const pet of (await page.evaluate(measure, repo)).pets) {
        if (pet.x === null) continue;
        closeups.push(await page.screenshot({ clip: { x: Math.max(0, Math.min(width - 150, pet.x - 75)), y: Math.max(0, Math.min(height - 120, pet.y - 90)), width: Math.min(150, width), height: 120 } }));
      }
      await strip(closeups, 6, resolve(out, `${name}-${label}-zoom.png`));
    }
    if (probing) record.probe = await page.evaluate(probe, repo);
    if (dom !== "") record.dom = await page.evaluate(describeCard, dom);
    if (option("cards", "") === "1") record.cards = await page.evaluate(listCards);
    const all = record.samples;
    record.summary = {
      pets: [...new Set(all.flatMap((sample) => sample.pets.map((pet) => pet.id)))].join(" "),
      most: Math.max(0, ...all.map((sample) => sample.pets.length)),
      last: all.at(-1)?.pets.map((pet) => `${pet.id}@${pet.x},${pet.y}${pet.edge === 0 ? "" : ` edge${pet.edge}`}${pet.opacity === 1 ? "" : ` o${pet.opacity}`}`).join("  "),
      stackedSamples: all.filter((sample) => sample.stacked.length > 0).length,
      stacked: [...new Set(all.flatMap((sample) => sample.stacked.map((entry) => entry.split(" ")[0])))].join(" "),
      coveredSamples: all.filter((sample) => sample.covered.length > 0).length,
      covered: [...new Set(all.flatMap((sample) => sample.covered.map((entry) => `${entry.pet}>${entry.tag}:${entry.text}`)))].slice(0, 8),
      offEdge: [...new Set(all.flatMap((sample) => sample.pets.filter((pet) => pet.opacity === 1 && pet.edge !== null && Math.abs(pet.edge) > 0.5).map((pet) => `${pet.id}:${pet.edge}`)))].slice(0, 12),
      moved: [...new Set(all.flatMap((sample, index) => (index === 0 ? [] : sample.pets.filter((pet) => all[index - 1].pets.some((entry) => entry.id === pet.id && (entry.x !== pet.x || entry.y !== pet.y))).map((pet) => pet.id))))].join(" "),
      overflow: Math.max(0, ...all.map((sample) => sample.overflow)),
    };
  }
  await context.close();
} catch (error) {
  findings.errors.push(`drive: ${error instanceof Error ? error.message : String(error)}`);
} finally {
  await browser.close();
}
writeFileSync(resolve(out, `${name}.json`), `${JSON.stringify(findings, null, 1)}\n`);
process.stdout.write(`${JSON.stringify({ name, console: findings.console, errors: findings.errors, failed: findings.failed, stops: Object.fromEntries(Object.entries(findings.stops).map(([label, stop]) => [label, { notes: stop.notes, ...stop.summary, probe: stop.probe, dom: stop.dom, cards: stop.cards }])) }, null, 1)}\n`);
