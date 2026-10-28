/** 🐕️ The pets of the site in a real browser, as the second round made them (design-v2 §22): decoration that lives on
 * the cards and answers the learner's hand. A click on a pet says hello, the next clicks ask for a trick and then for a
 * purr; a pet picked up hangs in the hand, and let go high above the page it opens its parachute, glides and lands;
 * circling the pointer round a pet changes its state; during a lively minute in which the learner drags pets over each
 * other and throws them, no two pets' bodies ever overlap; a control under a pet still receives its click; the "Show
 * pets" switch and the play group of the settings work by keyboard; a pet whose topic is a task of a quiz's page pushes a
 * copy of that task out of its place while the learner stays still, and is thrown off the moment the learner points at
 * the task or focuses it, which is back at once; on a quiz's page, whose cards leave the footer line as the only perch,
 * a climber raises its ladder against the lowest card and climbs on up its wall, and on the home screen a grappler on
 * the footer line reels itself up a rope to a card once the hand holds whoever sat there. What the first round asked still holds: the pets
 * show up on the home screen within seconds, hidden from assistive technology and never a hit target of their own,
 * they stand on what the cards show, the cast follows the learner and rests during a run, the learner's choice stops
 * them (`still`) or removes them (`off`). A device that asks for reduced motion leaves the pets as the learner set
 * them. One that forces its own colours gets none and is told why. A phone shows at most two small
 * ones, and every device stays clean throughout: no console error, no failed request, no policy violation.
 *
 * Every wait waits for a state the page shows: what a pet's depiction says it does (`data-pet-activity`), what carries
 * it (`data-pet-footing`) and the state it shows (`data-pet-state`), the hand's cursor on the document element
 * (`data-pet-cursor`), the client's own attributes. The depiction does not expose the solid body the stage keeps apart,
 * so the spec computes it: the size box of the species, mapped onto the screen through the drawing's own transform
 * (place, facing, size and tilt about the pivot). That box is the stage's body less its margin of 4 px on every side and
 * less what a canopy, a held gear or a floater's hover add, so two disjoint bodies leave these boxes at least 8 px apart;
 * the spec allows half a pixel for the transform's rounding to two decimals. The specs that wait for minutes of the
 * pets' time — calm pets that walk after a choice, a ladder and a rope — set the client's test seam `data-pets-tempo="8"` before the client
 * starts, and the mischief spec `data-pets-tempo="1"` (a prank within a minute, a copy out long enough for the hand to
 * reach it); everything else the learner's hand does runs at the wall clock, because the hand's timing (a hold, the
 * leak of a pet's warmth, the speed of a circle) is the learner's.
 * @see ../../../🐾️pets/🔣️.json — the ensemble whose casts, gear and states are expected on screen
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx — the quiz's glue to the pets
 * @see ../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🤏️grasp/🟦️.ts — how a press reaches a pet
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html — why `still` and `off` exist
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-gestures.html — why the settings offer every deed of the hand */
import {QUIZZES, card, expect, layoutQuiet, pane, primary, test, way, openQuizGrid} from "../../🎭️e2e/🚶️learner/🟦️.ts";
import { ANSWER_MS, APPEAR_MS, bodies, eventually, type Body, CARD_PARTS, carry, choosePets, CIRCLE_LAP_MS, CIRCLE_LAPS, CIRCLE_POINTS, circled, circleRound, CONTROLS, CURSOR, DEEDS, DROP_HEIGHT, EDGES, enterWithPets, expectCast, expectMoving, expectPerched, floorSpot, framesPass, FRONT_LEAST, HOME, inOrder, instrumented, type Instruments, KINDS, LANDING_MS, layer, LIVELY_MS, middleOf, misplaced, overControls, OVERLAP_SLACK, paceTo, pets, PLAY_LABEL, reachable, RECAST_MS, RELEASE_DEPTH, said, seenBoxes, takenTargets, trailOf, TEMPO } from "./🧰️pet-kit/🟦️.ts";

test("pets live on the home screen as decoration: hidden from assistive technology, never a hit target of their own, standing on what the cards show", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  await expect(learner.page.locator(".course-app")).toHaveAttribute("data-pets", "lively");
  await expect(layer(learner)).toHaveCount(1);
  await expect(layer(learner)).toHaveAttribute("aria-hidden", "true");
  await expect(layer(learner)).toHaveCSS("pointer-events", "none");
  await expect(layer(learner).locator("a, button, input, select, textarea, summary, [tabindex], [role], [id], style, script")).toHaveCount(0);
  await expectCast(learner, HOME, true);
  await expectMoving(learner);
  for (const pet of await pets(learner).all()) {
    await expect(pet).toHaveAttribute("data-pet-activity", /^[a-z]+$/u);
    await expect(pet).toHaveAttribute("data-pet-footing", /^[a-z]+$/u);
    await expect(pet).toHaveAttribute("data-pet-state", /^[a-z0-9-]+$/u);
  }
  expect(await takenTargets(learner)).toEqual([]);
  await expect.poll(() => misplaced(learner.page), { timeout: RECAST_MS }).toEqual([]);

  await primary(card(learner.page, "board")).click();
  await expect(pane(learner.page, "board")).toHaveAttribute("data-opened", "");
  expect(await takenTargets(learner)).toEqual([]);
  await way(learner.page, "overview").click();
  await expect(pane(learner.page, "board")).not.toHaveAttribute("data-opened", "");
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  expect(await learner.page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
});

test("a click on a pet says hello, the next one asks for a trick and the ones after it for a purr, and nothing beneath the pet acts", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await enterWithPets(learner);
  await instrumented(page);
  const tried = new Set<string>();
  await expect(async () => {
    const pet = await reachable(page, (body) => !tried.has(body.id));
    tried.add(pet.id);
    const click = async (): Promise<void> => {
      const now = (await bodies(page)).find((body) => body.id === pet.id);
      expect(now, `${pet.id} is still on stage`).toBeDefined();
      const middle = middleOf(now!);
      await page.mouse.click(middle.x, middle.y);
    };
    const since = Math.max((await trailOf(page, pet.id)).length - 1, 0);
    const answered = (wanted: readonly string[]): Promise<number> => trailOf(page, pet.id).then((trail) => inOrder(trail.slice(since), wanted));
    const middle = middleOf(pet);
    await page.mouse.move(middle.x, middle.y);
    await expect(page.locator("html")).toHaveAttribute(CURSOR, "grab", { timeout: ANSWER_MS });
    await click();
    await eventually(page, () => answered(["greet"]), (seen) => seen === 1, { stageMs: ANSWER_MS, message: `${pet.id} says hello` });
    await click();
    await eventually(page, () => answered(["greet", "trick"]), (seen) => seen === 2, { stageMs: ANSWER_MS, message: `${pet.id} does a trick` });
    for (let more = 0; more < 4 && (await answered(["greet", "trick", "purr"])) < 3; more++) {
      await click();
      await expect.poll(() => answered(["greet", "trick", "purr"]), { timeout: 400 }).toBe(3).catch(() => undefined);
    }
    expect(await answered(["greet", "trick", "purr"]), `${pet.id}: ${(await trailOf(page, pet.id)).join(" → ")}`).toBe(3);
    test.info().annotations.push({ type: "trail", description: `${pet.id}: ${(await trailOf(page, pet.id)).slice(since).join(" → ")}` });
  }).toPass({ timeout: RECAST_MS });
  expect(await page.evaluate(() => window.location.hash), "a press on a pet is the pet's: no card under it opened").toBe("");
});

test("a pet picked up hangs in the hand, and let go high above the page it opens its parachute, glides down and lands", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await enterWithPets(learner);
  await instrumented(page);
  const parachuted = (body: Body): boolean => KINDS[body.id]?.gear.includes("parachute") === true;
  let pet: Body | undefined;
  await expect(async () => {
    pet = await reachable(page, parachuted);
  })
    .toPass({ timeout: RECAST_MS })
    .catch(async () => {
      await page.evaluate(() => (window.location.hash = "heating"));
      await expect(pane(page, "heating")).toHaveAttribute("data-opened", "");
      await instrumented(page);
      await expect(async () => {
        pet = await reachable(page, parachuted);
      }).toPass({ timeout: RECAST_MS });
    });
  const held = pet!;
  const canopy = page.locator(`.pet-layer svg.pet[data-pet="${held.id}"] [data-pet-tool="chute"] > g > path`);
  await expect(canopy).toHaveCount(1);
  await expect(canopy).toHaveAttribute("d", KINDS[held.id]!.canopy!.d!);
  expect(await canopy.evaluate((path) => (path as SVGPathElement).getTotalLength())).toBeGreaterThan(0);
  const edges = (await page.evaluate(seenBoxes, EDGES)).filter((edge) => edge.topped);
  const top = 70;
  const width = page.viewportSize()?.width ?? 1440;
  const columns = Array.from({ length: Math.floor((width - 160) / 8) }, (_, index) => 80 + index * 8).map((x) => ({ x, floor: Math.min(...edges.filter((edge) => edge.left <= x && edge.right >= x && edge.top > top + 40).map((edge) => edge.top)) }));
  const column = columns.reduce((best, candidate) => (candidate.floor > best.floor && candidate.floor < Infinity ? candidate : best));
  const release = { x: column.x, y: Math.max(top, column.floor - DROP_HEIGHT) };
  expect(column.floor - release.y, "a fall long enough for a parachute").toBeGreaterThan(200);

  await carry(page, held, release);
  await expect(page.locator("html")).toHaveAttribute(CURSOR, "grabbing");
  await eventually(page, () => trailOf(page, held.id).then((trail) => inOrder(trail, ["hang/hand"])), (seen) => seen === 1, { stageMs: ANSWER_MS, message: `${held.id} hangs in the hand` });
  await eventually(page, async () => {
    const body = (await bodies(page)).find((body) => body.id === held.id);
    return body !== undefined && body.footing === "hand" && Math.abs(middleOf(body).x - release.x) < 32 && body.bottom < release.y + 120;
  }, (reached) => reached, { stageMs: ANSWER_MS, message: `${held.id} reaches the high release column` });
  await page.mouse.up();
  await eventually(page, () => trailOf(page, held.id).then((trail) => inOrder(trail, ["hang/hand", "glide/chute"])), (seen) => seen === 2, { stageMs: ANSWER_MS, message: `${held.id} opens its parachute` });
  await expectPerched(page, held.id, LANDING_MS);
  test.info().annotations.push({ type: "trail", description: `${held.id} let go at ${release.x}, ${release.y} above ${column.floor}: ${(await trailOf(page, held.id)).join(" → ")}` });
  expect(inOrder(await trailOf(page, held.id), ["hang/hand", "glide/chute", "idle/perch"]) >= 2, (await trailOf(page, held.id)).join(" → ")).toBe(true);
  await expect(page.locator("html")).not.toHaveAttribute(CURSOR, "grabbing");
});

test("no two pets ever overlap during a lively minute in which the learner drags them over each other and throws them", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await enterWithPets(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await choosePets(learner, "lively");
  await page.keyboard.press("Escape");
  await expect(pane(page, "prefs")).not.toHaveAttribute("data-opened", "");
  await instrumented(page);
  const watching = page.evaluate(({ milliseconds, slack }) => (window as unknown as { petWalk: Instruments }).petWalk.overlaps(milliseconds, slack), { milliseconds: LIVELY_MS, slack: OVERLAP_SLACK });
  watching.catch(() => undefined);
  const started = Date.now();
  const thrown: string[] = [];
  while (Date.now() - started < LIVELY_MS - 15_000) {
    const pet = await reachable(page).catch(() => undefined);
    if (pet === undefined) {
      await page.evaluate(framesPass, 30);
      continue;
    }
    const others = (await bodies(page)).filter((body) => body.id !== pet.id && body.opacity > 0);
    const over = others.length === 0 ? { x: 720, y: 300 } : middleOf(others[thrown.length % others.length]!);
    await carry(page, pet, over);
    const fling = { x: Math.min(Math.max(over.x + (thrown.length % 2 === 0 ? 160 : -160), 40), 1400), y: Math.max(over.y - 160, 60) };
    for (let step = 1; step <= 4; step++) await page.mouse.move(over.x + ((fling.x - over.x) * step) / 4, over.y + ((fling.y - over.y) * step) / 4);
    await page.mouse.up();
    thrown.push(pet.id);
    await expectPerched(page, pet.id, ANSWER_MS * 2).catch(() => undefined);
  }
  const watched = await watching;
  test.info().annotations.push({ type: "watched", description: `${watched.frames} frames, at most ${watched.most} pets on one, ${watched.overlaps.length} overlaps; thrown: ${thrown.join(", ")}` });
  expect(thrown.length, `pets thrown: ${thrown.join(", ")}`).toBeGreaterThanOrEqual(3);
  expect(watched.frames, "frames watched").toBeGreaterThan(300);
  expect(watched.most, "pets on one frame").toBeGreaterThanOrEqual(2);
  expect(watched.overlaps.map(({ frame, pair: [one, other] }) => `frame ${frame}: ${one.id} ${one.activity}/${one.footing} [${one.left.toFixed(1)} ${one.top.toFixed(1)} ${one.right.toFixed(1)} ${one.bottom.toFixed(1)}] × ${other.id} ${other.activity}/${other.footing} [${other.left.toFixed(1)} ${other.top.toFixed(1)} ${other.right.toFixed(1)} ${other.bottom.toFixed(1)}]`)).toEqual([]);
});

test("circling the pointer round a pet changes its state", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  const quiz = QUIZZES[1]!.id;
  await enterWithPets(learner);
  await page.evaluate((id) => (window.location.hash = id), quiz);
  await expect(pane(page, quiz)).toHaveAttribute("data-opened", "");
  await expectCast(learner, quiz, true);
  await instrumented(page);
  const tried = new Set<string>();
  const plan = async (): Promise<{ readonly body: Body; readonly cue: "circle" | "countercircle"; readonly becomes: string; readonly path: { readonly x: number; readonly y: number }[] } | undefined> => {
    const shown = (await bodies(page)).filter((body) => body.activity === "idle" && body.footing === "perch" && body.opacity > 0.99 && !tried.has(body.id));
    const plans = shown.flatMap((body) =>
      (["circle", "countercircle"] as const)
        .map((cue) => ({ body, cue, becomes: circled(KINDS[body.id]!, body.state, cue), path: circleRound(body, cue, CIRCLE_LAPS) }))
        .filter((candidate) => candidate.becomes !== candidate.body.state)
        .slice(0, 1),
    );
    for (const candidate of plans) if (!(await page.evaluate(overControls, { points: candidate.path, controls: CONTROLS })).includes(true)) return candidate;
    return undefined;
  };
  await expect(async () => {
    let chosen = await plan();
    if (chosen === undefined) {
      const spot = await page.evaluate(floorSpot, { parts: CARD_PARTS, controls: CONTROLS, reach: 70, room: 140 });
      expect(spot, "a place on the footer line with room for a circle").not.toBeNull();
      const pet = await reachable(page, (body) => !tried.has(body.id));
      await carry(page, pet, { x: spot!.x, y: spot!.top - 100 });
      await page.mouse.up();
      await expectPerched(page, pet.id, LANDING_MS);
      await eventually(page, async () => (await bodies(page)).find((body) => body.id === pet.id)?.activity, (seen) => seen === "idle", { stageMs: ANSWER_MS, message: `${pet.id} rests on the footer line` });
      chosen = await plan();
      if (chosen === undefined) throw new Error(`no pet could be circled clear of the controls, not even ${pet.id} on the footer line at ${spot!.x}`);
    }
    const { body, cue, becomes, path } = chosen;
    tried.add(body.id);
    const since = Math.max((await trailOf(page, body.id)).length - 1, 0);
    await page.mouse.move(path[0]!.x, path[0]!.y);
    const begun = await page.evaluate(() => performance.now());
    for (const [index, point] of path.entries()) {
      await page.mouse.move(point.x, point.y);
      await page.evaluate(paceTo, begun + (index * CIRCLE_LAP_MS) / CIRCLE_POINTS);
    }
    await page.mouse.move(path[0]!.x + 200, Math.max(path[0]!.y - 200, 4), { steps: 3 });
    await expect
      .poll(async () => ({ state: (await bodies(page)).find((candidate) => candidate.id === body.id)?.state, trail: (await trailOf(page, body.id)).slice(since) }), { message: `${body.id} circled ${cue} from ${body.state}`, timeout: ANSWER_MS })
      .toMatchObject({ state: becomes, trail: expect.arrayContaining([expect.stringMatching(/^trick\//u)]) });
    test.info().annotations.push({ type: "trail", description: `${body.id} circled ${cue}: ${(await trailOf(page, body.id)).slice(since).join(" → ")}` });
  }).toPass({ timeout: 2 * RECAST_MS });
});

test("keyboard gestures reach the real stage: circle and countercircle change a rung, stroke purrs, and shake changes state immediately", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await enterWithPets(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await instrumented(page);
  const group = pane(page, "prefs").getByRole("group", { name: PLAY_LABEL[learner.locale], exact: true });
  for (const deed of ["circle", "countercircle", "stroke", "shake"] as const) {
    for (const key of ["Enter", "Space"] as const) {
      await expect(async () => {
        const pet = await reachable(page, (body) => deed === "stroke" || circled(KINDS[body.id]!, body.state, deed) !== body.state);
        const name = KINDS[pet.id]!.name[learner.locale];
        const player = group.getByRole("group", { name, exact: true });
        const button = player.getByRole("button", { name: DEEDS[deed].button[learner.locale], exact: true });
        const becomes = deed === "stroke" ? pet.state : circled(KINDS[pet.id]!, pet.state, deed);
        const since = (await trailOf(page, pet.id)).length;
        await button.focus();
        await page.keyboard.press(key);
        await expect(button).toBeFocused();
        await expect(group.getByRole("status")).toHaveText(said(deed, name, learner.locale));
        await expect.poll(async () => ({
          state: (await bodies(page)).find((body) => body.id === pet.id)?.state,
          answered: deed === "shake" || (await trailOf(page, pet.id)).slice(since).some((line) => line.startsWith(deed === "stroke" ? "purr/" : "trick/")),
        }), { message: `${key} ${deed}: ${pet.id} ${pet.state} → ${becomes}`, timeout: ANSWER_MS }).toEqual({ state: becomes, answered: true });
        test.info().annotations.push({ type: "keyboard-stage", description: `${key} ${deed}: ${pet.id} ${pet.state} → ${becomes}; ${(await trailOf(page, pet.id)).slice(since).join(" → ")}` });
      }).toPass({ timeout: RECAST_MS });
    }
  }
});

/** 📏️ How tall the window of the deeds spec is: the row of deeds exists only while a pet is on stage, and a pet needs a perch with room above it. At 200 % text the preferences card is as tall as its content and, once that exceeds the room between the bar and the footer, as tall as the room itself (it scrolls inside): there is then no ground at all, whatever the window, and pets are right to stay away. A window tall enough for the card with its rows of deeds leaves a free band above the footer, so a pet stands there for good instead of by the luck of arriving while the card still opens. */
const DEED_HEIGHT = 2_400;

/** 🎡️ How much of the pets' time the deeds spec waits for a pet to stand on stage after the text doubled: the cast rotates every 60 to 120 seconds, so a wait of four rotations. */
const CAST_WAIT_MS = 4 * 120_000;

/** ⏳️ The hard limit of one deeds spec, only there so that a hang ends: four rotations of the cast in the pets' time, and the walk. */
const DEED_TIMEOUT_MS = 20 * 60_000;

for (const locale of ["en", "de"] as const) {
  for (const width of [320, 375]) {
    test(`all eight pet deeds fit and remain in tab order at ${width}px with 200% text and the longest authored ${locale} name`, async ({ device }) => {
      const learner = await device(locale);
      const page = learner.page;
      test.setTimeout(DEED_TIMEOUT_MS);
      await page.setViewportSize({ width, height: DEED_HEIGHT });
      await enterWithPets(learner);
      await page.evaluate(() => (window.location.hash = "prefs"));
      await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
      const group = pane(page, "prefs").getByRole("group", { name: PLAY_LABEL[locale], exact: true });
      const player = group.locator("[data-pets-player]").first();
      await expect(player).toBeVisible();
      const id = await player.getAttribute("data-pets-player");
      const longest = Object.values(KINDS).map((kind) => kind.name[locale]).reduce((one, other) => one.length >= other.length ? one : other);
      await page.evaluate(() => {
        const root = document.documentElement;
        root.style.fontSize = `${Number.parseFloat(getComputedStyle(root).fontSize) * 2}px`;
      });
      const buttons = player.getByRole("button");
      const prepare = async (): Promise<boolean> => {
        await eventually(page, () => player.count(), (shown) => shown > 0, { stageMs: CAST_WAIT_MS, message: `${locale} ${width}: a pet is on stage, so the row of deeds is there` });
        try {
          await layoutQuiet(page);
          const problems = await player.evaluate(
            (element, [name, texts]) => {
              const label = element.querySelector("span[id]")!;
              label.textContent = name;
              const named = (element.getAttribute("aria-labelledby") ?? "").split(" ").map((id) => element.ownerDocument.getElementById(id)?.textContent ?? "").join(" ").trim();
              const buttons = [...element.querySelectorAll("button")];
              const boxes = buttons.map((button) => button.getBoundingClientRect());
              const found: string[] = [];
              if (named !== name) found.push(`the row is named ${JSON.stringify(named)}, not ${JSON.stringify(name)}`);
              if (label.scrollWidth - label.clientWidth > 1) found.push("the longest authored name does not wrap inside its column");
              const shown = buttons.map((button) => (button.textContent ?? "").replace(/\s+/g, " ").trim());
              if (shown.length !== texts.length || shown.some((text, index) => text !== texts[index])) found.push(`the buttons read ${JSON.stringify(shown)}`);
              const overlaps = boxes.flatMap((one, index) => boxes.slice(index + 1).flatMap((other, offset) => Math.min(one.right, other.right) - Math.max(one.left, other.left) > 0.5 && Math.min(one.bottom, other.bottom) - Math.max(one.top, other.top) > 0.5 ? [[index, index + offset + 1]] : []));
              if (overlaps.length > 0) found.push(`the button hit areas overlap: ${JSON.stringify(overlaps)}`);
              if (found.length === 0) {
                element.setAttribute("data-walk", "on");
                for (const button of buttons) button.setAttribute("data-walk", "on");
              }
              return found;
            },
            [longest, Object.values(DEEDS).map((deed) => deed.button[locale])] as const,
          );
          expect(problems, `${locale} ${width}: the row of deeds, checked in one step (the cast may re-form between two steps)`).toEqual([]);
          return true;
        } catch (failure) {
          if ((await player.count()) === 0) return false;
          throw failure;
        }
      };
      const standing = (): Promise<boolean> => player.evaluate((element) => element.getAttribute("data-walk") === "on" && [...element.querySelectorAll("button")].every((button) => button.getAttribute("data-walk") === "on")).catch(() => false);
      const focused = (index: number): Promise<boolean> => buttons.nth(index).evaluate((element) => element === document.activeElement).catch(() => false);
      const SUBPIXEL = 0.5;
      for (const direction of ["Tab", "Shift+Tab"] as const) {
        const indices = direction === "Tab" ? [0, 1, 2, 3, 4, 5, 6, 7] : [7, 6, 5, 4, 3, 2, 1, 0];
        for (let attempt = 0; ; attempt++) {
          expect(attempt, "the pets re-formed the cast (the row of buttons is mounted anew) too often for the walk to finish").toBeLessThan(8);
          if (!(await prepare())) continue;
          await buttons.nth(indices[0]!).focus({ timeout: 5_000 }).catch(() => undefined);
          let remounted = false;
          for (const [step, index] of indices.entries()) {
            if (step > 0) await page.keyboard.press(direction);
            const button = buttons.nth(index);
            const taken = await expect.poll(async () => ((await focused(index)) ? "focused" : (await standing()) ? "waiting" : "remounted"), { message: `${locale} ${width} ${index}: the button takes the focus`, timeout: 5_000 }).not.toBe("waiting").then(() => true, () => false);
            if (!taken || !(await focused(index))) {
              remounted = true;
              break;
            }
            const bounds = await button.evaluate((element) => {
              const box = element.getBoundingClientRect();
              const style = getComputedStyle(element);
              const inset = Number.parseFloat(style.outlineWidth) + Math.max(0, Number.parseFloat(style.outlineOffset));
              return { left: box.left, right: box.right, top: box.top, bottom: box.bottom, width: box.width, height: box.height, outline: Number.isFinite(inset) ? inset : 0, scroll: element.scrollWidth, client: element.clientWidth, seen: (Math.max(0, Math.min(box.right, window.innerWidth) - Math.max(box.left, 0)) * Math.max(0, Math.min(box.bottom, window.innerHeight) - Math.max(box.top, 0))) / (box.width * box.height) };
            }).catch(() => undefined);
            if (bounds === undefined || !Number.isFinite(bounds.seen)) {
              remounted = true;
              break;
            }
            expect(bounds.seen, `${locale} ${width} ${index}: the button lies in the viewport`).toBeGreaterThanOrEqual(0.98);
            expect(bounds.left - bounds.outline, `${locale} ${width} ${index}: focus outline left`).toBeGreaterThanOrEqual(-SUBPIXEL);
            expect(bounds.right + bounds.outline, `${locale} ${width} ${index}: focus outline right`).toBeLessThanOrEqual(width + SUBPIXEL);
            expect(bounds.top - bounds.outline, `${locale} ${width} ${index}: focus outline top`).toBeGreaterThanOrEqual(-SUBPIXEL);
            expect(bounds.bottom + bounds.outline, `${locale} ${width} ${index}: focus outline bottom`).toBeLessThanOrEqual(DEED_HEIGHT + SUBPIXEL);
            expect(bounds.width).toBeGreaterThanOrEqual(24);
            expect(bounds.height).toBeGreaterThanOrEqual(24);
            expect(bounds.scroll, `${locale} ${width} ${index}: label fits its button`).toBeLessThanOrEqual(bounds.client + 1);
            test.info().annotations.push({ type: "pet-control-bounds", description: JSON.stringify({ locale, name: longest, id, direction, index, ...bounds }) });
          }
          if (!remounted) break;
        }
      }
      expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
    });
  }
}

test("a control under a pet still receives its click: the card a falling pet passes in front of opens, and the pet is not picked up", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await enterWithPets(learner);
  await openQuizGrid(learner, "physics");
  await instrumented(page);
  const fronts = await Promise.all(QUIZZES.map(async (quiz) => ({ id: quiz.id, box: await card(page, quiz.id).locator('[data-slot="window-chrome-body-surface"]').boundingBox() })));
  const { id: target, box } = fronts.reduce((tallest, candidate) => ((candidate.box?.height ?? 0) > (tallest.box?.height ?? 0) ? candidate : tallest));
  const front = box!;
  expect(front.height, "a card tall enough for the middle of a pet to pass in front of it").toBeGreaterThanOrEqual(FRONT_LEAST - OVERLAP_SLACK);
  await card(page, target).evaluate((node) => {
    const host = window as unknown as { petWalkClicked?: number };
    host.petWalkClicked = 0;
    node.addEventListener("click", () => (host.petWalkClicked = (host.petWalkClicked ?? 0) + 1));
  });
  let pet: Body | undefined;
  await expect(async () => {
    pet = await reachable(page, (body) => KINDS[body.id]?.gear.includes("parachute") === true).catch(() => reachable(page));
  }).toPass({ timeout: RECAST_MS });
  const fallen = pet!;
  await carry(page, fallen, { x: front.x + front.width / 2, y: front.y + RELEASE_DEPTH });
  await page.mouse.up();
  const passing = await page.evaluate(
    ({ id, area, card: cardId, budget, attribute }) =>
      new Promise<{ readonly x: number; readonly y: number; readonly footing: string } | null>((resolve) => {
        const tempo = Math.min(Math.max(Number(document.documentElement.getAttribute(attribute) ?? "1") || 1, 1 / 8), 8);
        let ran = 0;
        let last = performance.now();
        const look = (time: number): void => {
          ran += Math.min(Math.max(0, time - last) * tempo, 125);
          last = Math.max(last, time);
          const now = (window as unknown as { petWalk: Instruments }).petWalk.bodies().find((body) => body.id === id);
          if (now !== undefined && ["air", "chute"].includes(now.footing)) {
            const x = (now.left + now.right) / 2;
            const y = (now.top + now.bottom) / 2;
            const element = document.elementFromPoint(x, y);
            const inside = x > area.x + 4 && x < area.x + area.width - 4 && y > area.y + 4 && y < area.y + area.height - 4;
            if (inside && element?.closest(`[data-layered-card="${cardId}"]`) != null && element.closest("a[href], button, input, select, textarea, summary, label") === null) return resolve({ x, y, footing: now.footing });
          }
          if (ran >= budget) resolve(null);
          else window.requestAnimationFrame(look);
        };
        window.requestAnimationFrame(look);
      }),
    { id: fallen.id, area: front, card: target, budget: ANSWER_MS, attribute: TEMPO },
  );
  expect(passing, `${fallen.id} falls in front of the card of ${target}`).not.toBeNull();
  const pressed = passing!;
  await page.mouse.click(pressed.x, pressed.y);
  await expect(pane(page, target)).toHaveAttribute("data-opened", "");
  expect(await page.evaluate(() => (window as unknown as { petWalkClicked?: number }).petWalkClicked), `the card took the click at ${pressed.x}, ${pressed.y} in front of ${fallen.id} (${pressed.footing})`).toBe(1);
  const after = await trailOf(page, fallen.id);
  test.info().annotations.push({ type: "trail", description: `${fallen.id} in front of ${target} at ${pressed.x}, ${pressed.y} (${pressed.footing}): ${after.join(" → ")}` });
  expect(after.filter((line) => line.startsWith("hang/hand")), `${fallen.id} was picked up once, by the carry: ${after.join(" → ")}`).toHaveLength(1);
  await expect(page.locator("html")).not.toHaveAttribute(CURSOR, "grabbing");
});

test("the settings let the learner play with a pet on stage by keyboard: greetings, tricks, petting, tossing and explicit gestures", async ({ device }) => {
  const learner = await device("de");
  const page = learner.page;
  await enterWithPets(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await instrumented(page);
  const group = pane(page, "prefs").getByRole("group", { name: PLAY_LABEL[learner.locale], exact: true });
  await expect(group).toBeVisible();
  const status = group.getByRole("status");
  const tried = new Set<string>();
  await expect(async () => {
    const pet = await reachable(page, (body) => !tried.has(body.id));
    tried.add(pet.id);
    const name = KINDS[pet.id]!.name[learner.locale];
    const player = group.getByRole("group", { name, exact: true });
    const ask = async (deed: keyof typeof DEEDS, key: "Enter" | "Space"): Promise<void> => {
      const button = player.getByRole("button", { name: DEEDS[deed].button[learner.locale], exact: true });
      await button.focus();
      await expect(button).toBeFocused();
      await page.keyboard.press(key);
      await expect(button).toBeFocused();
      await expect(status).toHaveText(said(deed, name, learner.locale));
    };
    const since = Math.max((await trailOf(page, pet.id)).length - 1, 0);
    const answered = (wanted: readonly string[]): Promise<number> => trailOf(page, pet.id).then((trail) => inOrder(trail.slice(since), wanted));
    await ask("hello", "Enter");
    await eventually(page, () => answered(["greet"]), (seen) => seen === 1, { stageMs: ANSWER_MS, message: `${pet.id} says hello` });
    await ask("trick", "Space");
    await eventually(page, () => answered(["greet", "trick"]), (seen) => seen === 2, { stageMs: ANSWER_MS, message: `${pet.id} does a trick` });
    await ask("pet", "Enter");
    await eventually(page, () => answered(["greet", "trick", "purr"]), (seen) => seen === 3, { stageMs: ANSWER_MS, message: `${pet.id} purrs` });
    await ask("toss", "Space");
    await eventually(page, () => answered(["greet", "trick", "purr", "tumble/air"]), (seen) => seen === 4, { stageMs: ANSWER_MS, message: `${pet.id} is tossed up` });
    await expectPerched(page, pet.id, LANDING_MS);
    for (const deed of ["circle", "countercircle", "stroke", "shake"] as const) {
      await ask(deed, "Enter");
      await ask(deed, "Space");
    }
  }).toPass({ timeout: 2 * RECAST_MS });
});
