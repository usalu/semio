import sys, hashlib, json, os
B = sys.argv[1]
T = "🎓️teaching/🏛️architecture/📚️course/🧪️tests/🐕️pet-walk"
def cut(s, start, end, new, inclusive_end=False):
    a = s.index(start); b = s.index(end, a)
    if inclusive_end: b += len(end)
    return s[:a] + new + s[b:]

kit = open(f"{B}/base/kit.ts", encoding="utf8").read()
old_carry = kit[kit.index("export async function carry("):kit.index("/** ⏱️ How much of the pets' time one look of")]
new_carry = '''export async function carry(page: Page, body: Body, to: { readonly x: number; readonly y: number }): Promise<void> {
  let from = middleOf(body);
  await eventually(
    page,
    async () => {
      const now = (await bodies(page)).find((candidate) => candidate.id === body.id);
      if (now !== undefined) from = middleOf(now);
      await page.mouse.move(from.x, from.y);
      return page.locator("html").getAttribute(CURSOR);
    },
    (cursor) => cursor === "grab",
    { stageMs: ANSWER_MS, message: `${body.id} can be picked up where the hand is` },
  );
  await page.mouse.down();
  for (let step = 1; step <= 24; step++) {
    await page.mouse.move(from.x + ((to.x - from.x) * step) / 24, from.y + ((to.y - from.y) * step) / 24);
    await page.evaluate(framesPass, 1);
  }
  for (let step = 0; step < 10; step++) {
    await page.mouse.move(to.x, to.y);
    await page.evaluate(framesPass, 1);
  }
}

'''
assert "toHaveAttribute(CURSOR, \"grab\", { timeout: ANSWER_MS })" in old_carry
kit = kit.replace(old_carry, new_carry)
kit = kit.replace("/** 🤲️ Picks up the pet whose body is `body` at its middle", "/** 🤲️ Picks up the pet whose body is `body` at its middle (looked up again on every try, for at most {@link ANSWER_MS} of the pets' time: a pet that walked on while the harness was busy is followed)", 1)
open(f"{B}/files/kit.ts", "w", encoding="utf8").write(kit)

walk = open(f"{B}/base/walk.ts", encoding="utf8").read()
walk = walk.replace(" * leak of a pet's warmth, the speed of a circle) is the learner's.\n", " * leak of a pet's warmth, the speed of a circle) is the learner's. A spec that lets a pet fall watches the fall from before the hand lets go (the harness needs a\n * few frames for each call, on a loaded host a second, and a fall is over sooner) and tries again when another pet's head broke the fall: the casts and their places are random.\n", 1)
assert "A spec that lets a pet fall" in walk

old = walk[walk.index("  const held = pet!;\n  const canopy"):walk.index("  await expectPerched(page, held.id, LANDING_MS);\n  test.info().annotations.push({ type: \"trail\", description: `${held.id} let go at")]
new = '''  let held = pet!;
  const canopy = page.locator(`.pet-layer svg.pet[data-pet="${held.id}"] [data-pet-tool="chute"] > g > path`);
  await expect(canopy).toHaveCount(1);
  await expect(canopy).toHaveAttribute("d", KINDS[held.id]!.canopy!.d!);
  expect(await canopy.evaluate((path) => (path as SVGPathElement).getTotalLength())).toBeGreaterThan(0);
  const edges = (await page.evaluate(seenBoxes, EDGES)).filter((edge) => edge.topped);
  const top = 70;
  const width = page.viewportSize()?.width ?? 1440;
  let column = { x: 0, floor: 0 };
  let release = { x: 0, y: 0 };
  for (let attempt = 1; ; attempt++) {
    const reach = (held.right - held.left) / 2 + FALL_MARGIN;
    const others = (await bodies(page)).filter((body) => body.id !== held.id);
    const all = Array.from({ length: Math.floor((width - 160) / 8) }, (_, index) => 80 + index * 8).map((x) => ({ x, floor: Math.min(...edges.filter((edge) => edge.left <= x && edge.right >= x && edge.top > top + 40).map((edge) => edge.top)) }));
    const clear = all.filter((candidate) => others.every((body) => candidate.x < body.left - reach || candidate.x > body.right + reach));
    const pool = clear.some((candidate) => candidate.floor < Infinity) ? clear : all;
    column = pool.reduce((best, candidate) => (candidate.floor > best.floor && candidate.floor < Infinity ? candidate : best));
    release = { x: column.x, y: Math.max(top, column.floor - DROP_HEIGHT) };
    expect(column.floor - release.y, "a fall long enough for a parachute").toBeGreaterThan(200);

    const since = (await trailOf(page, held.id)).length;
    await carry(page, held, release);
    await expect(page.locator("html")).toHaveAttribute(CURSOR, "grabbing");
    await eventually(page, () => trailOf(page, held.id).then((trail) => trail.slice(since).some((line) => line.startsWith("hang/hand"))), (seen) => seen, { stageMs: ANSWER_MS, message: `${held.id} hangs in the hand` });
    await eventually(page, async () => {
      const body = (await bodies(page)).find((body) => body.id === held.id);
      return body !== undefined && body.footing === "hand" && Math.abs(middleOf(body).x - release.x) < 32 && body.bottom < release.y + 120;
    }, (reached) => reached, { stageMs: ANSWER_MS, message: `${held.id} reaches the high release column` });
    await page.mouse.up();
    const fall = await eventually(page, () => trailOf(page, held.id).then((trail) => trail.slice(since)), (trail) => trail.some((line) => line.startsWith("glide/chute") || line.startsWith("slide/head")), { stageMs: ANSWER_MS, message: `${held.id} opens its parachute` });
    if (fall.some((line) => line.startsWith("glide/chute"))) break;
    expect(attempt, `another pet's head broke the fall of ${held.id} every time: ${fall.join(" → ")}`).toBeLessThan(FALL_ATTEMPTS);
    await expectPerched(page, held.id, LANDING_MS);
    await expect(async () => {
      held = await reachable(page, (body) => body.id === held.id);
    }).toPass({ timeout: RECAST_MS });
  }
'''
walk = walk.replace(old, new)

old2 = walk[walk.index("  let pet: Body | undefined;\n  await expect(async () => {\n    pet = await reachable(page, (body) => KINDS[body.id]?.gear.includes(\"parachute\") === true).catch(() => reachable(page));"):walk.index("  const pressed = passing!;")]
new2 = '''  let passing: Falling | null = null;
  let fallen!: Body;
  let since = 0;
  for (let attempt = 1; passing === null; attempt++) {
    let pet: Body | undefined;
    await expect(async () => {
      pet = await reachable(page, (body) => KINDS[body.id]?.gear.includes("parachute") === true).catch(() => reachable(page));
    }).toPass({ timeout: RECAST_MS });
    fallen = pet!;
    since = (await trailOf(page, fallen.id)).length;
    await carry(page, fallen, { x: front.x + front.width / 2, y: front.y + RELEASE_DEPTH });
    await page.evaluate(watchFall, { id: fallen.id, area: front, card: target, budget: ANSWER_MS, attribute: TEMPO });
    await page.mouse.up();
    passing = await page.evaluate(() => (window as unknown as { petWalkFall: Promise<Falling | null> }).petWalkFall);
    if (passing === null) {
      expect(attempt, `${fallen.id} falls in front of the card of ${target} every time`).toBeLessThan(FALL_ATTEMPTS);
      await expectPerched(page, fallen.id, LANDING_MS);
    }
  }
'''
walk = walk.replace(old2, new2)
walk = walk.replace("  expect(after.filter((line) => line.startsWith(\"hang/hand\")), `${fallen.id} was picked up once, by the carry: ${after.join(\" → \")}`).toHaveLength(1);", "  expect(after.slice(since).filter((line) => line.startsWith(\"hang/hand\")), `${fallen.id} was picked up once, by the carry: ${after.slice(since).join(\" → \")}`).toHaveLength(1);")
assert "after.slice(since).filter" in walk

helpers = '''/** 🔂️ How often a spec that lets a pet fall tries again when another pet's head broke the fall or the pet was not seen in front of the card: the casts and their places are random, a host that is busy sees fewer frames of a fall. */
const FALL_ATTEMPTS = 5;

/** 📐️ The room a fall keeps clear of the other pets' sides, in pixels, beside half the width of the falling pet. */
const FALL_MARGIN = 24;

/** 🪂️ A pet seen in the air or under its parachute in front of a card. */
interface Falling {
  readonly x: number;
  readonly y: number;
  readonly footing: string;
}

/** 🪤️ In the page: arms a watch, before the hand lets go, for the pet `id` to pass in the air or under its parachute in front of the card `card` within `area`, away from every control, and leaves its answer (`null` when the pets' time `budget` ran out first) as `window.petWalkFall`. The pets' time counts only while the pet is no longer in the hand, and one frame carries at most 125 ms of it, as the pacer does. */
function watchFall({ id, area, card: cardId, budget, attribute }: { readonly id: string; readonly area: { readonly x: number; readonly y: number; readonly width: number; readonly height: number }; readonly card: string; readonly budget: number; readonly attribute: string }): void {
  (window as unknown as { petWalkFall: Promise<Falling | null> }).petWalkFall = new Promise<Falling | null>((resolve) => {
    const tempo = Math.min(Math.max(Number(document.documentElement.getAttribute(attribute) ?? "1") || 1, 1 / 8), 8);
    let ran = 0;
    let last = performance.now();
    const look = (time: number): void => {
      const step = Math.min(Math.max(0, time - last) * tempo, 125);
      last = Math.max(last, time);
      const now = (window as unknown as { petWalk: Instruments }).petWalk.bodies().find((body) => body.id === id);
      if (now !== undefined && now.footing !== "hand") ran += step;
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
  });
}

'''
marker = 'test("pets live on the home screen as decoration'
walk = walk.replace(marker, helpers + marker, 1)
# imports: Body already imported, expectPerched, trailOf, bodies, reachable present
open(f"{B}/files/walk.ts", "w", encoding="utf8").write(walk)
h = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
rows = {f"{T}/🟦️.ts": {"file": "walk.ts", "base": h(f"{B}/base/walk.ts"), "new": h(f"{B}/files/walk.ts")},
        f"{T}/🧰️pet-kit/🟦️.ts": {"file": "kit.ts", "base": h(f"{B}/base/kit.ts"), "new": h(f"{B}/files/kit.ts")}}
json.dump(rows, open(f"{B}/manifest.json", "w", encoding="utf8"), indent=1, ensure_ascii=False)
