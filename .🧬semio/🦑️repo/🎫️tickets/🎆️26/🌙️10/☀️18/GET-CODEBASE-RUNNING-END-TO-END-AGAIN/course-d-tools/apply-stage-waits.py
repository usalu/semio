import re, sys
D = "🎓️teaching/🏛️architecture/📚️course/🧪️tests/🐕️pet-walk/"
kit, main, ladder = D + "🧰️pet-kit/🟦️.ts", D + "🟦️.ts", D + "🪜️mischief-ladder-and-phone/🟦️.ts"
def rd(p): return open(p, encoding="utf-8").read()
def wr(p, s): open(p, "w", encoding="utf-8").write(s)
def sub(s, old, new, count=1):
    assert s.count(old) >= 1, old[:80]
    return s.replace(old, new, count)

k = rd(kit)
if "export function stagePasses" not in k:
    old = k[k.index("/** 🛬️ Waits until the pet `id` stands on a perch again"):k.index("export function castOf")]
    new = '''/** ⏱️ How much of the pets' time one look of {@link eventually} lets pass, in milliseconds. */
export const STAGE_LOOK_MS = 250;

/** ⏲️ In the page: waits until the pets' own clock has run `milliseconds` and answers how long it ran. The clock of the pet layer hands out at most eight ticks (125 ms) per animation frame and drops the rest, so on a loaded host the pets' time falls behind the wall clock; summing the frames the way the pacer does measures the time the pets lived, whatever the host did. */
export function stagePasses(wait: { readonly milliseconds: number; readonly attribute: string }): Promise<number> {
  return new Promise((resolve) => {
    const tempo = Math.min(Math.max(Number(document.documentElement.getAttribute(wait.attribute) ?? "1") || 1, 1 / 8), 8);
    let ran = 0;
    let last = performance.now();
    const next = (time: number): void => {
      ran += Math.min(Math.max(0, time - last) * tempo, 125);
      last = Math.max(last, time);
      if (ran >= wait.milliseconds) resolve(ran);
      else window.requestAnimationFrame(next);
    };
    window.requestAnimationFrame(next);
  });
}

/** 🔭️ Looks until `done` accepts what `probe` sees, for at most `stageMs` milliseconds of the pets' time ({@link stagePasses}), not of the wall clock: the budget is what an idle host spends in the same wall time, and a loaded host does not shorten it. A wait for something a pet does is declared this way; nothing sleeps. */
export async function eventually<T>(page: Page, probe: () => Promise<T>, done: (seen: T) => boolean, wait: { readonly stageMs: number; readonly message: string }): Promise<T> {
  let seen = await probe();
  for (let ran = 0; !done(seen); ) {
    expect(ran, `${wait.message} (last seen: ${JSON.stringify(seen)})`).toBeLessThan(wait.stageMs);
    ran += await page.evaluate(stagePasses, { milliseconds: STAGE_LOOK_MS, attribute: TEMPO });
    seen = await probe();
  }
  return seen;
}

/** 🛬️ Waits, for `stageMs` milliseconds of the pets' time, until the pet `id` stands on a perch again, wholly visible. */
export async function expectPerched(page: Page, id: string, stageMs: number): Promise<void> {
  await eventually(page, async () => (await bodies(page)).some((body) => body.id === id && body.footing === "perch" && body.opacity > 0.99 && !["hang", "tumble", "fall", "glide"].includes(body.activity)), (perched) => perched, { stageMs, message: `${id} stands on a perch again` });
}

'''
    k = k.replace(old, new)
    wr(kit, k)

m = rd(main)
pat = re.compile(r'^(\s*)await expect\.poll\((.+), \{ message: (.+), timeout: ANSWER_MS \}\)\.toBe\((.+)\);$', re.M)
m = pat.sub(lambda g: f'{g.group(1)}await eventually(page, {g.group(2)}, (seen) => seen === {g.group(4)}, {{ stageMs: ANSWER_MS, message: {g.group(3)} }});', m)
old = m[m.index("  await expect.poll(async () => {\n    const body = (await bodies(page)).find((body) => body.id === held.id);"):m.index("  await page.mouse.up();\n  await eventually(page, () => trailOf(page, held.id)")]
new = '''  await eventually(page, async () => {
    const body = (await bodies(page)).find((body) => body.id === held.id);
    return body !== undefined && body.footing === "hand" && Math.abs(middleOf(body).x - release.x) < 32 && body.bottom < release.y + 120;
  }, (reached) => reached, { stageMs: ANSWER_MS, message: `${held.id} reaches the high release column` });
'''
m = m.replace(old, new)
m = sub(m, "import { ANSWER_MS, APPEAR_MS, bodies,", "import { ANSWER_MS, APPEAR_MS, bodies, eventually,")
wr(main, m)

l = rd(ladder)
l = sub(l, "import { ANSWER_MS, APPEAR_MS, bodies,", "import { ANSWER_MS, APPEAR_MS, bodies, eventually,")
l = sub(l, 'await expect.poll(() => carried("ladder"), { message: `a pet on a ladder on the page of ${gutters!.id}`, timeout: WALK_MS }).not.toEqual([]);',
        'await eventually(page, () => carried("ladder"), (seen) => seen.length > 0, { stageMs: WALK_MS * HASTE, message: `a pet on a ladder on the page of ${gutters!.id}` });')
l = sub(l, "test.setTimeout(WALK_MS + ROPE_MS + 4 * RECAST_MS);", "test.setTimeout(30 * 60_000);")
l = sub(l, 'await expect(async () => {\n    if ((await carried("rope")).length > 0) return;', 'const ropeRound = async (): Promise<void> => {\n    if ((await carried("rope")).length > 0) return;')
l = sub(l, "    const until = Date.now() + ROPE_ROUND_MS;\n", "    let round = 0;\n")
l = sub(l, 'while (Date.now() < until && (await carried("rope")).length === 0) {', 'while (round < ROPE_ROUND_MS * HASTE && (await carried("rope")).length === 0) {')
l = sub(l, "        await page.evaluate(framesPass, 30);\n      }\n    } finally {", "        round += await page.evaluate(stagePasses, { milliseconds: 3_750, attribute: TEMPO });\n      }\n    } finally {")
l = sub(l, "  }).toPass({ timeout: ROPE_MS, intervals: [0] });", "  };\n  for (let attempt = 1; ; attempt++) {\n    try {\n      await ropeRound();\n      break;\n    } catch (failure) {\n      if (attempt >= ROPE_MS / ROPE_ROUND_MS) throw failure;\n    }\n  }")
l = sub(l, "reachable, RECAST_MS, ROPE_MS, ROPE_REACH, ROPE_ROUND_MS, standing, TEMPO,", "reachable, RECAST_MS, ROPE_MS, ROPE_REACH, ROPE_ROUND_MS, stagePasses, standing, TEMPO,")
wr(ladder, l)
print("applied")
